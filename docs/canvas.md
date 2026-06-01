# Canvas Model

AtomicWM uses a continuous 2D world instead of fixed workspaces. Windows are placed in world coordinates. Outputs show screen-space views into that world.

## World Coordinates

World coordinates describe where objects live on the canvas.

A window has a rectangle:

```text
window = { x, y, width, height }
```

`x` and `y` are world coordinates. `width` and `height` are world units.

World units should be treated as logical units. They are not tied directly to physical pixels.

## Screen Coordinates

Screen coordinates describe pixels or logical pixels on an output.

The top-left of an output viewport is usually:

```text
screen = { x: 0, y: 0 }
```

Screen coordinates are used for pointer position, output size, and final placement on the display.

## Camera

A camera defines which part of the world an output is looking at.

Initial camera state can be represented as:

```text
camera.position = world point at the viewport origin
camera.zoom = scale factor
```

At `zoom = 1.0`, one world unit maps to one screen unit. At `zoom = 2.0`, one world unit maps to two screen units.

## Viewport

A viewport is the visible screen area for an output.

```text
viewport = { width, height }
```

The viewport does not move by itself. The camera moves over the world, and the viewport displays the result.

## Zoom

Zoom changes scale.

```text
screen_size = world_size * zoom
world_size = screen_size / zoom
```

Zoom should be clamped to a practical range. The exact range can be decided during implementation.

## Pan

Pan changes the camera position.

```text
camera.position.x += dx
camera.position.y += dy
```

If panning is controlled by pointer motion in screen coordinates, the motion must be converted to world units:

```text
world_dx = screen_dx / zoom
world_dy = screen_dy / zoom
```

## world_to_screen

`world_to_screen` converts a world point to a screen point.

```text
screen.x = (world.x - camera.position.x) * camera.zoom
screen.y = (world.y - camera.position.y) * camera.zoom
```

For rectangles:

```text
screen.x = (world.x - camera.position.x) * camera.zoom
screen.y = (world.y - camera.position.y) * camera.zoom
screen.width = world.width * camera.zoom
screen.height = world.height * camera.zoom
```

## screen_to_world

`screen_to_world` converts a screen point to a world point.

```text
world.x = (screen.x / camera.zoom) + camera.position.x
world.y = (screen.y / camera.zoom) + camera.position.y
```

This is needed for pointer interaction. A click at a screen position must resolve to a world position before hit testing windows.

## Fitting a Window

Fitting one window means choosing a camera position and zoom that make the window visible in the viewport.

A simple first version can center the window:

```text
camera.position.x = window.center.x - (viewport.width / zoom) / 2
camera.position.y = window.center.y - (viewport.height / zoom) / 2
```

To fit the whole window:

```text
zoom_x = viewport.width / window.width
zoom_y = viewport.height / window.height
zoom = min(zoom_x, zoom_y) * margin
```

`margin` should be less than `1.0`, such as `0.9`, to leave space around the window.

## Fitting All Windows

Fitting all windows means computing the bounding rectangle of every mapped window.

```text
bounds = union(all_window_rectangles)
zoom_x = viewport.width / bounds.width
zoom_y = viewport.height / bounds.height
zoom = min(zoom_x, zoom_y) * margin
```

Then center the camera on `bounds.center`.

Empty worlds need a default camera position and zoom.

## Difference From Workspaces

Workspaces divide windows into discrete sets. A window is usually visible because it belongs to the active workspace.

AtomicWM uses position instead. A window is visible when its world rectangle intersects the current viewport after the camera transform.

This means navigation can be based on distance and direction:

- pan to nearby windows
- zoom out to see context
- zoom in to focus on a region
- group windows by proximity later

The base model does not require switching between named desktop containers.
