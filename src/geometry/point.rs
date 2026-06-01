use super::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn translate(self, dx: f64, dy: f64) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    pub fn distance_to(self, other: Self) -> f64 {
        self.vector_to(other).length()
    }

    pub fn offset(self, vector: Vector) -> Self {
        self.translate(vector.dx, vector.dy)
    }

    pub fn vector_to(self, other: Self) -> Vector {
        Vector::new(other.x - self.x, other.y - self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn creates_point() {
        let point = Point::new(10.0, 20.0);

        assert_eq!(point.x, 10.0);
        assert_eq!(point.y, 20.0);
    }

    #[test]
    fn translates_point() {
        let point = Point::new(10.0, 20.0).translate(5.0, -8.0);

        assert_eq!(point, Point::new(15.0, 12.0));
    }

    #[test]
    fn offsets_point_by_vector() {
        let point = Point::new(10.0, 20.0).offset(super::Vector::new(5.0, -8.0));

        assert_eq!(point, Point::new(15.0, 12.0));
    }

    #[test]
    fn returns_distance_to_other_point() {
        let distance = Point::new(0.0, 0.0).distance_to(Point::new(3.0, 4.0));

        assert_eq!(distance, 5.0);
    }
}
