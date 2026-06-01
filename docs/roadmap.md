# AtomicWM Roadmap

This roadmap is incremental. Each phase should leave the project in a testable state where possible. Later phases may change as implementation details become clearer.

## 1. Project Definition

Goal: Define what AtomicWM is and what it is not.

Expected output: Initial documentation covering the project scope, canvas model, MVP, and architecture.

What should not be included yet: Rust source files, dependencies, compositor code, or claims that the project is usable.

## 2. Rust Project Skeleton

Goal: Create the minimal Rust project layout.

Expected output: A crate that builds and runs a placeholder binary.

What should not be included yet: Wayland integration, rendering, window management, or large dependency choices.

## 3. Geometry Primitives

Goal: Define basic spatial types used throughout the project.

Expected output: Points, sizes, rectangles, vectors, and scale types with unit tests.

What should not be included yet: Window policy, output logic, rendering, or Wayland types in geometry code.

## 4. World/Camera Model

Goal: Implement the conversion between world coordinates and screen coordinates.

Expected output: Camera state, viewport state, pan, zoom, `world_to_screen`, and `screen_to_world`.

What should not be included yet: Real Wayland surfaces, compositor event loops, or input bindings.

## 5. Internal Window Model

Goal: Represent windows independently from Wayland protocol objects.

Expected output: Window records with IDs, world rectangles, focus state, and basic lifecycle state.

What should not be included yet: xdg-shell handling, decorations, rules, snapping, or clusters.

## 6. Simulation Mode

Goal: Test the canvas model without running a compositor.

Expected output: A local simulation that creates fake windows, moves the camera, and checks coordinate behavior.

What should not be included yet: Wayland clients, hardware backends, GPU rendering, or configuration complexity.

## 7. Configuration

Goal: Add a small configuration format for early options.

Expected output: Config loading for keybindings, initial camera values, and simple defaults.

What should not be included yet: Runtime reload, IPC, plugin hooks, or full rule systems.

## 8. Minimal Wayland Compositor

Goal: Start a basic Wayland compositor process.

Expected output: A compositor that can start, create a display, run an event loop, and create one output.

What should not be included yet: xdg-shell windows, camera transforms, decorations, or multi-output support.

## 9. First xdg-shell Window

Goal: Accept and display one xdg-shell client surface.

Expected output: One client window shown on the output and tracked by the internal window model.

What should not be included yet: Multiple window policy, resizing, moving, snapping, or focus navigation.

## 10. Keyboard and Pointer Input

Goal: Route basic input events to compositor state and focused clients.

Expected output: Keyboard focus, pointer focus, pointer motion, button handling, and one exit keybinding.

What should not be included yet: Gestures, configurable input devices, complex grabs, or global shortcut systems.

## 11. Moving and Resizing Windows

Goal: Change a window's world rectangle through pointer and keyboard operations.

Expected output: Basic move and resize operations that update the internal window model and rendered position.

What should not be included yet: Snapping, tiling, constraints beyond minimum viable bounds, or decorations.

## 12. Applying Pan and Zoom to Real Surfaces

Goal: Render real client surfaces through the camera transform.

Expected output: Panning and zooming the viewport changes how client surfaces appear on the output.

What should not be included yet: Animations, high-level navigation commands, visual effects, or per-output camera policies.

## 13. Spatial Focus Navigation

Goal: Move focus using spatial relationships between windows.

Expected output: Directional focus commands based on world-space positions.

What should not be included yet: Clusters, history-aware navigation, workspace emulation, or complex ranking rules.

## 14. Snapping

Goal: Add simple alignment behavior for nearby windows.

Expected output: Optional snapping to window edges, centers, and simple grid increments.

What should not be included yet: Constraint solvers, layout engines, or persistent groups.

## 15. Implicit Clusters

Goal: Detect loose groups of nearby windows without requiring explicit workspaces.

Expected output: Cluster detection based on distance, overlap, or user movement patterns.

What should not be included yet: Named workspaces, complex session persistence, or automatic rearrangement.

## 16. Decorations

Goal: Add basic server-side decoration support.

Expected output: Simple borders or title regions sufficient for resize and move affordances.

What should not be included yet: Theme engines, complex shadows, blur, or custom widget systems.

## 17. Window Rules

Goal: Apply simple rules when windows are created.

Expected output: Matching by app ID or title with initial size, position, focus, or floating behavior.

What should not be included yet: Scripting, IPC-driven rule updates, or compatibility layers for other window managers.

## 18. Layer-Shell Support

Goal: Support layer-shell clients used by panels, launchers, and overlays.

Expected output: Basic placement for background, bottom, top, and overlay layers.

What should not be included yet: Custom bars, desktop shell features, or advanced exclusive-zone policies.

## 19. Multi-Output Support

Goal: Show different viewports into the same world across multiple outputs.

Expected output: Multiple outputs with explicit viewport and camera state.

What should not be included yet: Complex monitor profiles, dynamic layout editors, or network display support.

## 20. XWayland Support

Goal: Run legacy X11 clients through XWayland.

Expected output: Basic XWayland startup and window tracking.

What should not be included yet: Full window-manager hint coverage, advanced X11 focus policies, or legacy desktop environment compatibility.

## 21. Polish and Debugging Tools

Goal: Make development and diagnosis practical.

Expected output: Logging, debug overlays, coordinate inspection, state dumps, and test fixtures.

What should not be included yet: A public plugin system, large UI tools, or promises of stability.
