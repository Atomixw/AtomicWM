pub mod app;
pub mod backend;
pub mod canvas;
pub mod config;
pub mod geometry;
pub mod input;
pub mod logging;
pub mod render;
pub mod window;

use app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init();

    let mut app = App::new()?;
    app.run()
}
