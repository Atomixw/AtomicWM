pub mod app;
pub mod backend;
pub mod canvas;
pub mod config;
pub mod geometry;
pub mod input;
pub mod logging;
pub mod render;
pub mod sim;
pub mod window;

use app::{App, RuntimeMode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init();

    let mode = RuntimeMode::from_args(std::env::args().skip(1))?;
    let mut app = App::new(mode)?;
    app.run()
}
