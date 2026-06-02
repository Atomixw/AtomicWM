use std::{
    error::Error,
    fmt, fs, io,
    path::{Path, PathBuf},
};

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub general: GeneralConfig,
    pub camera: CameraConfig,
    pub appearance: AppearanceConfig,
    pub snapping: SnappingConfig,
    pub keybindings: KeybindingsConfig,
    pub commands: CommandsConfig,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let Some(path) = Self::config_path() else {
            return Ok(Self::default());
        };

        Self::load_from_path(path)
    }

    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let contents = match fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Self::default()),
            Err(error) => {
                return Err(ConfigError::Read {
                    path: path.to_path_buf(),
                    source: error,
                });
            }
        };

        let config: Self = toml::from_str(&contents).map_err(|source| ConfigError::Parse {
            path: path.to_path_buf(),
            source,
        })?;

        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        validate_positive("camera.pan_step", self.camera.pan_step)?;
        validate_greater_than("camera.zoom_step", self.camera.zoom_step, 1.0)?;
        validate_positive("camera.min_zoom", self.camera.min_zoom)?;

        if !self.camera.max_zoom.is_finite() || self.camera.max_zoom <= self.camera.min_zoom {
            return Err(ConfigError::Validation(
                "camera.max_zoom must be greater than camera.min_zoom".to_string(),
            ));
        }

        validate_non_negative("appearance.border_width", self.appearance.border_width)?;
        validate_non_negative("appearance.gap", self.appearance.gap)?;
        validate_color("appearance.background", &self.appearance.background)?;
        validate_color("appearance.focused_border", &self.appearance.focused_border)?;
        validate_color("appearance.normal_border", &self.appearance.normal_border)?;
        validate_non_negative("snapping.threshold", self.snapping.threshold)?;
        validate_non_negative("snapping.gap", self.snapping.gap)?;
        validate_not_empty("commands.terminal", &self.commands.terminal)?;

        for (name, value) in self.keybindings.entries() {
            validate_not_empty(name, value)?;
        }

        Ok(())
    }

    pub fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|path| path.join("atomicwm").join("config.toml"))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            camera: CameraConfig::default(),
            appearance: AppearanceConfig::default(),
            snapping: SnappingConfig::default(),
            keybindings: KeybindingsConfig::default(),
            commands: CommandsConfig::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct GeneralConfig {
    pub mod_key: String,
    pub focus_follows_mouse: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            mod_key: "Super".to_string(),
            focus_follows_mouse: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct CameraConfig {
    pub pan_step: f64,
    pub zoom_step: f64,
    pub min_zoom: f64,
    pub max_zoom: f64,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            pan_step: 80.0,
            zoom_step: 1.1,
            min_zoom: 0.1,
            max_zoom: 4.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct AppearanceConfig {
    pub border_width: f64,
    pub gap: f64,
    pub background: String,
    pub focused_border: String,
    pub normal_border: String,
}

impl Default for AppearanceConfig {
    fn default() -> Self {
        Self {
            border_width: 2.0,
            gap: 8.0,
            background: "#111111".to_string(),
            focused_border: "#7C3AED".to_string(),
            normal_border: "#333333".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct SnappingConfig {
    pub enabled: bool,
    pub threshold: f64,
    pub gap: f64,
}

impl Default for SnappingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            threshold: 24.0,
            gap: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct KeybindingsConfig {
    pub quit: String,
    pub spawn_terminal: String,
    pub zoom_in: String,
    pub zoom_out: String,
    pub reset_zoom: String,
    pub pan_left: String,
    pub pan_right: String,
    pub pan_up: String,
    pub pan_down: String,
    pub focus_left: String,
    pub focus_right: String,
    pub focus_up: String,
    pub focus_down: String,
    pub center_focused: String,
    pub fit_all: String,
}

impl KeybindingsConfig {
    fn entries(&self) -> [(&'static str, &str); 15] {
        [
            ("keybindings.quit", &self.quit),
            ("keybindings.spawn_terminal", &self.spawn_terminal),
            ("keybindings.zoom_in", &self.zoom_in),
            ("keybindings.zoom_out", &self.zoom_out),
            ("keybindings.reset_zoom", &self.reset_zoom),
            ("keybindings.pan_left", &self.pan_left),
            ("keybindings.pan_right", &self.pan_right),
            ("keybindings.pan_up", &self.pan_up),
            ("keybindings.pan_down", &self.pan_down),
            ("keybindings.focus_left", &self.focus_left),
            ("keybindings.focus_right", &self.focus_right),
            ("keybindings.focus_up", &self.focus_up),
            ("keybindings.focus_down", &self.focus_down),
            ("keybindings.center_focused", &self.center_focused),
            ("keybindings.fit_all", &self.fit_all),
        ]
    }
}

impl Default for KeybindingsConfig {
    fn default() -> Self {
        Self {
            quit: "Super+Shift+Q".to_string(),
            spawn_terminal: "Super+Enter".to_string(),
            zoom_in: "Super+Equal".to_string(),
            zoom_out: "Super+Minus".to_string(),
            reset_zoom: "Super+0".to_string(),
            pan_left: "Super+Ctrl+Left".to_string(),
            pan_right: "Super+Ctrl+Right".to_string(),
            pan_up: "Super+Ctrl+Up".to_string(),
            pan_down: "Super+Ctrl+Down".to_string(),
            focus_left: "Super+Left".to_string(),
            focus_right: "Super+Right".to_string(),
            focus_up: "Super+Up".to_string(),
            focus_down: "Super+Down".to_string(),
            center_focused: "Super+C".to_string(),
            fit_all: "Super+W".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct CommandsConfig {
    pub terminal: String,
}

impl Default for CommandsConfig {
    fn default() -> Self {
        Self {
            terminal: "alacritty".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Read {
        path: PathBuf,
        source: io::Error,
    },
    Parse {
        path: PathBuf,
        source: toml::de::Error,
    },
    Validation(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, source } => {
                write!(
                    formatter,
                    "failed to read config {}: {source}",
                    path.display()
                )
            }
            Self::Parse { path, source } => {
                write!(
                    formatter,
                    "failed to parse config {}: {source}",
                    path.display()
                )
            }
            Self::Validation(message) => write!(formatter, "invalid config: {message}"),
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Read { source, .. } => Some(source),
            Self::Parse { source, .. } => Some(source),
            Self::Validation(_) => None,
        }
    }
}

fn validate_positive(name: &str, value: f64) -> Result<(), ConfigError> {
    if value.is_finite() && value > 0.0 {
        Ok(())
    } else {
        Err(ConfigError::Validation(format!(
            "{name} must be greater than 0"
        )))
    }
}

fn validate_greater_than(name: &str, value: f64, minimum: f64) -> Result<(), ConfigError> {
    if value.is_finite() && value > minimum {
        Ok(())
    } else {
        Err(ConfigError::Validation(format!(
            "{name} must be greater than {minimum}"
        )))
    }
}

fn validate_non_negative(name: &str, value: f64) -> Result<(), ConfigError> {
    if value.is_finite() && value >= 0.0 {
        Ok(())
    } else {
        Err(ConfigError::Validation(format!(
            "{name} must be greater than or equal to 0"
        )))
    }
}

fn validate_not_empty(name: &str, value: &str) -> Result<(), ConfigError> {
    if value.trim().is_empty() {
        Err(ConfigError::Validation(format!("{name} must not be empty")))
    } else {
        Ok(())
    }
}

fn validate_color(name: &str, value: &str) -> Result<(), ConfigError> {
    let valid = value.len() == 7
        && value.starts_with('#')
        && value[1..]
            .chars()
            .all(|character| character.is_ascii_hexdigit());

    if valid {
        Ok(())
    } else {
        Err(ConfigError::Validation(format!(
            "{name} must use #RRGGBB format"
        )))
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::{Config, ConfigError};

    #[test]
    fn default_config_is_valid() {
        assert!(Config::default().validate().is_ok());
    }

    #[test]
    fn partial_toml_uses_defaults() {
        let path = temp_file_path("partial.toml");
        fs::write(
            &path,
            r#"
                [camera]
                pan_step = 120.0

                [commands]
                terminal = "foot"
            "#,
        )
        .unwrap();

        let config = Config::load_from_path(&path).unwrap();

        assert_eq!(config.camera.pan_step, 120.0);
        assert_eq!(config.camera.zoom_step, 1.1);
        assert_eq!(config.commands.terminal, "foot");
        assert_eq!(config.general.mod_key, "Super");
    }

    #[test]
    fn invalid_toml_returns_error() {
        let path = temp_file_path("invalid.toml");
        fs::write(&path, "[camera").unwrap();

        let error = Config::load_from_path(&path).unwrap_err();

        assert!(matches!(error, ConfigError::Parse { .. }));
    }

    #[test]
    fn missing_file_returns_default_config() {
        let path = temp_file_path("missing.toml");

        let config = Config::load_from_path(path).unwrap();

        assert_eq!(config, Config::default());
    }

    #[test]
    fn invalid_zoom_values_fail_validation() {
        let mut config = Config::default();
        config.camera.zoom_step = 1.0;
        assert!(matches!(config.validate(), Err(ConfigError::Validation(_))));

        let mut config = Config::default();
        config.camera.min_zoom = 0.0;
        assert!(matches!(config.validate(), Err(ConfigError::Validation(_))));

        let mut config = Config::default();
        config.camera.max_zoom = config.camera.min_zoom;
        assert!(matches!(config.validate(), Err(ConfigError::Validation(_))));
    }

    #[test]
    fn invalid_colors_fail_validation() {
        let mut config = Config::default();
        config.appearance.background = "111111".to_string();

        assert!(matches!(config.validate(), Err(ConfigError::Validation(_))));
    }

    #[test]
    fn default_snapping_config_is_valid() {
        let config = Config::default();

        assert!(config.snapping.enabled);
        assert_eq!(config.snapping.threshold, 24.0);
        assert_eq!(config.snapping.gap, 0.0);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn negative_snapping_threshold_fails_validation() {
        let mut config = Config::default();
        config.snapping.threshold = -1.0;

        assert!(matches!(config.validate(), Err(ConfigError::Validation(_))));
    }

    #[test]
    fn negative_snapping_gap_fails_validation() {
        let mut config = Config::default();
        config.snapping.gap = -1.0;

        assert!(matches!(config.validate(), Err(ConfigError::Validation(_))));
    }

    #[test]
    fn empty_terminal_command_fails_validation() {
        let mut config = Config::default();
        config.commands.terminal = " ".to_string();

        assert!(matches!(config.validate(), Err(ConfigError::Validation(_))));
    }

    fn temp_file_path(file_name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let directory = std::env::temp_dir().join(format!("atomicwm-config-test-{unique}"));

        fs::create_dir_all(&directory).unwrap();
        directory.join(file_name)
    }
}
