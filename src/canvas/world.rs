use crate::{
    canvas::Camera,
    config::AppearanceConfig,
    geometry::{Rect, Size, Vector},
    window::{
        Cluster, ClusterGraph, ClusterId, DecorationGeometry, Direction, PlacementMode,
        PlacementRequest, WindowId, WindowNode, compute_decoration_geometry, find_snap_adjustment,
        find_window_in_direction,
    },
};

#[derive(Debug, Default)]
pub struct World {
    windows: Vec<WindowNode>,
}

impl World {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
        }
    }

    pub fn add_window(&mut self, window: WindowNode) {
        self.windows.push(window);
    }

    pub fn add_window_with_placement(
        &mut self,
        id: WindowId,
        title: impl Into<String>,
        app_id: impl Into<String>,
        camera: &Camera,
        request: PlacementRequest,
    ) -> WindowId {
        let rect = self.place_new_window(camera, request);
        let window = WindowNode::new(id, title, app_id, rect);

        self.add_window(window);
        self.focus_window(id);
        id
    }

    pub fn is_empty(&self) -> bool {
        self.windows.is_empty()
    }

    pub fn remove_window(&mut self, id: WindowId) -> Option<WindowNode> {
        let index = self.windows.iter().position(|window| window.id == id)?;

        Some(self.windows.remove(index))
    }

    pub fn window(&self, id: WindowId) -> Option<&WindowNode> {
        self.windows.iter().find(|window| window.id == id)
    }

    pub fn window_mut(&mut self, id: WindowId) -> Option<&mut WindowNode> {
        self.windows.iter_mut().find(|window| window.id == id)
    }

    pub fn windows(&self) -> &[WindowNode] {
        &self.windows
    }

    pub fn focused_window(&self) -> Option<&WindowNode> {
        self.windows.iter().find(|window| window.focused)
    }

    pub fn focused_window_rect(&self) -> Option<Rect> {
        self.focused_window().map(|window| window.rect)
    }

    pub fn focused_window_id(&self) -> Option<WindowId> {
        self.focused_window().map(|window| window.id)
    }

    pub fn focus_first(&mut self) -> Option<WindowId> {
        let id = self.windows.first()?.id;

        self.focus_window(id);
        Some(id)
    }

    pub fn focus_window(&mut self, id: WindowId) -> bool {
        if !self.windows.iter().any(|window| window.id == id) {
            return false;
        }

        for window in &mut self.windows {
            window.focused = window.id == id;
        }

        true
    }

    pub fn focus_in_direction(&mut self, direction: Direction) -> Option<WindowId> {
        let Some(focused_id) = self.focused_window_id() else {
            return self.focus_first();
        };

        let selected = find_window_in_direction(&self.windows, focused_id, direction)?;
        self.focus_window(selected);

        Some(selected)
    }

    pub fn move_window(&mut self, id: WindowId, delta: Vector) -> bool {
        let Some(window) = self.window_mut(id) else {
            return false;
        };

        window.move_by(delta);
        true
    }

    pub fn move_window_with_snapping(
        &mut self,
        id: WindowId,
        delta: Vector,
        threshold: f64,
        gap: f64,
    ) -> bool {
        let Some(window) = self.window(id) else {
            return false;
        };

        let moved = window.rect().translate(delta.dx, delta.dy);
        let targets = self
            .windows
            .iter()
            .filter(|window| window.id != id)
            .map(|window| (&window.id, &window.rect));
        let rect = if let Some(candidate) = find_snap_adjustment(moved, targets, threshold, gap) {
            moved.translate(candidate.adjustment.dx, candidate.adjustment.dy)
        } else {
            moved
        };

        if let Some(window) = self.window_mut(id) {
            window.set_rect(rect);
        }

        true
    }

    pub fn resize_window(&mut self, id: WindowId, size: Size) -> bool {
        let Some(window) = self.window_mut(id) else {
            return false;
        };

        window.resize(size);
        true
    }

    pub fn bounds(&self) -> Option<Rect> {
        let mut windows = self.windows.iter();
        let first = windows.next()?.rect;

        Some(windows.fold(first, |bounds, window| bounds.union(window.rect)))
    }

    pub fn place_new_window(&self, camera: &Camera, request: PlacementRequest) -> Rect {
        debug_assert!(request.validate());

        match request.mode {
            PlacementMode::ViewportCenter => rect_centered_at(camera.position, request.size),
            PlacementMode::NearFocused => {
                let Some(focused) = self.focused_window() else {
                    return rect_centered_at(camera.position, request.size);
                };

                Rect::new(
                    focused.rect.right() + request.gap,
                    focused.rect.top(),
                    request.size.width,
                    request.size.height,
                )
            }
            PlacementMode::AtWorldPosition(point) => Rect::from_origin_size(point, request.size),
        }
    }

    pub fn center_focused_window(&self, camera: &mut Camera) -> bool {
        let Some(window) = self.focused_window() else {
            return false;
        };

        camera.center_on(window.center());
        true
    }

    pub fn fit_all(&self, camera: &mut Camera, padding: f64) -> bool {
        let Some(bounds) = self.bounds() else {
            return false;
        };

        camera.fit_rect(bounds, padding);
        true
    }

    pub fn fit_focused(&self, camera: &mut Camera, padding: f64) -> bool {
        let Some(rect) = self.focused_window_rect() else {
            return false;
        };

        camera.fit_rect(rect, padding);
        true
    }

    pub fn clusters(&self, tolerance: f64) -> Vec<Cluster> {
        ClusterGraph::from_windows(&self.windows, tolerance).clusters(&self.windows)
    }

    pub fn cluster_for_window(&self, id: WindowId, tolerance: f64) -> Option<Cluster> {
        ClusterGraph::from_windows(&self.windows, tolerance).cluster_for_window(id, &self.windows)
    }

    pub fn focused_cluster(&self, tolerance: f64) -> Option<Cluster> {
        self.cluster_for_window(self.focused_window_id()?, tolerance)
    }

    pub fn move_cluster(&mut self, id: ClusterId, delta: Vector, tolerance: f64) -> bool {
        let Some(cluster) = self
            .clusters(tolerance)
            .into_iter()
            .find(|cluster| cluster.id == id)
        else {
            return false;
        };

        for window_id in cluster.windows {
            if let Some(window) = self.window_mut(window_id) {
                window.move_by(delta);
            }
        }

        true
    }

    pub fn fit_focused_cluster(&self, camera: &mut Camera, padding: f64, tolerance: f64) -> bool {
        let Some(cluster) = self.focused_cluster(tolerance) else {
            return false;
        };

        camera.fit_rect(cluster.bounds, padding);
        true
    }

    pub fn window_decoration_geometry(
        &self,
        id: WindowId,
        appearance: &AppearanceConfig,
    ) -> Option<DecorationGeometry> {
        let window = self.window(id)?;

        Some(compute_decoration_geometry(
            window.rect(),
            window.decoration_mode,
            appearance.border_width,
            appearance.titlebar_height,
            appearance.titlebar_button_size,
        ))
    }
}

fn rect_centered_at(center: crate::geometry::Point, size: Size) -> Rect {
    Rect::new(
        center.x - size.width / 2.0,
        center.y - size.height / 2.0,
        size.width,
        size.height,
    )
}

#[cfg(test)]
mod tests {
    use super::World;
    use crate::{
        canvas::Camera,
        geometry::{Point, Rect, Size, Vector},
        window::{
            ClusterId, DecorationMode, Direction, PlacementMode, PlacementRequest, WindowId,
            WindowNode,
        },
    };

    fn window(id: u64, rect: Rect) -> WindowNode {
        WindowNode::new(WindowId::new(id), format!("Window {id}"), "test-app", rect)
    }

    #[test]
    fn adds_and_removes_window() {
        let mut world = World::new();
        let id = WindowId::new(1);

        world.add_window(window(1, Rect::new(0.0, 0.0, 100.0, 100.0)));

        assert_eq!(world.windows().len(), 1);
        assert!(world.window(id).is_some());
        assert_eq!(world.remove_window(id).map(|window| window.id), Some(id));
        assert!(world.is_empty());
    }

    #[test]
    fn focuses_only_one_window() {
        let mut world = World::new();
        let first = WindowId::new(1);
        let second = WindowId::new(2);

        world.add_window(window(1, Rect::new(0.0, 0.0, 100.0, 100.0)));
        world.add_window(window(2, Rect::new(200.0, 0.0, 100.0, 100.0)));

        assert!(world.focus_window(first));
        assert_eq!(world.focused_window().map(|window| window.id), Some(first));

        assert!(world.focus_window(second));
        assert_eq!(world.focused_window().map(|window| window.id), Some(second));
        assert!(!world.window(first).unwrap().focused);
        assert!(!world.focus_window(WindowId::new(99)));
    }

    #[test]
    fn focus_first_returns_none_for_empty_world() {
        assert_eq!(World::new().focus_first(), None);
    }

    #[test]
    fn focus_first_focuses_first_window() {
        let mut world = World::new();
        world.add_window(window(1, Rect::new(0.0, 0.0, 100.0, 100.0)));
        world.add_window(window(2, Rect::new(200.0, 0.0, 100.0, 100.0)));

        assert_eq!(world.focus_first(), Some(WindowId::new(1)));
        assert_eq!(world.focused_window_id(), Some(WindowId::new(1)));
    }

    #[test]
    fn focus_in_direction_updates_focused_window_id() {
        let mut world = World::new();
        world.add_window(window(1, Rect::new(0.0, 0.0, 100.0, 100.0)));
        world.add_window(window(2, Rect::new(200.0, 0.0, 100.0, 100.0)));
        world.focus_window(WindowId::new(1));

        assert_eq!(
            world.focus_in_direction(Direction::Right),
            Some(WindowId::new(2))
        );
        assert_eq!(world.focused_window_id(), Some(WindowId::new(2)));
    }

    #[test]
    fn focus_in_direction_does_not_change_focus_without_candidate() {
        let mut world = World::new();
        world.add_window(window(1, Rect::new(0.0, 0.0, 100.0, 100.0)));
        world.add_window(window(2, Rect::new(200.0, 0.0, 100.0, 100.0)));
        world.focus_window(WindowId::new(1));

        assert_eq!(world.focus_in_direction(Direction::Left), None);
        assert_eq!(world.focused_window_id(), Some(WindowId::new(1)));
    }

    #[test]
    fn only_one_window_is_focused_after_navigation() {
        let mut world = World::new();
        world.add_window(window(1, Rect::new(0.0, 0.0, 100.0, 100.0)));
        world.add_window(window(2, Rect::new(200.0, 0.0, 100.0, 100.0)));
        world.add_window(window(3, Rect::new(400.0, 0.0, 100.0, 100.0)));
        world.focus_window(WindowId::new(1));

        world.focus_in_direction(Direction::Right);

        assert_eq!(
            world
                .windows()
                .iter()
                .filter(|window| window.focused)
                .count(),
            1
        );
    }

    #[test]
    fn moves_window() {
        let mut world = World::new();
        let id = WindowId::new(1);

        world.add_window(window(1, Rect::new(0.0, 0.0, 100.0, 100.0)));

        assert!(world.move_window(id, Vector::new(10.0, 20.0)));
        assert_eq!(
            world.window(id).unwrap().rect,
            Rect::new(10.0, 20.0, 100.0, 100.0)
        );
        assert!(!world.move_window(WindowId::new(99), Vector::zero()));
    }

    #[test]
    fn move_window_with_snapping_moves_normally_without_snap() {
        let mut world = World::new();
        let id = WindowId::new(1);

        world.add_window(window(1, Rect::new(0.0, 0.0, 100.0, 100.0)));
        world.add_window(window(2, Rect::new(400.0, 0.0, 100.0, 100.0)));

        assert!(world.move_window_with_snapping(id, Vector::new(50.0, 0.0), 10.0, 0.0));
        assert_eq!(
            world.window(id).unwrap().rect(),
            Rect::new(50.0, 0.0, 100.0, 100.0)
        );
    }

    #[test]
    fn move_window_with_snapping_snaps_when_close_enough() {
        let mut world = World::new();
        let id = WindowId::new(1);

        world.add_window(window(1, Rect::new(0.0, 0.0, 100.0, 100.0)));
        world.add_window(window(2, Rect::new(210.0, 0.0, 100.0, 100.0)));

        assert!(world.move_window_with_snapping(id, Vector::new(105.0, 0.0), 10.0, 0.0));
        assert_eq!(
            world.window(id).unwrap().rect(),
            Rect::new(110.0, 0.0, 100.0, 100.0)
        );
    }

    #[test]
    fn move_window_with_snapping_returns_false_for_missing_window() {
        let mut world = World::new();

        assert!(!world.move_window_with_snapping(WindowId::new(99), Vector::zero(), 10.0, 0.0));
    }

    #[test]
    fn snapping_does_not_move_target_window() {
        let mut world = World::new();

        world.add_window(window(1, Rect::new(0.0, 0.0, 100.0, 100.0)));
        world.add_window(window(2, Rect::new(210.0, 0.0, 100.0, 100.0)));

        world.move_window_with_snapping(WindowId::new(1), Vector::new(105.0, 0.0), 10.0, 0.0);

        assert_eq!(
            world.window(WindowId::new(2)).unwrap().rect(),
            Rect::new(210.0, 0.0, 100.0, 100.0)
        );
    }

    #[test]
    fn snapping_ignores_the_moving_window_itself() {
        let mut world = World::new();
        let id = WindowId::new(1);

        world.add_window(window(1, Rect::new(0.0, 0.0, 100.0, 100.0)));

        assert!(world.move_window_with_snapping(id, Vector::new(5.0, 0.0), 10.0, 0.0));
        assert_eq!(
            world.window(id).unwrap().rect(),
            Rect::new(5.0, 0.0, 100.0, 100.0)
        );
    }

    #[test]
    fn viewport_center_places_window_at_camera_center() {
        let world = World::new();
        let camera = Camera::new(Point::new(100.0, 200.0), 1.0, Size::new(800.0, 600.0));
        let request =
            PlacementRequest::new(Size::new(300.0, 200.0), PlacementMode::ViewportCenter, 8.0);

        assert_eq!(
            world.place_new_window(&camera, request),
            Rect::new(-50.0, 100.0, 300.0, 200.0)
        );
    }

    #[test]
    fn near_focused_places_window_to_the_right_of_focused_window() {
        let mut world = World::new();
        let camera = Camera::default_for_viewport(Size::new(800.0, 600.0));
        let request =
            PlacementRequest::new(Size::new(300.0, 200.0), PlacementMode::NearFocused, 12.0);

        world.add_window(window(1, Rect::new(10.0, 20.0, 100.0, 80.0)));
        world.focus_window(WindowId::new(1));

        assert_eq!(
            world.place_new_window(&camera, request),
            Rect::new(122.0, 20.0, 300.0, 200.0)
        );
    }

    #[test]
    fn near_focused_falls_back_to_viewport_center_without_focused_window() {
        let world = World::new();
        let camera = Camera::new(Point::new(100.0, 200.0), 1.0, Size::new(800.0, 600.0));
        let request =
            PlacementRequest::new(Size::new(300.0, 200.0), PlacementMode::NearFocused, 12.0);

        assert_eq!(
            world.place_new_window(&camera, request),
            Rect::new(-50.0, 100.0, 300.0, 200.0)
        );
    }

    #[test]
    fn at_world_position_uses_given_point() {
        let world = World::new();
        let camera = Camera::default_for_viewport(Size::new(800.0, 600.0));
        let request = PlacementRequest::new(
            Size::new(300.0, 200.0),
            PlacementMode::AtWorldPosition(Point::new(20.0, 30.0)),
            8.0,
        );

        assert_eq!(
            world.place_new_window(&camera, request),
            Rect::new(20.0, 30.0, 300.0, 200.0)
        );
    }

    #[test]
    fn add_window_with_placement_inserts_and_focuses_new_window() {
        let mut world = World::new();
        let camera = Camera::default_for_viewport(Size::new(800.0, 600.0));
        let request =
            PlacementRequest::new(Size::new(300.0, 200.0), PlacementMode::ViewportCenter, 8.0);

        let id = world.add_window_with_placement(
            WindowId::new(1),
            "Terminal",
            "alacritty",
            &camera,
            request,
        );

        assert_eq!(id, WindowId::new(1));
        assert_eq!(world.windows().len(), 1);
        assert_eq!(world.focused_window_id(), Some(WindowId::new(1)));
    }

    #[test]
    fn center_focused_window_centers_camera_on_focus() {
        let mut world = World::new();
        let mut camera = Camera::default_for_viewport(Size::new(800.0, 600.0));
        world.add_window(window(1, Rect::new(100.0, 200.0, 300.0, 200.0)));
        world.focus_window(WindowId::new(1));

        assert!(world.center_focused_window(&mut camera));
        assert_eq!(camera.position, Point::new(250.0, 300.0));
    }

    #[test]
    fn fit_all_returns_false_for_empty_world() {
        let world = World::new();
        let mut camera = Camera::default_for_viewport(Size::new(800.0, 600.0));

        assert!(!world.fit_all(&mut camera, 8.0));
    }

    #[test]
    fn fit_all_fits_bounds_when_windows_exist() {
        let mut world = World::new();
        let mut camera = Camera::default_for_viewport(Size::new(800.0, 600.0));

        world.add_window(window(1, Rect::new(0.0, 0.0, 100.0, 100.0)));
        world.add_window(window(2, Rect::new(200.0, 100.0, 100.0, 100.0)));

        assert!(world.fit_all(&mut camera, 8.0));
        assert!(
            camera
                .viewport_rect_world()
                .contains_rect(world.bounds().unwrap())
        );
    }

    #[test]
    fn fit_focused_returns_false_without_focused_window() {
        let world = World::new();
        let mut camera = Camera::default_for_viewport(Size::new(800.0, 600.0));

        assert!(!world.fit_focused(&mut camera, 8.0));
    }

    #[test]
    fn fit_focused_fits_focused_window() {
        let mut world = World::new();
        let mut camera = Camera::default_for_viewport(Size::new(800.0, 600.0));

        world.add_window(window(1, Rect::new(100.0, 200.0, 300.0, 200.0)));
        world.focus_window(WindowId::new(1));

        assert!(world.fit_focused(&mut camera, 8.0));
        assert!(
            camera
                .viewport_rect_world()
                .contains_rect(world.focused_window_rect().unwrap())
        );
    }

    #[test]
    fn resizes_window() {
        let mut world = World::new();
        let id = WindowId::new(1);

        world.add_window(window(1, Rect::new(0.0, 0.0, 100.0, 100.0)));

        assert!(world.resize_window(id, Size::new(320.0, 240.0)));
        assert_eq!(
            world.window(id).unwrap().rect,
            Rect::new(0.0, 0.0, 320.0, 240.0)
        );
        assert!(!world.resize_window(WindowId::new(99), Size::new(1.0, 1.0)));
    }

    #[test]
    fn returns_bounds_of_multiple_windows() {
        let mut world = World::new();

        world.add_window(window(1, Rect::new(10.0, 10.0, 100.0, 100.0)));
        world.add_window(window(2, Rect::new(-50.0, 40.0, 25.0, 200.0)));

        assert_eq!(world.bounds(), Some(Rect::new(-50.0, 10.0, 160.0, 230.0)));
    }

    #[test]
    fn empty_world_has_no_bounds() {
        assert_eq!(World::new().bounds(), None);
    }

    #[test]
    fn no_windows_returns_none_for_directional_focus() {
        assert_eq!(World::new().focus_in_direction(Direction::Right), None);
    }

    #[test]
    fn clusters_return_stable_ordering() {
        let mut world = World::new();
        world.add_window(window(1, Rect::new(300.0, 0.0, 100.0, 100.0)));
        world.add_window(window(2, Rect::new(400.0, 0.0, 100.0, 100.0)));
        world.add_window(window(3, Rect::new(0.0, 0.0, 100.0, 100.0)));

        let clusters = world.clusters(1.0);

        assert_eq!(clusters.len(), 2);
        assert_eq!(
            clusters[0].windows,
            vec![WindowId::new(1), WindowId::new(2)]
        );
        assert_eq!(clusters[1].windows, vec![WindowId::new(3)]);
    }

    #[test]
    fn cluster_for_window_returns_expected_cluster() {
        let mut world = World::new();
        world.add_window(window(1, Rect::new(0.0, 0.0, 100.0, 100.0)));
        world.add_window(window(2, Rect::new(100.0, 0.0, 100.0, 100.0)));

        let cluster = world.cluster_for_window(WindowId::new(2), 1.0).unwrap();

        assert_eq!(cluster.windows, vec![WindowId::new(1), WindowId::new(2)]);
    }

    #[test]
    fn focused_cluster_returns_cluster_containing_focused_window() {
        let mut world = World::new();
        world.add_window(window(1, Rect::new(0.0, 0.0, 100.0, 100.0)));
        world.add_window(window(2, Rect::new(100.0, 0.0, 100.0, 100.0)));
        world.focus_window(WindowId::new(2));

        let cluster = world.focused_cluster(1.0).unwrap();

        assert_eq!(cluster.windows, vec![WindowId::new(1), WindowId::new(2)]);
    }

    #[test]
    fn move_cluster_moves_all_windows_in_cluster() {
        let mut world = World::new();
        world.add_window(window(1, Rect::new(0.0, 0.0, 100.0, 100.0)));
        world.add_window(window(2, Rect::new(100.0, 0.0, 100.0, 100.0)));

        assert!(world.move_cluster(ClusterId::new(1), Vector::new(20.0, 10.0), 1.0));
        assert_eq!(
            world.window(WindowId::new(1)).unwrap().rect(),
            Rect::new(20.0, 10.0, 100.0, 100.0)
        );
        assert_eq!(
            world.window(WindowId::new(2)).unwrap().rect(),
            Rect::new(120.0, 10.0, 100.0, 100.0)
        );
    }

    #[test]
    fn move_cluster_does_not_move_windows_outside_cluster() {
        let mut world = World::new();
        world.add_window(window(1, Rect::new(0.0, 0.0, 100.0, 100.0)));
        world.add_window(window(2, Rect::new(100.0, 0.0, 100.0, 100.0)));
        world.add_window(window(3, Rect::new(400.0, 0.0, 100.0, 100.0)));

        world.move_cluster(ClusterId::new(1), Vector::new(20.0, 0.0), 1.0);

        assert_eq!(
            world.window(WindowId::new(3)).unwrap().rect(),
            Rect::new(400.0, 0.0, 100.0, 100.0)
        );
    }

    #[test]
    fn fit_focused_cluster_returns_false_without_focused_window() {
        let world = World::new();
        let mut camera = Camera::default_for_viewport(Size::new(800.0, 600.0));

        assert!(!world.fit_focused_cluster(&mut camera, 8.0, 1.0));
    }

    #[test]
    fn fit_focused_cluster_fits_cluster_bounds() {
        let mut world = World::new();
        let mut camera = Camera::default_for_viewport(Size::new(800.0, 600.0));
        world.add_window(window(1, Rect::new(0.0, 0.0, 100.0, 100.0)));
        world.add_window(window(2, Rect::new(100.0, 0.0, 100.0, 100.0)));
        world.focus_window(WindowId::new(1));

        assert!(world.fit_focused_cluster(&mut camera, 8.0, 1.0));
        assert!(
            camera
                .viewport_rect_world()
                .contains_rect(world.focused_cluster(1.0).unwrap().bounds)
        );
    }

    #[test]
    fn focused_window_can_produce_decoration_geometry() {
        let mut world = World::new();
        let mut window = window(1, Rect::new(10.0, 20.0, 300.0, 200.0));
        window.decoration_mode = DecorationMode::Titlebar;
        world.add_window(window);
        world.focus_window(WindowId::new(1));

        let geometry = world
            .window_decoration_geometry(
                world.focused_window_id().unwrap(),
                &crate::config::AppearanceConfig::default(),
            )
            .unwrap();

        assert_eq!(geometry.content_rect, Rect::new(10.0, 20.0, 300.0, 200.0));
        assert!(geometry.titlebar_rect.is_some());
    }

    #[test]
    fn missing_window_returns_no_decoration_geometry() {
        let world = World::new();

        assert!(
            world
                .window_decoration_geometry(
                    WindowId::new(99),
                    &crate::config::AppearanceConfig::default()
                )
                .is_none()
        );
    }
}
