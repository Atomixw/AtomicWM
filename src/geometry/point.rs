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
}
