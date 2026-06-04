# AtomicWM

AtomicWM is not a usable compositor yet.

This repository currently contains internal models, simulation mode, and a minimal Smithay-based Wayland backend.

AtomicWM can initialize minimal Wayland compositor infrastructure, run a clear-screen render path, and track basic xdg-shell toplevel lifecycle state. It does not display client windows yet.

Run the internal simulation with:

```bash
cargo run -- --simulate
```
