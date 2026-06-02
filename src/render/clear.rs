use std::fmt;

use crate::geometry::Size;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn from_hex_rgb(value: &str) -> Result<Self, ColorParseError> {
        if !value.starts_with('#') {
            return Err(ColorParseError::MissingPrefix);
        }

        if value.len() != 7 {
            return Err(ColorParseError::InvalidLength);
        }

        let red = parse_channel(&value[1..3])?;
        let green = parse_channel(&value[3..5])?;
        let blue = parse_channel(&value[5..7])?;

        Ok(Self { red, green, blue })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorParseError {
    MissingPrefix,
    InvalidLength,
    InvalidHex,
}

impl fmt::Display for ColorParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingPrefix => write!(formatter, "color must start with #"),
            Self::InvalidLength => write!(formatter, "color must use #RRGGBB format"),
            Self::InvalidHex => write!(formatter, "color contains invalid hex digits"),
        }
    }
}

impl std::error::Error for ColorParseError {}

#[derive(Debug, Clone, PartialEq)]
pub struct OutputState {
    pub name: String,
    pub size: Size,
    pub scale: f64,
}

impl OutputState {
    pub fn new(name: impl Into<String>, size: Size, scale: f64) -> Self {
        Self {
            name: name.into(),
            size,
            scale,
        }
    }

    pub fn default_headless() -> Self {
        Self::new("atomicwm-0", Size::new(1920.0, 1080.0), 1.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClearFrame {
    pub output_size: Size,
    pub background: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClearRenderer {
    background: Color,
}

impl ClearRenderer {
    pub fn new(background: Color) -> Self {
        Self { background }
    }

    pub fn background(&self) -> Color {
        self.background
    }

    pub fn clear_frame(&self, output: &OutputState) -> ClearFrame {
        ClearFrame {
            output_size: output.size,
            background: self.background,
        }
    }
}

fn parse_channel(value: &str) -> Result<f32, ColorParseError> {
    let byte = u8::from_str_radix(value, 16).map_err(|_| ColorParseError::InvalidHex)?;

    Ok(byte as f32 / 255.0)
}

#[cfg(test)]
mod tests {
    use crate::config::Config;

    use super::{ClearRenderer, Color, ColorParseError, OutputState};

    #[test]
    fn parses_black() {
        assert_eq!(
            Color::from_hex_rgb("#000000").unwrap(),
            Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
            }
        );
    }

    #[test]
    fn parses_white() {
        assert_eq!(
            Color::from_hex_rgb("#FFFFFF").unwrap(),
            Color {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
            }
        );
    }

    #[test]
    fn parses_default_focused_border_color() {
        assert_eq!(
            Color::from_hex_rgb("#7C3AED").unwrap(),
            Color {
                red: 124.0 / 255.0,
                green: 58.0 / 255.0,
                blue: 237.0 / 255.0,
            }
        );
    }

    #[test]
    fn rejects_missing_prefix() {
        assert_eq!(
            Color::from_hex_rgb("000000").unwrap_err(),
            ColorParseError::MissingPrefix
        );
    }

    #[test]
    fn rejects_short_strings() {
        assert_eq!(
            Color::from_hex_rgb("#000").unwrap_err(),
            ColorParseError::InvalidLength
        );
    }

    #[test]
    fn rejects_invalid_hex() {
        assert_eq!(
            Color::from_hex_rgb("#GGGGGG").unwrap_err(),
            ColorParseError::InvalidHex
        );
    }

    #[test]
    fn clear_renderer_stores_background_color() {
        let color = Color::from_hex_rgb("#111111").unwrap();
        let renderer = ClearRenderer::new(color);

        assert_eq!(renderer.background(), color);
    }

    #[test]
    fn clear_frame_uses_output_size_and_background() {
        let color = Color::from_hex_rgb("#111111").unwrap();
        let renderer = ClearRenderer::new(color);
        let output = OutputState::default_headless();
        let frame = renderer.clear_frame(&output);

        assert_eq!(frame.output_size, output.size);
        assert_eq!(frame.background, color);
    }

    #[test]
    fn default_background_color_parses() {
        let config = Config::default();

        assert!(Color::from_hex_rgb(&config.appearance.background).is_ok());
    }
}
