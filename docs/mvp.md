# MVP

The first real MVP should prove that AtomicWM can act as a Wayland compositor and apply the canvas model to a real client surface.

The MVP is not a complete desktop environment.

## Required Behavior

AtomicWM should start as a Wayland compositor.

It should create one output and accept Wayland clients.

It should show one xdg-shell window. Supporting more than one window can happen after the first surface path is reliable.

Keyboard focus should be assigned to the visible xdg-shell window.

Pointer focus should follow hit testing against the visible window.

The compositor should support moving a window by changing its world rectangle.

The compositor should support resizing a window by changing its world rectangle and applying the size to the client surface.

The camera should be pannable.

The camera should be zoomable.

Pan and zoom should affect how the xdg-shell window appears on the output.

There should be a keybinding that exits the compositor.

## Expected Result

At the end of the MVP, a developer should be able to:

- start AtomicWM
- launch or connect one Wayland client
- see that client's xdg-shell surface
- focus it with keyboard and pointer input
- move and resize it
- pan around the world
- zoom the viewport
- exit cleanly with a keybinding

## Out of Scope for MVP

- animations
- blur
- shaders
- XWayland
- multi-monitor
- gestures
- IPC
- plugin system
- custom bars
- complex decorations
