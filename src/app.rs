use crate::config::{Config, ConfigError};

pub struct App {
    config: Config,
}

impl App {
    pub fn new() -> Result<Self, ConfigError> {
        Ok(Self {
            config: Config::load()?,
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let _config = &self.config;

        println!("AtomicWM skeleton initialized. Config loaded.");
        Ok(())
    }
}
