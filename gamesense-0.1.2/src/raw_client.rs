use std::env;
use std::fs;
use crate::handler;
use anyhow::{Result, anyhow, bail};
use reqwest;
use serde_json;
use serde::Serialize;
use serde_json::json;

macro_rules! cond_argument {
    ($data:expr, $key:literal, $option_value:ident) => {
        if let Some(value) = $option_value {
            $data.as_object_mut().unwrap().insert(String::from($key), json!(value));
        }
    };
}

pub struct RawGameSenseClient {
    client: reqwest::blocking::Client,
    address: String
}

impl RawGameSenseClient {
    pub fn new() -> Result<RawGameSenseClient> {
        let path = match env::consts::OS {
            "macos" => Ok(String::from("/Library/Application Support/SteelSeries Engine 3/coreProps.json")),
            "windows" => Ok(env::var("PROGRAMDATA")? + "/SteelSeries/SteelSeries Engine 3/coreProps.json"),
            _ => Err(anyhow!("Discovery failed: Platform must be either MacOS or Windows - Got {}", env::consts::OS))
        };

        let config: serde_json::Value = serde_json::from_str(&fs::read_to_string(path?)?)?;

        return Ok(RawGameSenseClient {
            client: reqwest::blocking::Client::new(),
            address: config["address"].as_str().expect("Discovery failed: `address` not found").to_owned()
        });
    }

    pub fn send_data(&self, endpoint: &str, data: &serde_json::Value) -> Result<String> {


        let data = self.client.post(format!("http://{}/{}", self.address, endpoint))
            .json(data)
            .send()?
            .text()?;

        // dbg!(&data);
        if data == "Page not found" {
            bail!("Endpoint not found");
        }

        let data: serde_json::Map<String, serde_json::Value> = serde_json::from_str(&data)?;

        let (key, value) = data.iter().next().unwrap();
        let value = value.as_str().unwrap_or(&value.to_string()).to_owned();
        match key.as_str() {
            "error" => bail!(value),
            _ => Ok(value)
        }
    }

    pub fn game_event(&self, game: &str, event: &str, value: isize, frame: Option<serde_json::Value>) -> Result<String> {
        let mut data = json!({
            "game": game,
            "event": event,
            "data": {
                "value": value
            }
        });

        cond_argument!(data.get_mut("data").unwrap(), "frame", frame);

        self.send_data("game_event", &data)
    }


    pub fn bitmap_event(&self, game: &str, event: &str, frame: Vec<Vec<u8>>) -> Result<String> {
        let mut data = json!({
            "game": game,
            "event": event,
            "data": {
                "frame": {
                    "bitmap": frame
                }
            }
        });
        // dbg!(&data);
        self.send_data("game_event", &data)
    }

    pub fn heartbeat(&self, game: &str) -> Result<String> {
        let data = json!({
            "game": game
        });

        self.send_data("game_heartbeat", &data)
    }

    pub fn register_game(&self, game: &str, game_display_name: Option<&str>, developer: Option<&str>, deinitialize_timer_length_ms: Option<u16>) -> Result<String> {
        let mut data = json!({
            "game": game
        });

        cond_argument!(data, "game_display_name", game_display_name);
        cond_argument!(data, "developer", developer);
        cond_argument!(data, "deinitialize_timer_length_ms", deinitialize_timer_length_ms);

        self.send_data("game_metadata", &data)
    }

    pub fn remove_game(&self, game: &str) -> Result<String> {
        let data = json!({
            "game": game,
        });

        self.send_data("remove_game", &data)
    }

    pub fn bind_event<T: Serialize + handler::Handler>(&self, game: &str, event: &str, min_value: Option<isize>, max_value: Option<isize>, icon_id: Option<u8>, value_optional: Option<bool>, handlers: Vec<T>) -> Result<String> {
        let mut data = json!({
            "game": game,
            "event": event,
            "handlers": handlers
        });

        cond_argument!(data, "min_value", min_value);
        cond_argument!(data, "max_value", max_value);
        cond_argument!(data, "icon_id", icon_id);
        cond_argument!(data, "value_optional", value_optional);

        self.send_data("bind_game_event", &data)
    }

    pub fn bind_bitmap_event<T: Serialize + handler::Handler>(&self, game: &str, event: &str, handlers: Vec<T>) -> Result<String> {
        let mut data = json!({
            "game": game,
            "event": event,
            "handlers": handlers
        });


        self.send_data("bind_game_event", &data)
    }

    pub fn register_event(&self, game: &str, event: &str, min_value: Option<isize>, max_value: Option<isize>, icon_id: Option<u8>, value_optional: Option<bool>) -> Result<String> {
        let mut data = json!({
            "game": game,
            "event": event
        });

        cond_argument!(data, "min_value", min_value);
        cond_argument!(data, "max_value", max_value);
        cond_argument!(data, "icon_id", icon_id);
        cond_argument!(data, "value_optional", value_optional);

        self.send_data("register_game_event", &data)
    }

    pub fn remove_event(&self, game: &str, event: &str) -> Result<String> {
        let data = json!({
            "game": game,
            "event": event
        });

        self.send_data("remove_game_event", &data)
    }
}
