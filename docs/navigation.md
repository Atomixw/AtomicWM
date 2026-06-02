# Spatial Focus Navigation

AtomicWM navigates focus using window positions in world space.

The current implementation uses each window's center point. It does not use edges, occlusion, cones, workspaces, tiling, or snapping.

## Direction Filter

For a focused window center `F` and candidate center `C`:

- `Left`: `C.x < F.x`
- `Right`: `C.x > F.x`
- `Up`: `C.y < F.y`
- `Down`: `C.y > F.y`

Candidates outside the requested direction are ignored.

## Scoring

Candidates are ranked by primary distance along the requested axis and secondary distance on the perpendicular axis.

For `Right`:

```text
primary = C.x - F.x
secondary = abs(C.y - F.y)
score = primary * 1000.0 + secondary
```

The other directions use the same rule with the relevant axis.

The smallest score wins. This keeps direction more important than slight alignment differences.

## Empty Focus

If no window is focused, directional navigation focuses the first window.

If there are no windows, navigation returns `None`.

## Limitations

This is an initial model for internal behavior and tests. Cone search, edge-aware navigation, occlusion, and cluster-aware navigation can be added later.
