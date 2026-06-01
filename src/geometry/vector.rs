#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    pub dx: f64,
    pub dy: f64,
}

impl Vector {
    pub fn new(dx: f64, dy: f64) -> Self {
        Self { dx, dy }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn length(self) -> f64 {
        (self.dx * self.dx + self.dy * self.dy).sqrt()
    }

    pub fn scale(self, factor: f64) -> Self {
        Self::new(self.dx * factor, self.dy * factor)
    }
}

#[cfg(test)]
mod tests {
    use super::Vector;

    #[test]
    fn returns_vector_length() {
        let vector = Vector::new(3.0, 4.0);

        assert_eq!(vector.length(), 5.0);
    }

    #[test]
    fn scales_vector() {
        let vector = Vector::new(3.0, 4.0).scale(2.0);

        assert_eq!(vector, Vector::new(6.0, 8.0));
    }
}
