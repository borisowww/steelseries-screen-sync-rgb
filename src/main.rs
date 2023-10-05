extern crate gamesense;
extern crate anyhow;

use std::vec;
use anyhow::{Result};
use gamesense::client::GameSenseClient;
use gamesense::handler::screen::{BitmapHandler};

use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;

use image::{Rgba, ImageBuffer, RgbaImage};

const NEW_W: u32 = 22;
const NEW_H: u32 = 6;


use std::thread;
use std::time::Duration;
use image::imageops::{FilterType, resize};
use num::clamp;
use show_image::{create_window, ImageInfo, ImageView};

#[show_image::main]
fn main() -> Result<()> {
    let mut client = GameSenseClient::new("RGB_SYNC", "SteelSeries RGB sync", "Author", None)?;

    let handlers: Vec<BitmapHandler> = vec![BitmapHandler::new("rgb-per-key-zones", "rgb-103-zone")];

    client.bind_rgb_event("HEHEHE", handlers).expect("TODO: panic message");


    // draw_gradient(&client)?;
    draw_screen(&client)?;
    Ok(())
}


fn draw_screen(client: &GameSenseClient) -> Result<()> {
    let d = Display::primary().unwrap();
    let (w, h) = (d.width(), d.height());
    let window = create_window("image", Default::default())?;

    let mut capturer = Capturer::new(d).unwrap();
    loop {
        match capturer.frame() {
            Ok(frame) => {
                    let frame_vec = frame.to_vec();
                    let image: RgbaImage = ImageBuffer::from_vec(w as u32, h as u32, frame_vec).expect("Buffer problem");
                    let scaled = resize(&image, NEW_W, NEW_H, FilterType::Gaussian);

                    let sync = bgra8_to_rgb8(scaled);
                    let response =  client.trigger_rgb_event("HEHEHE", sync.clone())?;
                    // Convert sync from Vec<Vec<u8>> to &[u8]
                    let sync_slice = sync.as_slice().concat().as_slice().to_vec();


                    let image = ImageView::new(ImageInfo::rgb8(22, 8), &sync_slice);
                    window.set_image("image-001", image)?;


            }
            Err(ref e) if e.kind() == WouldBlock => {
                // dbg!(e);
            }
            Err(err) => {
                // We're done here.
                dbg!(err);
                break;
            }
        }
    }
    Ok(())
}

// Infite loop that draws the gradient
fn draw_gradient(client: &GameSenseClient) -> Result<()> {
    for i in (1..250).rev() {
        let  data = build_gradient(i);
        // let  data = get_data();
        let response =  client.trigger_rgb_event("HEHEHE", data)?;

        thread::sleep(Duration::from_secs_f32(0.1));
    }
    Ok(())
}

fn build_gradient(timestamp: u8) -> Vec<Vec<u8>> {
    // const NUMBER_OF_KEYS: u8 = 132;
    const NUMBER_OF_COLS: u8 = 22;
    const NUMBER_OF_ROWS: u8 = 6;

    // Some different color mappings to test the rgb input.

    // let rainbow_mapping: Vec<Vec<u8>> = vec![vec![148, 0, 211], vec![0,0,255], vec![0, 255, 0], vec![255, 255, 0], vec![255, 127, 0], vec![255, 0, 255]];
    // let rainbow_mapping: Vec<Vec<u8>> = vec![vec![148, 0, 211], vec![0,0,255], vec![0, 255, 0], vec![255, 255, 0], vec![255, 127, 0], vec![255, 0, 0]];
    let rainbow_mapping: Vec<Vec<u8>> = vec![vec![0, 255, 0], vec![0, 255, 0], vec![255, 0, 0], vec![255, 0, 0], vec![255, 255, 255], vec![255,255,255]];

    let mut color_vector: Vec<Vec<u8>> = vec![];

    // for i in 1..=number_of_keys  {
    //     color_vector.push(vec![255, 255, 255]);
    // }

    let start = timestamp - 1;
    let end = start + 5;

    for i in start..= end {
        for j in 1..=NUMBER_OF_COLS {
            let index = i%6;
            let val = &rainbow_mapping[index as usize];
            color_vector.push(val.to_owned());
        }
    }

    println!("{}", color_vector.len() as usize);
    return color_vector;
}

/**
* Convert a BGRA8 image to RGB8 and (hopefully brighten the pixels?)
*/
fn bgra8_to_rgb8(input: image::ImageBuffer<Rgba<u8>, Vec<u8>>) -> Vec<Vec<u8>> {
    let width = input.width() as usize;
    let height = input.height() as usize;

    // Get the raw image data as a vector
    let input: &Vec<u8> = input.as_raw();

    // Allocate a new buffer for the RGB image, 3 bytes per pixel
    let mut output_data: Vec<Vec<u8>> = vec![];

    // Iterate through 4-byte chunks of the image data (RGBA bytes)
    for chunk in input.chunks(4) {
        // ... and copy each of them to output, leaving out the A byte and increase the brightness of each pixel by 25 percent capping at 255.

        // This multiplication clamp probably doesn't work correctly.git remote add origin git@github.com:borisowww/steelseries-screen-sync-rgb.git
        let mut r = clamp(chunk[2] as f32 * 1.5, 0f32, 255f32);
        let mut g = clamp(chunk[1] as f32 * 1.5, 0f32, 255f32);
        let mut b = clamp(chunk[0] as f32 * 1.5, 0f32, 255f32);

        output_data.push(vec![r as u8, g as u8, b as u8]);

    }

    // Construct a new image
    return output_data;
}

/**
* Draw a gradient on the keyboard
*/
fn get_data() -> Vec<Vec<u8>> {
    let a = vec![255, 0, 0];
    return vec![
        vec![255,0,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,255,0],
        vec![255,0,0],
        vec![255,0,0]
    ]
}