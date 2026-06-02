use crate::geometry::{Point, Size};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlacementMode {
    ViewportCenter,
    NearFocused,
    AtWorldPosition(Point),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlacementRequest {
    pub size: Size,
    pub mode: PlacementMode,
    pub gap: f64,
}

impl PlacementRequest {
    pub fn new(size: Size, mode: PlacementMode, gap: f64) -> Self {
        Self { size, mode, gap }
    }

    pub fn validate(self) -> bool {
        self.size.width.is_finite()
            && self.size.width > 0.0
            && self.size.height.is_finite()
            && self.size.height > 0.0
            && self.gap.is_finite()
            && self.gap >= 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::{PlacementMode, PlacementRequest};
    use crate::geometry::Size;

    #[test]
    fn validates_request_values() {
        assert!(
            PlacementRequest::new(Size::new(100.0, 100.0), PlacementMode::ViewportCenter, 8.0,)
                .validate()
        );

        assert!(
            !PlacementRequest::new(Size::new(0.0, 100.0), PlacementMode::ViewportCenter, 8.0)
                .validate()
        );
        assert!(
            !PlacementRequest::new(Size::new(100.0, -1.0), PlacementMode::ViewportCenter, 8.0)
                .validate()
        );
        assert!(
            !PlacementRequest::new(Size::new(100.0, 100.0), PlacementMode::ViewportCenter, -1.0)
                .validate()
        );
    }
}
