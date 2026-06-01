use crate::geometry::{Point, Rect, Size, Vector};

const DEFAULT_ZOOM: f64 = 1.0;
const MIN_ZOOM: f64 = 0.01;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    pub position: Point,
    pub zoom: f64,
    pub viewport_size: Size,
}

impl Camera {
    pub fn new(position: Point, zoom: f64, viewport_size: Size) -> Self {
        Self {
            position,
            zoom: clamp_zoom(zoom),
            viewport_size,
        }
    }

    pub fn default_for_viewport(viewport_size: Size) -> Self {
        Self::new(Point::new(0.0, 0.0), DEFAULT_ZOOM, viewport_size)
    }

    pub fn viewport_rect_world(self) -> Rect {
        let width = self.viewport_size.width / self.zoom;
        let height = self.viewport_size.height / self.zoom;

        Rect::new(
            self.position.x - width / 2.0,
            self.position.y - height / 2.0,
            width,
            height,
        )
    }

    pub fn world_to_screen(self, point: Point) -> Point {
        Point::new(
            self.viewport_size.width / 2.0 + (point.x - self.position.x) * self.zoom,
            self.viewport_size.height / 2.0 + (point.y - self.position.y) * self.zoom,
        )
    }

    pub fn screen_to_world(self, point: Point) -> Point {
        Point::new(
            self.position.x + (point.x - self.viewport_size.width / 2.0) / self.zoom,
            self.position.y + (point.y - self.viewport_size.height / 2.0) / self.zoom,
        )
    }

    pub fn pan(&mut self, delta: Vector) {
        self.position = self.position.offset(delta);
    }

    pub fn set_zoom(&mut self, zoom: f64) {
        self.zoom = clamp_zoom(zoom);
    }

    pub fn zoom_at(&mut self, screen_point: Point, zoom_factor: f64) {
        let world_before = self.screen_to_world(screen_point);
        self.set_zoom(self.zoom * zoom_factor);
        let world_after = self.screen_to_world(screen_point);

        self.position = self.position.offset(world_after.vector_to(world_before));
    }

    pub fn center_on(&mut self, world_point: Point) {
        self.position = world_point;
    }

    pub fn fit_rect(&mut self, rect: Rect, padding: f64) {
        self.center_on(rect.center());

        let padding = padding.max(0.0);
        let width = rect.width() + padding * 2.0;
        let height = rect.height() + padding * 2.0;

        if width <= 0.0 || height <= 0.0 || self.viewport_size.is_empty() {
            self.reset_zoom();
            return;
        }

        self.set_zoom((self.viewport_size.width / width).min(self.viewport_size.height / height));
    }

    pub fn reset_zoom(&mut self) {
        self.zoom = DEFAULT_ZOOM;
    }
}

fn clamp_zoom(zoom: f64) -> f64 {
    if zoom.is_finite() && zoom >= MIN_ZOOM {
        zoom
    } else {
        MIN_ZOOM
    }
}

#[cfg(test)]
mod tests {
    use super::{Camera, MIN_ZOOM};
    use crate::geometry::{Point, Rect, Size, Vector};

    fn assert_close(left: f64, right: f64) {
        assert!((left - right).abs() < 0.000_001, "{left} != {right}");
    }

    fn assert_point_close(left: Point, right: Point) {
        assert_close(left.x, right.x);
        assert_close(left.y, right.y);
    }

    #[test]
    fn maps_camera_center_to_viewport_center() {
        let camera = Camera::new(Point::new(100.0, 50.0), 2.0, Size::new(800.0, 600.0));

        assert_eq!(
            camera.world_to_screen(camera.position),
            Point::new(400.0, 300.0)
        );
    }

    #[test]
    fn screen_to_world_reverses_world_to_screen() {
        let camera = Camera::new(Point::new(100.0, 50.0), 2.0, Size::new(800.0, 600.0));
        let world = Point::new(150.0, 25.0);

        assert_point_close(camera.screen_to_world(camera.world_to_screen(world)), world);
    }

    #[test]
    fn pan_changes_camera_position_in_world_units() {
        let mut camera = Camera::default_for_viewport(Size::new(800.0, 600.0));

        camera.pan(Vector::new(12.0, -8.0));

        assert_eq!(camera.position, Point::new(12.0, -8.0));
    }

    #[test]
    fn zoom_at_keeps_world_point_under_cursor_stable() {
        let mut camera = Camera::default_for_viewport(Size::new(800.0, 600.0));
        let cursor = Point::new(600.0, 450.0);
        let before = camera.screen_to_world(cursor);

        camera.zoom_at(cursor, 2.0);

        assert_point_close(camera.screen_to_world(cursor), before);
        assert_eq!(camera.zoom, 2.0);
    }

    #[test]
    fn fit_rect_centers_on_rect() {
        let mut camera = Camera::default_for_viewport(Size::new(800.0, 600.0));
        let rect = Rect::new(100.0, 200.0, 400.0, 100.0);

        camera.fit_rect(rect, 0.0);

        assert_eq!(camera.position, rect.center());
    }

    #[test]
    fn fit_rect_chooses_zoom_that_fits_rect() {
        let mut camera = Camera::default_for_viewport(Size::new(800.0, 600.0));
        let rect = Rect::new(100.0, 200.0, 400.0, 100.0);

        camera.fit_rect(rect, 50.0);

        assert_close(camera.zoom, 1.6);
        assert!(camera.viewport_rect_world().contains_rect(rect));
    }

    #[test]
    fn clamps_invalid_zoom_values() {
        let mut camera = Camera::new(Point::new(0.0, 0.0), f64::NAN, Size::new(800.0, 600.0));
        assert_eq!(camera.zoom, MIN_ZOOM);

        camera.set_zoom(-1.0);
        assert_eq!(camera.zoom, MIN_ZOOM);

        camera.set_zoom(f64::INFINITY);
        assert_eq!(camera.zoom, MIN_ZOOM);
    }
}
