#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
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
}
