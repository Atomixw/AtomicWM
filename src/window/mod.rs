mod cluster;
mod decoration;
mod navigation;
mod node;
mod placement;
mod snapping;

pub use cluster::{Cluster, ClusterGraph, ClusterId};
pub use decoration::{
    BorderRects, DecorationGeometry, DecorationHit, DecorationMode, compute_decoration_geometry,
};
pub use navigation::{Direction, find_window_in_direction};
pub use node::{WindowId, WindowNode};
pub use placement::{PlacementMode, PlacementRequest};
pub use snapping::{SnapAdjustment, SnapCandidate, SnapEdge, find_snap_adjustment};
