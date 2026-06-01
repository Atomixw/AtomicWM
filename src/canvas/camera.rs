use crate::geometry::{Point, Size};

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
            zoom,
            viewport_size,
        }
    }

    pub fn default_for_viewport(viewport_size: Size) -> Self {
        Self::new(Point::new(0.0, 0.0), 1.0, viewport_size)
    }
}
