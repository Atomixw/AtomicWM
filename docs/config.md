# Configuration

AtomicWM reads TOML configuration from:

```text
~/.config/atomicwm/config.toml
```

If the file does not exist, AtomicWM uses the built-in defaults. It does not create the file automatically.

If the file exists and cannot be read, parsing fails, or validation fails, startup returns an error.

## Sections

`[general]`

- `mod_key`
- `focus_follows_mouse`

`[camera]`

- `pan_step`
- `zoom_step`
- `min_zoom`
- `max_zoom`

`[appearance]`

- `border_width`
- `gap`
- `background`
- `focused_border`
- `normal_border`

`[keybindings]`

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

`[commands]`

- `terminal`

## Validation

- `camera.pan_step` must be greater than `0`
- `camera.zoom_step` must be greater than `1`
- `camera.min_zoom` must be greater than `0`
- `camera.max_zoom` must be greater than `camera.min_zoom`
- `appearance.border_width` must be greater than or equal to `0`
- `appearance.gap` must be greater than or equal to `0`
- colors must use `#RRGGBB`
- `commands.terminal` must not be empty
- keybinding strings must not be empty

## Example

See [../config.example.toml](../config.example.toml) for all supported fields with default values.
