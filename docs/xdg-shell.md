# xdg-shell

AtomicWM has initial xdg-shell toplevel lifecycle support.

This is protocol and state tracking only. Client surfaces are accepted and configured, but client contents are not rendered yet.

## Current Behavior

- creates the Smithay xdg-shell global
- handles new xdg toplevel creation
- sends an initial configure before the first client buffer
- maps a toplevel when its surface commits a buffer
- assigns mapped toplevels a logical `WindowId`
- creates a matching `WindowNode` in `World` on map
- unmaps a logical window when its surface loses the buffer
- tracks destroyed toplevels and removes their logical windows
- keeps focus on another window when the focused logical window is removed
- sends a minimal configure for popups so clients do not panic the compositor

## Logical Mapping

Smithay protocol handles stay in the backend state.

`WindowNode` remains protocol-agnostic. It stores logical state such as title, app id, rect, focus, mapped state, and pending size.

The backend keeps a small mapping from xdg toplevel handle to `WindowId`. A toplevel may exist before it has a `WindowId`; this happens before the first buffer commit.

## Initial Size

If a toplevel has a usable pending size, AtomicWM uses it.

Otherwise the initial logical size is:

```text
800 x 600
```

The window is placed through the existing placement helper. New xdg toplevels currently use `NearFocused`, falling back to the viewport center when nothing is focused.

## Limitations

- no client surface rendering yet
- no layer-shell
- no XWayland
- no server-side decoration protocol
- no real popup placement
- no fullscreen, maximize, or minimize behavior
- no advanced configure negotiation
- no pointer-driven move or resize
- no keyboard focus protocol integration yet
