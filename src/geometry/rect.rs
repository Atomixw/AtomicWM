use super::{Point, Size};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Rect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self::from_origin_size(Point::new(x, y), Size::new(width, height))
    }

    pub fn from_origin_size(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }

    pub fn width(self) -> f64 {
        self.size.width
    }

    pub fn height(self) -> f64 {
        self.size.height
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

    pub fn top_left(self) -> Point {
        self.origin
    }

    pub fn top_right(self) -> Point {
        Point::new(self.right(), self.top())
    }

    pub fn bottom_left(self) -> Point {
        Point::new(self.left(), self.bottom())
    }

    pub fn bottom_right(self) -> Point {
        Point::new(self.right(), self.bottom())
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

    pub fn intersects(self, other: Self) -> bool {
        self.left() <= other.right()
            && self.right() >= other.left()
            && self.top() <= other.bottom()
            && self.bottom() >= other.top()
    }

    pub fn intersection(self, other: Self) -> Option<Self> {
        if !self.intersects(other) {
            return None;
        }

        let left = self.left().max(other.left());
        let top = self.top().max(other.top());
        let right = self.right().min(other.right());
        let bottom = self.bottom().min(other.bottom());

        Some(Self::new(left, top, right - left, bottom - top))
    }

    pub fn union(self, other: Self) -> Self {
        let left = self.left().min(other.left());
        let top = self.top().min(other.top());
        let right = self.right().max(other.right());
        let bottom = self.bottom().max(other.bottom());

        Self::new(left, top, right - left, bottom - top)
    }

    pub fn inflate(self, amount: f64) -> Self {
        Self::new(
            self.left() - amount,
            self.top() - amount,
            self.width() + amount * 2.0,
            self.height() + amount * 2.0,
        )
    }

    pub fn contains_rect(self, other: Self) -> bool {
        other.left() >= self.left()
            && other.right() <= self.right()
            && other.top() >= self.top()
            && other.bottom() <= self.bottom()
    }

    pub fn distance_to_point(self, point: Point) -> f64 {
        let dx = if point.x < self.left() {
            self.left() - point.x
        } else if point.x > self.right() {
            point.x - self.right()
        } else {
            0.0
        };

        let dy = if point.y < self.top() {
            self.top() - point.y
        } else if point.y > self.bottom() {
            point.y - self.bottom()
        } else {
            0.0
        };

        (dx * dx + dy * dy).sqrt()
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
        assert_eq!(rect.width(), 300.0);
        assert_eq!(rect.height(), 200.0);
        assert_eq!(rect.top_left(), Point::new(10.0, 20.0));
        assert_eq!(rect.top_right(), Point::new(310.0, 20.0));
        assert_eq!(rect.bottom_left(), Point::new(10.0, 220.0));
        assert_eq!(rect.bottom_right(), Point::new(310.0, 220.0));
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
    fn checks_rect_containment() {
        let rect = Rect::new(10.0, 20.0, 300.0, 200.0);

        assert!(rect.contains_rect(Rect::new(20.0, 30.0, 100.0, 80.0)));
        assert!(!rect.contains_rect(Rect::new(0.0, 30.0, 100.0, 80.0)));
    }

    #[test]
    fn translates_rect() {
        let rect = Rect::new(10.0, 20.0, 300.0, 200.0).translate(5.0, -8.0);

        assert_eq!(rect, Rect::new(15.0, 12.0, 300.0, 200.0));
    }

    #[test]
    fn intersects_rects() {
        let rect = Rect::new(0.0, 0.0, 100.0, 100.0);

        assert!(rect.intersects(Rect::new(50.0, 50.0, 100.0, 100.0)));
        assert!(!rect.intersects(Rect::new(101.0, 0.0, 100.0, 100.0)));
    }

    #[test]
    fn returns_intersection() {
        let intersection =
            Rect::new(0.0, 0.0, 100.0, 100.0).intersection(Rect::new(50.0, 60.0, 100.0, 100.0));

        assert_eq!(intersection, Some(Rect::new(50.0, 60.0, 50.0, 40.0)));
    }

    #[test]
    fn returns_union() {
        let union = Rect::new(0.0, 0.0, 100.0, 100.0).union(Rect::new(50.0, 60.0, 100.0, 100.0));

        assert_eq!(union, Rect::new(0.0, 0.0, 150.0, 160.0));
    }

    #[test]
    fn inflates_rect() {
        let rect = Rect::new(10.0, 20.0, 100.0, 50.0).inflate(5.0);

        assert_eq!(rect, Rect::new(5.0, 15.0, 110.0, 60.0));
    }

    #[test]
    fn returns_distance_to_point() {
        let rect = Rect::new(10.0, 20.0, 100.0, 50.0);

        assert_eq!(rect.distance_to_point(Point::new(20.0, 30.0)), 0.0);
        assert_eq!(rect.distance_to_point(Point::new(0.0, 30.0)), 10.0);
        assert_eq!(
            rect.distance_to_point(Point::new(0.0, 10.0)),
            200.0_f64.sqrt()
        );
    }
}
