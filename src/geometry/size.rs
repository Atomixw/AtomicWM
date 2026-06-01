#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    pub fn is_empty(self) -> bool {
        self.width <= 0.0 || self.height <= 0.0
    }

    pub fn scale(self, factor: f64) -> Self {
        Self::new(self.width * factor, self.height * factor)
    }
}

#[cfg(test)]
mod tests {
    use super::Size;

    #[test]
    fn creates_size() {
        let size = Size::new(800.0, 600.0);

        assert_eq!(size.width, 800.0);
        assert_eq!(size.height, 600.0);
    }

    #[test]
    fn detects_empty_size() {
        assert!(Size::new(0.0, 600.0).is_empty());
        assert!(Size::new(800.0, -1.0).is_empty());
        assert!(!Size::new(800.0, 600.0).is_empty());
    }

    #[test]
    fn scales_size() {
        let size = Size::new(800.0, 600.0).scale(0.5);

        assert_eq!(size, Size::new(400.0, 300.0));
    }
}
