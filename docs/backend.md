# Backend

AtomicWM now has a minimal Wayland backend using Smithay.

This is still early infrastructure. It creates a Wayland display, registers a listening socket, runs a small event loop, owns one logical output, and runs a clear-frame path.

## Implemented

- Smithay `wayland_frontend`
- Wayland display creation
- automatic Wayland socket selection
- client socket acceptance
- minimal compositor global
- one logical output
- clear-screen frame path
- event loop dispatch
- `--backend-test` mode for startup/shutdown checks

## Not Implemented

- xdg-shell
- layer-shell
- XWayland
- GPU setup
- real surface presentation
- keyboard or pointer input
- real window management
- connecting logical `World` windows to Wayland surfaces

The backend does not display application windows yet. Clear rendering uses the configured background color, but visible GPU-backed presentation is not implemented yet.

## Running

Run the minimal backend:

```bash
cargo run
```

Run backend startup/shutdown test mode:

```bash
cargo run -- --backend-test
```

Run the internal simulation:

```bash
cargo run -- --simulate
```

xdg-shell is intentionally left out. The next backend step should add one protocol at a time after the event loop and socket lifecycle are stable.
