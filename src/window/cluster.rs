use std::collections::HashSet;

use crate::{
    geometry::Rect,
    window::{WindowId, WindowNode},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClusterId(pub u64);

impl ClusterId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cluster {
    pub id: ClusterId,
    pub windows: Vec<WindowId>,
    pub bounds: Rect,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClusterGraph {
    edges: Vec<(WindowId, WindowId)>,
}

impl ClusterGraph {
    pub fn from_windows(windows: &[WindowNode], tolerance: f64) -> Self {
        let tolerance = tolerance.max(0.0);
        let mut edges = Vec::new();

        for (left_index, left) in windows.iter().enumerate() {
            for right in windows.iter().skip(left_index + 1) {
                if rects_connected(left.rect(), right.rect(), tolerance) {
                    edges.push((left.id, right.id));
                }
            }
        }

        Self { edges }
    }

    pub fn clusters(&self, windows: &[WindowNode]) -> Vec<Cluster> {
        let mut visited = HashSet::new();
        let mut clusters = Vec::new();

        for window in windows {
            if visited.contains(&window.id) {
                continue;
            }

            let mut ids = Vec::new();
            self.collect_component(window.id, windows, &mut visited, &mut ids);

            if let Some(bounds) = bounds_for(&ids, windows) {
                clusters.push(Cluster {
                    id: ClusterId::new((clusters.len() + 1) as u64),
                    windows: ids,
                    bounds,
                });
            }
        }

        clusters
    }

    pub fn cluster_for_window(&self, id: WindowId, windows: &[WindowNode]) -> Option<Cluster> {
        self.clusters(windows)
            .into_iter()
            .find(|cluster| cluster.windows.contains(&id))
    }

    fn collect_component(
        &self,
        id: WindowId,
        windows: &[WindowNode],
        visited: &mut HashSet<WindowId>,
        ids: &mut Vec<WindowId>,
    ) {
        if !visited.insert(id) {
            return;
        }

        ids.push(id);

        for neighbor in self.neighbors(id, windows) {
            self.collect_component(neighbor, windows, visited, ids);
        }
    }

    fn neighbors(&self, id: WindowId, windows: &[WindowNode]) -> Vec<WindowId> {
        windows
            .iter()
            .filter_map(|window| {
                if self.connected(id, window.id) {
                    Some(window.id)
                } else {
                    None
                }
            })
            .collect()
    }

    fn connected(&self, left: WindowId, right: WindowId) -> bool {
        self.edges
            .iter()
            .any(|(a, b)| (*a == left && *b == right) || (*a == right && *b == left))
    }
}

fn bounds_for(ids: &[WindowId], windows: &[WindowNode]) -> Option<Rect> {
    let mut rects = ids.iter().filter_map(|id| {
        windows
            .iter()
            .find(|window| window.id == *id)
            .map(WindowNode::rect)
    });
    let first = rects.next()?;

    Some(rects.fold(first, |bounds, rect| bounds.union(rect)))
}

fn rects_connected(left: Rect, right: Rect, tolerance: f64) -> bool {
    let horizontal = vertical_overlap(left, right)
        && ((left.right() - right.left()).abs() <= tolerance
            || (left.left() - right.right()).abs() <= tolerance);
    let vertical = horizontal_overlap(left, right)
        && ((left.bottom() - right.top()).abs() <= tolerance
            || (left.top() - right.bottom()).abs() <= tolerance);

    horizontal || vertical
}

fn vertical_overlap(left: Rect, right: Rect) -> bool {
    left.bottom().min(right.bottom()) - left.top().max(right.top()) > 0.0
}

fn horizontal_overlap(left: Rect, right: Rect) -> bool {
    left.right().min(right.right()) - left.left().max(right.left()) > 0.0
}

#[cfg(test)]
mod tests {
    use super::{ClusterGraph, ClusterId};
    use crate::{
        geometry::Rect,
        window::{WindowId, WindowNode},
    };

    fn window(id: u64, rect: Rect) -> WindowNode {
        WindowNode::new(WindowId::new(id), format!("Window {id}"), "test-app", rect)
    }

    #[test]
    fn single_window_creates_one_cluster() {
        let windows = [window(1, Rect::new(0.0, 0.0, 100.0, 100.0))];
        let graph = ClusterGraph::from_windows(&windows, 1.0);
        let clusters = graph.clusters(&windows);

        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0].windows, vec![WindowId::new(1)]);
    }

    #[test]
    fn two_touching_horizontal_windows_create_one_cluster() {
        let windows = [
            window(1, Rect::new(0.0, 0.0, 100.0, 100.0)),
            window(2, Rect::new(100.0, 0.0, 100.0, 100.0)),
        ];
        let graph = ClusterGraph::from_windows(&windows, 1.0);

        assert_eq!(graph.clusters(&windows).len(), 1);
    }

    #[test]
    fn two_touching_vertical_windows_create_one_cluster() {
        let windows = [
            window(1, Rect::new(0.0, 0.0, 100.0, 100.0)),
            window(2, Rect::new(0.0, 100.0, 100.0, 100.0)),
        ];
        let graph = ClusterGraph::from_windows(&windows, 1.0);

        assert_eq!(graph.clusters(&windows).len(), 1);
    }

    #[test]
    fn separated_windows_create_separate_clusters() {
        let windows = [
            window(1, Rect::new(0.0, 0.0, 100.0, 100.0)),
            window(2, Rect::new(300.0, 0.0, 100.0, 100.0)),
        ];
        let graph = ClusterGraph::from_windows(&windows, 1.0);

        assert_eq!(graph.clusters(&windows).len(), 2);
    }

    #[test]
    fn diagonal_corner_touching_does_not_create_one_cluster() {
        let windows = [
            window(1, Rect::new(0.0, 0.0, 100.0, 100.0)),
            window(2, Rect::new(100.0, 100.0, 100.0, 100.0)),
        ];
        let graph = ClusterGraph::from_windows(&windows, 1.0);

        assert_eq!(graph.clusters(&windows).len(), 2);
    }

    #[test]
    fn overlapping_but_not_edge_aligned_windows_do_not_connect() {
        let windows = [
            window(1, Rect::new(0.0, 0.0, 100.0, 100.0)),
            window(2, Rect::new(50.0, 50.0, 100.0, 100.0)),
        ];
        let graph = ClusterGraph::from_windows(&windows, 1.0);

        assert_eq!(graph.clusters(&windows).len(), 2);
    }

    #[test]
    fn three_connected_windows_create_one_cluster() {
        let windows = [
            window(1, Rect::new(0.0, 0.0, 100.0, 100.0)),
            window(2, Rect::new(100.0, 0.0, 100.0, 100.0)),
            window(3, Rect::new(200.0, 0.0, 100.0, 100.0)),
        ];
        let graph = ClusterGraph::from_windows(&windows, 1.0);

        assert_eq!(graph.clusters(&windows).len(), 1);
    }

    #[test]
    fn cluster_bounds_are_union_of_member_rects() {
        let windows = [
            window(1, Rect::new(0.0, 0.0, 100.0, 100.0)),
            window(2, Rect::new(100.0, 50.0, 100.0, 100.0)),
        ];
        let graph = ClusterGraph::from_windows(&windows, 1.0);
        let clusters = graph.clusters(&windows);

        assert_eq!(clusters[0].id, ClusterId::new(1));
        assert_eq!(clusters[0].bounds, Rect::new(0.0, 0.0, 200.0, 150.0));
    }
}
