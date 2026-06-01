# AtomicWM

AtomicWM is a planned experimental Wayland compositor written in Rust, based on a 2D world-space model for window placement and navigation.

## Current Status

AtomicWM is not implemented yet.

This repository currently contains project documentation only. It does not contain compositor code, a Rust crate, dependencies, or a working desktop environment.

## Design Goal

AtomicWM will model windows as objects placed in a continuous 2D world. Outputs will show viewports into that world. The viewport can pan and zoom, so moving around the desktop is treated as camera movement rather than switching between fixed workspaces.

The first implementation should keep the model small enough to test directly:

- world-space rectangles for windows
- screen-space rectangles for outputs
- explicit camera state
- reversible coordinate conversion
- simple input operations for moving windows and moving the camera

## Non-Goals

AtomicWM is not intended to be a traditional workspace-based compositor.

The initial project will not include:

- a complete desktop shell
- panels, launchers, or custom bars
- animation systems
- visual effects such as blur or shader effects
- plugin APIs
- IPC protocols
- XWayland support
- multi-output layout management
- copied code, structure, wording, or implementation details from driftwm, vxwm, or other projects

## Core Model

AtomicWM is organized around a 2D canvas:

- windows live at world coordinates
- outputs display viewports into the world
- each viewport has a camera position and zoom level
- pan changes the camera position
- zoom changes the scale between world units and screen pixels
- focus and navigation are based on spatial relationships

Workspaces may be simulated later as regions, clusters, or named views, but they are not the base abstraction.

## Planned MVP

The first real MVP should:

- start as a Wayland compositor
- create one output
- accept Wayland clients
- show one xdg-shell window
- support keyboard focus
- support pointer focus
- move a window
- resize a window
- pan the camera
- zoom the camera
- exit with a keybinding

The MVP should prove that real Wayland surfaces can be positioned through the world/camera model.

## Development Phases

Initial work should proceed in small phases:

1. define the project and document the model
2. create a Rust project skeleton
3. implement geometry primitives
4. implement the world and camera model
5. implement an internal window model
6. add a simulation mode for testing without Wayland clients
7. add configuration loading
8. build a minimal Wayland compositor
9. show the first xdg-shell window
10. add keyboard and pointer input
11. add window movement and resizing
12. apply pan and zoom to real surfaces
13. add spatial focus navigation
14. add snapping
15. add implicit clusters
16. add decorations
17. add window rules
18. add layer-shell support
19. add multi-output support
20. add XWayland support
21. add polish and debugging tools

See [docs/roadmap.md](docs/roadmap.md) for the detailed roadmap.

## License

License: To be decided.
