use std::{collections::HashMap, str::FromStr};

use crate::{
    config::Config,
    input::{Action, KeyBinding, KeyBindingParseError},
};

#[derive(Debug, Clone)]
pub struct KeyMap {
    bindings: HashMap<KeyBinding, Action>,
}

impl KeyMap {
    pub fn from_config(config: &Config) -> Result<Self, KeyBindingParseError> {
        let entries = [
            ("quit", &config.keybindings.quit, Action::Quit),
            (
                "spawn_terminal",
                &config.keybindings.spawn_terminal,
                Action::SpawnTerminal,
            ),
            ("zoom_in", &config.keybindings.zoom_in, Action::ZoomIn),
            ("zoom_out", &config.keybindings.zoom_out, Action::ZoomOut),
            (
                "reset_zoom",
                &config.keybindings.reset_zoom,
                Action::ResetZoom,
            ),
            ("pan_left", &config.keybindings.pan_left, Action::PanLeft),
            ("pan_right", &config.keybindings.pan_right, Action::PanRight),
            ("pan_up", &config.keybindings.pan_up, Action::PanUp),
            ("pan_down", &config.keybindings.pan_down, Action::PanDown),
            (
                "focus_left",
                &config.keybindings.focus_left,
                Action::FocusLeft,
            ),
            (
                "focus_right",
                &config.keybindings.focus_right,
                Action::FocusRight,
            ),
            ("focus_up", &config.keybindings.focus_up, Action::FocusUp),
            (
                "focus_down",
                &config.keybindings.focus_down,
                Action::FocusDown,
            ),
            (
                "center_focused",
                &config.keybindings.center_focused,
                Action::CenterFocused,
            ),
            ("fit_all", &config.keybindings.fit_all, Action::FitAll),
            (
                "move_cluster_left",
                &config.keybindings.move_cluster_left,
                Action::MoveClusterLeft,
            ),
            (
                "move_cluster_right",
                &config.keybindings.move_cluster_right,
                Action::MoveClusterRight,
            ),
            (
                "move_cluster_up",
                &config.keybindings.move_cluster_up,
                Action::MoveClusterUp,
            ),
            (
                "move_cluster_down",
                &config.keybindings.move_cluster_down,
                Action::MoveClusterDown,
            ),
            (
                "fit_focused_cluster",
                &config.keybindings.fit_focused_cluster,
                Action::FitFocusedCluster,
            ),
        ];

        let mut bindings = HashMap::new();
        let mut names = HashMap::new();

        for (name, value, action) in entries {
            let binding = KeyBinding::from_str(value)?;

            if let Some(first) = names.insert(binding, name) {
                return Err(KeyBindingParseError::DuplicateBinding {
                    first,
                    second: name,
                });
            }

            bindings.insert(binding, action);
        }

        Ok(Self { bindings })
    }

    pub fn get(&self, binding: &KeyBinding) -> Option<Action> {
        self.bindings.get(binding).copied()
    }

    pub fn bindings(&self) -> impl Iterator<Item = (&KeyBinding, &Action)> {
        self.bindings.iter()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::KeyMap;
    use crate::{
        config::Config,
        input::{Action, KeyBinding, KeyBindingParseError},
    };

    #[test]
    fn builds_from_default_config() {
        let keymap = KeyMap::from_config(&Config::default()).unwrap();

        assert_eq!(keymap.bindings().count(), 20);
    }

    #[test]
    fn returns_expected_action_for_known_binding() {
        let keymap = KeyMap::from_config(&Config::default()).unwrap();
        let binding = KeyBinding::from_str("Super+Equal").unwrap();

        assert_eq!(keymap.get(&binding), Some(Action::ZoomIn));
    }

    #[test]
    fn rejects_duplicate_bindings() {
        let mut config = Config::default();
        config.keybindings.fit_all = config.keybindings.center_focused.clone();

        assert!(matches!(
            KeyMap::from_config(&config),
            Err(KeyBindingParseError::DuplicateBinding { .. })
        ));
    }
}
