# Planned Architecture

AtomicWM should keep a small internal model and separate protocol details from spatial policy. The architecture should support testing the canvas model before full compositor behavior exists.

## app

Responsibility: Own the top-level runtime, initialize modules, run the main event loop, and coordinate shutdown.

Data it owns: Application state, module handles, runtime flags, and high-level lifecycle state.

What it should not own: Geometry math, Wayland protocol object internals, rendering resources, or window placement policy.

## backend

Responsibility: Abstract the system backend used by the compositor.

Data it owns: Backend handles for display devices, input devices, output creation, and platform-specific state.

What it should not own: Window rules, focus policy, canvas navigation decisions, or user configuration semantics.

## config

Responsibility: Load and validate user configuration.

Data it owns: Parsed configuration, defaults, keybinding definitions, and early startup options.

What it should not own: Runtime input state, rendered surfaces, window objects, or live protocol resources.

## geometry

Responsibility: Provide unit-aware spatial primitives and operations.

Data it owns: Types such as points, vectors, sizes, rectangles, scales, and intersection helpers.

What it should not own: Windows, outputs, cameras, input events, or compositor protocol state.

## canvas

Responsibility: Model the 2D world, cameras, viewports, pan, zoom, and coordinate conversion.

Data it owns: World bounds if needed, camera position, viewport size, zoom level, and transformation helpers.

What it should not own: Wayland surfaces, keyboard state, pointer devices, decorations, or app-specific window rules.

## window

Responsibility: Represent managed windows in compositor-neutral terms.

Data it owns: Window IDs, world rectangles, focus state, mapped/unmapped state, size constraints, and links to protocol surface handles through stable references.

What it should not own: Backend devices, raw input events, renderer pipelines, or global camera state.

## input

Responsibility: Translate keyboard and pointer events into compositor actions.

Data it owns: Input device state, pointer position, focus targets, grabs, and resolved keybinding actions.

What it should not own: Geometry primitives beyond using them, render resources, protocol implementation details, or persistent configuration storage.

## render

Responsibility: Draw outputs from the current window and canvas state.

Data it owns: Renderer resources, frame state, surface textures, damage tracking, and output render targets.

What it should not own: Window placement decisions, keybinding definitions, protocol object lifetimes, or configuration parsing.

## protocols

Responsibility: Contain Wayland protocol integration and translate protocol events into internal state changes.

Data it owns: Protocol objects for xdg-shell, compositor globals, seats, outputs, layer-shell, and future XWayland integration points.

What it should not own: Canvas policy, user-facing configuration, rendering algorithms, or spatial navigation rules.
