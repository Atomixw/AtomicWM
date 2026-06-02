# Snapping

AtomicWM's current snapping model is logical only. It compares window rectangles in world space and returns a movement adjustment.

It does not use Wayland surfaces, pointer events, rendering, animations, clusters, tiling, or workspaces.

## Edges

The model supports edge-to-edge snapping:

- moving left edge to target right edge
- moving right edge to target left edge
- moving top edge to target bottom edge
- moving bottom edge to target top edge

Only one axis snaps at a time.

Horizontal snapping requires vertical overlap between the moving rectangle and the target rectangle. Vertical snapping requires horizontal overlap. Touching edges count as overlap.

## Settings

Snapping uses:

- `enabled`
- `threshold`
- `gap`

The threshold is the maximum edge distance that can snap. The gap is applied between snapped edges.

## Limits

- no diagonal snapping
- no clusters
- no graph-based grouping
- no collision avoidance
- no snapping while resizing yet

Real pointer dragging can call this logic later.
