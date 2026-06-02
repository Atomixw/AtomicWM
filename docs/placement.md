# Window Placement

AtomicWM separates logical placement from Wayland surface creation.

Placement computes a world-space rectangle for a future window. It does not create a real client, allocate a Wayland surface, render anything, or choose an output.

## Placement Modes

`ViewportCenter`

Places the new window centered on the current camera position.

`NearFocused`

Places the new window to the right of the focused window:

```text
x = focused.right + gap
y = focused.top
```

If no window is focused, it falls back to `ViewportCenter`.

`AtWorldPosition(point)`

Uses `point` as the top-left world coordinate.

## Request

A placement request contains:

- window size
- placement mode
- gap

Width and height must be greater than `0`. Gap must be greater than or equal to `0`.

## Layout Helpers

The world model can:

- center the camera on the focused window
- fit the camera to all windows
- fit the camera to the focused window

These helpers only update camera state.

## Limitations

- no cursor placement
- no output-aware placement
- no collision avoidance
- no snapping yet
- no workspaces
- no tiling
