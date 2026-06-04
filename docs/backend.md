# Backend

AtomicWM now has a minimal Wayland backend using Smithay.

This is still early infrastructure. It creates a Wayland display, registers a listening socket, runs a small event loop, owns one logical output, and runs a clear-frame path.

## Implemented

- Smithay `wayland_frontend`
- Wayland display creation
- automatic Wayland socket selection
- client socket acceptance
- minimal compositor global
- xdg-shell global
- xdg toplevel lifecycle tracking
- one logical output
- clear-screen frame path
- event loop dispatch
- `--backend-test` mode for startup/shutdown checks

## Not Implemented

- layer-shell
- XWayland
- GPU setup
- real surface presentation
- keyboard or pointer input
- real window management
- connecting logical `World` windows to Wayland surfaces
- advanced xdg-shell configure negotiation

The backend can track xdg-shell toplevels, but it does not display application windows yet. Clear rendering uses the configured background color, but visible GPU-backed presentation is not implemented yet.

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

The next backend step should connect tracked xdg toplevels to surface rendering without adding unrelated protocols.
