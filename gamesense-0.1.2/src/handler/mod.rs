pub mod screen;

pub trait Handler {}

impl Handler for screen::ScreenHandler {}
impl Handler for screen::BitmapHandler {}
