use crate::window::{WindowId, WindowNode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub fn find_window_in_direction(
    windows: &[WindowNode],
    focused_id: WindowId,
    direction: Direction,
) -> Option<WindowId> {
    let focused = windows.iter().find(|window| window.id == focused_id)?;
    let focused_center = focused.center();

    windows
        .iter()
        .filter(|window| window.id != focused_id)
        .filter_map(|window| {
            let candidate_center = window.center();
            let (primary, secondary) = match direction {
                Direction::Left if candidate_center.x < focused_center.x => (
                    focused_center.x - candidate_center.x,
                    (candidate_center.y - focused_center.y).abs(),
                ),
                Direction::Right if candidate_center.x > focused_center.x => (
                    candidate_center.x - focused_center.x,
                    (candidate_center.y - focused_center.y).abs(),
                ),
                Direction::Up if candidate_center.y < focused_center.y => (
                    focused_center.y - candidate_center.y,
                    (candidate_center.x - focused_center.x).abs(),
                ),
                Direction::Down if candidate_center.y > focused_center.y => (
                    candidate_center.y - focused_center.y,
                    (candidate_center.x - focused_center.x).abs(),
                ),
                _ => return None,
            };

            Some((window.id, primary * 1000.0 + secondary))
        })
        .min_by(|(_, left_score), (_, right_score)| left_score.total_cmp(right_score))
        .map(|(id, _)| id)
}

#[cfg(test)]
mod tests {
    use super::{Direction, find_window_in_direction};
    use crate::{
        geometry::Rect,
        window::{WindowId, WindowNode},
    };

    fn window(id: u64, rect: Rect) -> WindowNode {
        WindowNode::new(WindowId::new(id), format!("Window {id}"), "test-app", rect)
    }

    #[test]
    fn selects_nearest_window_to_the_right() {
        let windows = [
            window(1, Rect::new(0.0, 0.0, 100.0, 100.0)),
            window(2, Rect::new(300.0, 0.0, 100.0, 100.0)),
            window(3, Rect::new(600.0, 0.0, 100.0, 100.0)),
        ];

        assert_eq!(
            find_window_in_direction(&windows, WindowId::new(1), Direction::Right),
            Some(WindowId::new(2))
        );
    }

    #[test]
    fn selects_nearest_window_to_the_left() {
        let windows = [
            window(1, Rect::new(0.0, 0.0, 100.0, 100.0)),
            window(2, Rect::new(-300.0, 0.0, 100.0, 100.0)),
            window(3, Rect::new(-600.0, 0.0, 100.0, 100.0)),
        ];

        assert_eq!(
            find_window_in_direction(&windows, WindowId::new(1), Direction::Left),
            Some(WindowId::new(2))
        );
    }

    #[test]
    fn selects_nearest_window_above() {
        let windows = [
            window(1, Rect::new(0.0, 0.0, 100.0, 100.0)),
            window(2, Rect::new(0.0, -300.0, 100.0, 100.0)),
            window(3, Rect::new(0.0, -600.0, 100.0, 100.0)),
        ];

        assert_eq!(
            find_window_in_direction(&windows, WindowId::new(1), Direction::Up),
            Some(WindowId::new(2))
        );
    }

    #[test]
    fn selects_nearest_window_below() {
        let windows = [
            window(1, Rect::new(0.0, 0.0, 100.0, 100.0)),
            window(2, Rect::new(0.0, 300.0, 100.0, 100.0)),
            window(3, Rect::new(0.0, 600.0, 100.0, 100.0)),
        ];

        assert_eq!(
            find_window_in_direction(&windows, WindowId::new(1), Direction::Down),
            Some(WindowId::new(2))
        );
    }

    #[test]
    fn uses_perpendicular_distance_as_tie_breaker() {
        let windows = [
            window(1, Rect::new(0.0, 0.0, 100.0, 100.0)),
            window(2, Rect::new(300.0, 400.0, 100.0, 100.0)),
            window(3, Rect::new(300.0, 20.0, 100.0, 100.0)),
        ];

        assert_eq!(
            find_window_in_direction(&windows, WindowId::new(1), Direction::Right),
            Some(WindowId::new(3))
        );
    }

    #[test]
    fn returns_none_when_no_candidate_exists() {
        let windows = [
            window(1, Rect::new(0.0, 0.0, 100.0, 100.0)),
            window(2, Rect::new(300.0, 0.0, 100.0, 100.0)),
        ];

        assert_eq!(
            find_window_in_direction(&windows, WindowId::new(1), Direction::Left),
            None
        );
    }
}
