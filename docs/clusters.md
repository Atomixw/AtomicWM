# Implicit Clusters

AtomicWM computes clusters from current window geometry.

Clusters are not stored as permanent user groups. They are rebuilt when code asks for them.

## Connection Rule

Two windows are connected when compatible edges touch or nearly touch within a tolerance:

- right edge to left edge, with vertical overlap
- left edge to right edge, with vertical overlap
- bottom edge to top edge, with horizontal overlap
- top edge to bottom edge, with horizontal overlap

Touching along an edge counts. Diagonal corner contact does not.

Each connected component becomes one cluster. A window with no connections is still a cluster of one.

## World Helpers

The world model can:

- return all clusters
- find the cluster for a window
- find the focused cluster
- move every window in a cluster
- fit the camera to the focused cluster

These helpers only use logical rectangles.

## Limits

- no visual cluster indicators
- no persistent grouping
- no cluster-aware snapping
- no pointer-driven cluster movement yet
- no tiling or workspaces
