use crate::{
    backend::{Backend, BackendConfig},
    config::{Config, ConfigError},
    input::{KeyBindingParseError, KeyMap},
    render::Color,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeMode {
    Compositor,
    Simulation,
    BackendTest,
}

impl RuntimeMode {
    pub fn from_args(args: impl IntoIterator<Item = String>) -> Result<Self, AppError> {
        let mut mode = Self::Compositor;

        for arg in args {
            match arg.as_str() {
                "--simulate" | "-s" => mode = Self::Simulation,
                "--backend-test" => mode = Self::BackendTest,
                unknown => return Err(AppError::UnknownArgument(unknown.to_string())),
            }
        }

        Ok(mode)
    }
}

pub struct App {
    config: Config,
    keymap: KeyMap,
    mode: RuntimeMode,
}

impl App {
    pub fn new(mode: RuntimeMode) -> Result<Self, AppError> {
        let config = Config::load()?;
        let keymap = KeyMap::from_config(&config)?;

        Ok(Self {
            config,
            keymap,
            mode,
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match self.mode {
            RuntimeMode::Compositor => {
                let _keymap = &self.keymap;
                println!(
                    "AtomicWM starting minimal renderer. Client windows are not supported yet."
                );
                let background = Color::from_hex_rgb(&self.config.appearance.background)?;
                let mut backend = Backend::new(BackendConfig::compositor(background))?;
                backend.run()?;
            }
            RuntimeMode::Simulation => crate::sim::run_simulation(&self.config)?,
            RuntimeMode::BackendTest => {
                println!("AtomicWM starting minimal renderer backend test.");
                let background = Color::from_hex_rgb(&self.config.appearance.background)?;
                let mut backend = Backend::new(BackendConfig::backend_test(background))?;
                backend.run()?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppError {
    UnknownArgument(String),
    Config(String),
    KeyBinding(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownArgument(argument) => write!(formatter, "unknown argument: {argument}"),
            Self::Config(message) => write!(formatter, "{message}"),
            Self::KeyBinding(message) => write!(formatter, "invalid keybinding: {message}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<ConfigError> for AppError {
    fn from(error: ConfigError) -> Self {
        Self::Config(error.to_string())
    }
}

impl From<KeyBindingParseError> for AppError {
    fn from(error: KeyBindingParseError) -> Self {
        Self::KeyBinding(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{App, AppError, RuntimeMode};

    #[test]
    fn argument_parsing_recognizes_compositor_mode() {
        assert_eq!(RuntimeMode::from_args([]).unwrap(), RuntimeMode::Compositor);
    }

    #[test]
    fn argument_parsing_recognizes_simulation_mode() {
        assert_eq!(
            RuntimeMode::from_args(["--simulate".to_string()]).unwrap(),
            RuntimeMode::Simulation
        );
        assert_eq!(
            RuntimeMode::from_args(["-s".to_string()]).unwrap(),
            RuntimeMode::Simulation
        );
    }

    #[test]
    fn argument_parsing_recognizes_backend_test_mode() {
        assert_eq!(
            RuntimeMode::from_args(["--backend-test".to_string()]).unwrap(),
            RuntimeMode::BackendTest
        );
    }

    #[test]
    fn unknown_argument_returns_error() {
        assert_eq!(
            RuntimeMode::from_args(["--bad".to_string()]).unwrap_err(),
            AppError::UnknownArgument("--bad".to_string())
        );
    }

    #[test]
    fn app_can_construct_with_default_config() {
        let app = App::new(RuntimeMode::Simulation).unwrap();

        assert_eq!(app.mode, RuntimeMode::Simulation);
    }

    #[test]
    fn simulation_mode_still_runs() {
        let mut app = App::new(RuntimeMode::Simulation).unwrap();

        app.run().unwrap();
    }
}
