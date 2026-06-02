# Rendering

AtomicWM currently has only clear-screen rendering.

The renderer stores the configured background color and produces a clear frame for one logical output. This proves the backend can run a frame path without introducing window rendering yet.

The current backend does not open a visible GPU-backed window. A winit or DRM presenter will be added later.

## Current Behavior

- one logical output named `atomicwm-0`
- default output size is `1920 x 1080`
- background color comes from `appearance.background`
- color format is `#RRGGBB`
- backend test mode runs a few frame cycles and exits

## Not Implemented

- client surface rendering
- decoration rendering
- cursor rendering
- text rendering
- wallpaper support
- animations
- shaders
- multi-output rendering
- visible winit or DRM presentation

Future rendering work should add Wayland surfaces first, then decoration geometry and canvas transforms.
