# Input Model

AtomicWM currently has an internal action and keybinding model only. It does not read real keyboard events yet.

Keybinding strings from config are parsed into small typed values. Those values map to internal actions. Later, the Wayland/backend layer will translate real key events into the same keybinding representation.

## Supported Actions

- `quit`
- `spawn_terminal`
- `zoom_in`
- `zoom_out`
- `reset_zoom`
- `pan_left`
- `pan_right`
- `pan_up`
- `pan_down`
- `focus_left`
- `focus_right`
- `focus_up`
- `focus_down`
- `center_focused`
- `fit_all`
- `move_cluster_left`
- `move_cluster_right`
- `move_cluster_up`
- `move_cluster_down`
- `fit_focused_cluster`

## Default Keybindings

- `quit = "Super+Shift+Q"`
- `spawn_terminal = "Super+Enter"`
- `zoom_in = "Super+Equal"`
- `zoom_out = "Super+Minus"`
- `reset_zoom = "Super+0"`
- `pan_left = "Super+Ctrl+Left"`
- `pan_right = "Super+Ctrl+Right"`
- `pan_up = "Super+Ctrl+Up"`
- `pan_down = "Super+Ctrl+Down"`
- `focus_left = "Super+Left"`
- `focus_right = "Super+Right"`
- `focus_up = "Super+Up"`
- `focus_down = "Super+Down"`
- `center_focused = "Super+C"`
- `fit_all = "Super+W"`
- `move_cluster_left = "Super+Shift+Ctrl+Left"`
- `move_cluster_right = "Super+Shift+Ctrl+Right"`
- `move_cluster_up = "Super+Shift+Ctrl+Up"`
- `move_cluster_down = "Super+Shift+Ctrl+Down"`
- `fit_focused_cluster = "Super+Shift+W"`

## Parser Scope

The parser supports these modifiers:

- `Super`
- `Ctrl`
- `Alt`
- `Shift`

The parser supports these keys:

- single ASCII alphanumeric characters
- `Enter`
- `Equal`
- `Minus`
- `Left`
- `Right`
- `Up`
- `Down`
- `0`

This is intentionally small. It is not a full hotkey language and does not use xkbcommon yet.
