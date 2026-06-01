use super::{Point, Size};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Rect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            origin: Point::new(x, y),
            size: Size::new(width, height),
        }
    }

    pub fn left(self) -> f64 {
        self.origin.x
    }

    pub fn right(self) -> f64 {
        self.origin.x + self.size.width
    }

    pub fn top(self) -> f64 {
        self.origin.y
    }

    pub fn bottom(self) -> f64 {
        self.origin.y + self.size.height
    }

    pub fn center(self) -> Point {
        Point::new(
            self.origin.x + self.size.width / 2.0,
            self.origin.y + self.size.height / 2.0,
        )
    }

    pub fn contains(self, point: Point) -> bool {
        point.x >= self.left()
            && point.x <= self.right()
            && point.y >= self.top()
            && point.y <= self.bottom()
    }

    pub fn translate(self, dx: f64, dy: f64) -> Self {
        Self {
            origin: self.origin.translate(dx, dy),
            size: self.size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Point, Rect};

    #[test]
    fn creates_rect() {
        let rect = Rect::new(10.0, 20.0, 300.0, 200.0);

        assert_eq!(rect.origin, Point::new(10.0, 20.0));
        assert_eq!(rect.size.width, 300.0);
        assert_eq!(rect.size.height, 200.0);
    }

    #[test]
    fn returns_edges() {
        let rect = Rect::new(10.0, 20.0, 300.0, 200.0);

        assert_eq!(rect.left(), 10.0);
        assert_eq!(rect.right(), 310.0);
        assert_eq!(rect.top(), 20.0);
        assert_eq!(rect.bottom(), 220.0);
    }

    #[test]
    fn returns_center() {
        let rect = Rect::new(10.0, 20.0, 300.0, 200.0);

        assert_eq!(rect.center(), Point::new(160.0, 120.0));
    }

    #[test]
    fn checks_containment() {
        let rect = Rect::new(10.0, 20.0, 300.0, 200.0);

        assert!(rect.contains(Point::new(10.0, 20.0)));
        assert!(rect.contains(Point::new(310.0, 220.0)));
        assert!(rect.contains(Point::new(160.0, 120.0)));
        assert!(!rect.contains(Point::new(9.0, 120.0)));
        assert!(!rect.contains(Point::new(160.0, 221.0)));
    }

    #[test]
    fn translates_rect() {
        let rect = Rect::new(10.0, 20.0, 300.0, 200.0).translate(5.0, -8.0);

        assert_eq!(rect, Rect::new(15.0, 12.0, 300.0, 200.0));
    }
}
