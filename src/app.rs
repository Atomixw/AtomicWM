use crate::{
    config::{Config, ConfigError},
    input::{KeyBindingParseError, KeyMap},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Normal,
    Simulation,
}

impl AppMode {
    pub fn from_args(args: impl IntoIterator<Item = String>) -> Result<Self, AppError> {
        let mut mode = Self::Normal;

        for arg in args {
            match arg.as_str() {
                "--simulate" | "-s" => mode = Self::Simulation,
                unknown => return Err(AppError::UnknownArgument(unknown.to_string())),
            }
        }

        Ok(mode)
    }
}

pub struct App {
    config: Config,
    keymap: KeyMap,
    mode: AppMode,
}

impl App {
    pub fn new(mode: AppMode) -> Result<Self, AppError> {
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
            AppMode::Normal => {
                let _keymap = &self.keymap;
                println!("AtomicWM skeleton initialized. Config loaded.");
            }
            AppMode::Simulation => crate::sim::run_simulation(&self.config)?,
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
    use super::{AppError, AppMode};

    #[test]
    fn argument_parsing_recognizes_normal_mode() {
        assert_eq!(AppMode::from_args([]).unwrap(), AppMode::Normal);
    }

    #[test]
    fn argument_parsing_recognizes_simulation_mode() {
        assert_eq!(
            AppMode::from_args(["--simulate".to_string()]).unwrap(),
            AppMode::Simulation
        );
        assert_eq!(
            AppMode::from_args(["-s".to_string()]).unwrap(),
            AppMode::Simulation
        );
    }

    #[test]
    fn unknown_argument_returns_error() {
        assert_eq!(
            AppMode::from_args(["--bad".to_string()]).unwrap_err(),
            AppError::UnknownArgument("--bad".to_string())
        );
    }
}
