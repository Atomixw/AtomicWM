# Simulation Mode

Simulation mode exercises AtomicWM's internal world, camera, and window model from the terminal.

Run it with:

```bash
cargo run -- --simulate
```

or:

```bash
cargo run -- -s
```

The simulation creates fake windows, focuses one, moves and resizes windows, pans and zooms the camera, fits the camera to world bounds, prints state, and exits.

It does not start Wayland. It does not create real windows. It does not test rendering, input devices, xdg-shell, layer-shell, or compositor behavior.
