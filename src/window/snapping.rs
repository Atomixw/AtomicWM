use crate::{geometry::Rect, window::WindowId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapEdge {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SnapAdjustment {
    pub dx: f64,
    pub dy: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SnapCandidate {
    pub target: WindowId,
    pub edge: SnapEdge,
    pub adjustment: SnapAdjustment,
    pub distance: f64,
}

pub fn find_snap_adjustment<'a>(
    moving: Rect,
    targets: impl Iterator<Item = (&'a WindowId, &'a Rect)>,
    threshold: f64,
    gap: f64,
) -> Option<SnapCandidate> {
    let threshold = threshold.max(0.0);
    let gap = gap.max(0.0);

    targets
        .flat_map(|(target, rect)| snap_candidates(moving, *target, *rect, threshold, gap))
        .min_by(|left, right| left.distance.total_cmp(&right.distance))
}

fn snap_candidates(
    moving: Rect,
    target: WindowId,
    rect: Rect,
    threshold: f64,
    gap: f64,
) -> Vec<SnapCandidate> {
    let mut candidates = Vec::new();

    if vertical_overlap(moving, rect) {
        let desired_left = rect.right() + gap;
        let dx = desired_left - moving.left();
        push_candidate(
            &mut candidates,
            target,
            SnapEdge::LeftToRight,
            SnapAdjustment { dx, dy: 0.0 },
            dx.abs(),
            threshold,
        );

        let desired_right = rect.left() - gap;
        let dx = desired_right - moving.right();
        push_candidate(
            &mut candidates,
            target,
            SnapEdge::RightToLeft,
            SnapAdjustment { dx, dy: 0.0 },
            dx.abs(),
            threshold,
        );
    }

    if horizontal_overlap(moving, rect) {
        let desired_top = rect.bottom() + gap;
        let dy = desired_top - moving.top();
        push_candidate(
            &mut candidates,
            target,
            SnapEdge::TopToBottom,
            SnapAdjustment { dx: 0.0, dy },
            dy.abs(),
            threshold,
        );

        let desired_bottom = rect.top() - gap;
        let dy = desired_bottom - moving.bottom();
        push_candidate(
            &mut candidates,
            target,
            SnapEdge::BottomToTop,
            SnapAdjustment { dx: 0.0, dy },
            dy.abs(),
            threshold,
        );
    }

    candidates
}

fn push_candidate(
    candidates: &mut Vec<SnapCandidate>,
    target: WindowId,
    edge: SnapEdge,
    adjustment: SnapAdjustment,
    distance: f64,
    threshold: f64,
) {
    if distance <= threshold {
        candidates.push(SnapCandidate {
            target,
            edge,
            adjustment,
            distance,
        });
    }
}

fn vertical_overlap(left: Rect, right: Rect) -> bool {
    left.top() <= right.bottom() && left.bottom() >= right.top()
}

fn horizontal_overlap(left: Rect, right: Rect) -> bool {
    left.left() <= right.right() && left.right() >= right.left()
}

#[cfg(test)]
mod tests {
    use super::{SnapAdjustment, SnapEdge, find_snap_adjustment};
    use crate::{geometry::Rect, window::WindowId};

    fn targets(items: &[(WindowId, Rect)]) -> impl Iterator<Item = (&WindowId, &Rect)> {
        items.iter().map(|(id, rect)| (id, rect))
    }

    #[test]
    fn moving_left_edge_snaps_to_target_right_edge() {
        let items = [(WindowId::new(1), Rect::new(0.0, 0.0, 100.0, 100.0))];
        let moving = Rect::new(105.0, 0.0, 100.0, 100.0);

        let snap = find_snap_adjustment(moving, targets(&items), 10.0, 0.0).unwrap();

        assert_eq!(snap.edge, SnapEdge::LeftToRight);
        assert_eq!(snap.adjustment, SnapAdjustment { dx: -5.0, dy: 0.0 });
    }

    #[test]
    fn moving_right_edge_snaps_to_target_left_edge() {
        let items = [(WindowId::new(1), Rect::new(200.0, 0.0, 100.0, 100.0))];
        let moving = Rect::new(95.0, 0.0, 100.0, 100.0);

        let snap = find_snap_adjustment(moving, targets(&items), 10.0, 0.0).unwrap();

        assert_eq!(snap.edge, SnapEdge::RightToLeft);
        assert_eq!(snap.adjustment, SnapAdjustment { dx: 5.0, dy: 0.0 });
    }

    #[test]
    fn moving_top_edge_snaps_to_target_bottom_edge() {
        let items = [(WindowId::new(1), Rect::new(0.0, 0.0, 100.0, 100.0))];
        let moving = Rect::new(0.0, 105.0, 100.0, 100.0);

        let snap = find_snap_adjustment(moving, targets(&items), 10.0, 0.0).unwrap();

        assert_eq!(snap.edge, SnapEdge::TopToBottom);
        assert_eq!(snap.adjustment, SnapAdjustment { dx: 0.0, dy: -5.0 });
    }

    #[test]
    fn moving_bottom_edge_snaps_to_target_top_edge() {
        let items = [(WindowId::new(1), Rect::new(0.0, 200.0, 100.0, 100.0))];
        let moving = Rect::new(0.0, 95.0, 100.0, 100.0);

        let snap = find_snap_adjustment(moving, targets(&items), 10.0, 0.0).unwrap();

        assert_eq!(snap.edge, SnapEdge::BottomToTop);
        assert_eq!(snap.adjustment, SnapAdjustment { dx: 0.0, dy: 5.0 });
    }

    #[test]
    fn no_snap_when_distance_exceeds_threshold() {
        let items = [(WindowId::new(1), Rect::new(0.0, 0.0, 100.0, 100.0))];
        let moving = Rect::new(125.0, 0.0, 100.0, 100.0);

        assert!(find_snap_adjustment(moving, targets(&items), 10.0, 0.0).is_none());
    }

    #[test]
    fn no_horizontal_snap_without_vertical_overlap() {
        let items = [(WindowId::new(1), Rect::new(0.0, 0.0, 100.0, 100.0))];
        let moving = Rect::new(105.0, 101.0, 100.0, 100.0);

        assert!(find_snap_adjustment(moving, targets(&items), 10.0, 0.0).is_none());
    }

    #[test]
    fn no_vertical_snap_without_horizontal_overlap() {
        let items = [(WindowId::new(1), Rect::new(0.0, 0.0, 100.0, 100.0))];
        let moving = Rect::new(101.0, 105.0, 100.0, 100.0);

        assert!(find_snap_adjustment(moving, targets(&items), 10.0, 0.0).is_none());
    }

    #[test]
    fn closest_snap_candidate_wins() {
        let items = [
            (WindowId::new(1), Rect::new(0.0, 0.0, 100.0, 100.0)),
            (WindowId::new(2), Rect::new(210.0, 0.0, 100.0, 100.0)),
        ];
        let moving = Rect::new(105.0, 0.0, 100.0, 100.0);

        let snap = find_snap_adjustment(moving, targets(&items), 20.0, 0.0).unwrap();

        assert_eq!(snap.target, WindowId::new(1));
        assert_eq!(snap.distance, 5.0);
    }

    #[test]
    fn gap_is_respected() {
        let items = [(WindowId::new(1), Rect::new(0.0, 0.0, 100.0, 100.0))];
        let moving = Rect::new(112.0, 0.0, 100.0, 100.0);

        let snap = find_snap_adjustment(moving, targets(&items), 10.0, 8.0).unwrap();

        assert_eq!(snap.adjustment, SnapAdjustment { dx: -4.0, dy: 0.0 });
    }
}
