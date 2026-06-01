# Canvas Model

AtomicWM uses a continuous 2D world. Windows have rectangles in world coordinates. Outputs show a viewport into that world.

## Coordinates

World coordinates use `f64`.

- `x` grows to the right
- `y` grows downward
- a rectangle origin is its top-left corner
- rectangle width and height are world units

Screen coordinates also use `f64`. They describe positions inside an output viewport. The top-left screen point is `(0, 0)`.

## Camera

The camera stores:

```text
position: world point at the center of the viewport
zoom: scale from world units to screen units
viewport_size: screen size of the output viewport
```

At `zoom = 1.0`, one world unit maps to one screen unit. At `zoom = 2.0`, one world unit maps to two screen units.

Invalid zoom values are clamped. Zoom must not be zero, negative, NaN, or infinite.

## Transforms

`world_to_screen` maps the camera position to the center of the viewport:

```text
screen.x = viewport.width / 2 + (world.x - camera.x) * zoom
screen.y = viewport.height / 2 + (world.y - camera.y) * zoom
```

`screen_to_world` is the inverse:

```text
world.x = camera.x + (screen.x - viewport.width / 2) / zoom
world.y = camera.y + (screen.y - viewport.height / 2) / zoom
```

These formulas are used for hit testing and for placing future surfaces.

## Pan and Zoom

Pan moves the camera in world units:

```text
camera.position += delta
```

`zoom_at(screen_point, zoom_factor)` changes zoom while keeping the world point under `screen_point` stable. This is needed for pointer-centered zoom.

## fit_rect

`fit_rect(rect, padding)` centers the camera on `rect` and chooses a zoom that fits the padded rectangle inside the viewport.

It does not animate. It only updates camera state.
