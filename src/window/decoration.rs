use serde::Deserialize;

use crate::geometry::{Point, Rect};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DecorationMode {
    None,
    Border,
    Titlebar,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BorderRects {
    pub top: Rect,
    pub right: Rect,
    pub bottom: Rect,
    pub left: Rect,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DecorationGeometry {
    pub outer_rect: Rect,
    pub content_rect: Rect,
    pub border_rects: BorderRects,
    pub titlebar_rect: Option<Rect>,
    pub close_button_rect: Option<Rect>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecorationHit {
    None,
    Content,
    BorderTop,
    BorderRight,
    BorderBottom,
    BorderLeft,
    Titlebar,
    CloseButton,
}

impl DecorationGeometry {
    pub fn hit_test(self, point: Point) -> DecorationHit {
        if !self.outer_rect.contains(point) {
            return DecorationHit::None;
        }

        if self
            .close_button_rect
            .is_some_and(|rect| rect.contains(point))
        {
            return DecorationHit::CloseButton;
        }

        if self.titlebar_rect.is_some_and(|rect| rect.contains(point)) {
            return DecorationHit::Titlebar;
        }

        if self.border_rects.top.contains(point) {
            return DecorationHit::BorderTop;
        }

        if self.border_rects.right.contains(point) {
            return DecorationHit::BorderRight;
        }

        if self.border_rects.bottom.contains(point) {
            return DecorationHit::BorderBottom;
        }

        if self.border_rects.left.contains(point) {
            return DecorationHit::BorderLeft;
        }

        if self.content_rect.contains(point) {
            DecorationHit::Content
        } else {
            DecorationHit::None
        }
    }
}

pub fn compute_decoration_geometry(
    content_rect: Rect,
    mode: DecorationMode,
    border_width: f64,
    titlebar_height: f64,
    titlebar_button_size: f64,
) -> DecorationGeometry {
    let border_width = border_width.max(0.0);
    let titlebar_height = titlebar_height.max(0.0);
    let titlebar_button_size = titlebar_button_size.max(0.0);

    match mode {
        DecorationMode::None => DecorationGeometry {
            outer_rect: content_rect,
            content_rect,
            border_rects: empty_borders(content_rect),
            titlebar_rect: None,
            close_button_rect: None,
        },
        DecorationMode::Border => {
            let outer_rect = content_rect.inflate(border_width);
            DecorationGeometry {
                outer_rect,
                content_rect,
                border_rects: border_rects(outer_rect, border_width),
                titlebar_rect: None,
                close_button_rect: None,
            }
        }
        DecorationMode::Titlebar => {
            let outer_rect = Rect::new(
                content_rect.left() - border_width,
                content_rect.top() - titlebar_height - border_width,
                content_rect.width() + border_width * 2.0,
                content_rect.height() + titlebar_height + border_width * 2.0,
            );
            let titlebar_rect = Rect::new(
                content_rect.left(),
                content_rect.top() - titlebar_height,
                content_rect.width(),
                titlebar_height,
            );
            let close_button_rect = close_button_rect(titlebar_rect, titlebar_button_size);

            DecorationGeometry {
                outer_rect,
                content_rect,
                border_rects: border_rects(outer_rect, border_width),
                titlebar_rect: Some(titlebar_rect),
                close_button_rect,
            }
        }
    }
}

fn border_rects(outer_rect: Rect, border_width: f64) -> BorderRects {
    BorderRects {
        top: Rect::new(
            outer_rect.left(),
            outer_rect.top(),
            outer_rect.width(),
            border_width,
        ),
        right: Rect::new(
            outer_rect.right() - border_width,
            outer_rect.top(),
            border_width,
            outer_rect.height(),
        ),
        bottom: Rect::new(
            outer_rect.left(),
            outer_rect.bottom() - border_width,
            outer_rect.width(),
            border_width,
        ),
        left: Rect::new(
            outer_rect.left(),
            outer_rect.top(),
            border_width,
            outer_rect.height(),
        ),
    }
}

fn empty_borders(rect: Rect) -> BorderRects {
    BorderRects {
        top: Rect::new(rect.left(), rect.top(), 0.0, 0.0),
        right: Rect::new(rect.right(), rect.top(), 0.0, 0.0),
        bottom: Rect::new(rect.left(), rect.bottom(), 0.0, 0.0),
        left: Rect::new(rect.left(), rect.top(), 0.0, 0.0),
    }
}

fn close_button_rect(titlebar_rect: Rect, size: f64) -> Option<Rect> {
    if size <= 0.0 || titlebar_rect.height() <= 0.0 {
        return None;
    }

    let button_size = size.min(titlebar_rect.height()).min(titlebar_rect.width());

    Some(Rect::new(
        titlebar_rect.right() - button_size,
        titlebar_rect.top() + (titlebar_rect.height() - button_size) / 2.0,
        button_size,
        button_size,
    ))
}

#[cfg(test)]
mod tests {
    use super::{DecorationHit, DecorationMode, compute_decoration_geometry};
    use crate::geometry::{Point, Rect};

    #[test]
    fn none_mode_keeps_outer_rect_equal_to_content_rect() {
        let content = Rect::new(10.0, 20.0, 300.0, 200.0);
        let geometry = compute_decoration_geometry(content, DecorationMode::None, 2.0, 28.0, 14.0);

        assert_eq!(geometry.outer_rect, content);
        assert_eq!(geometry.titlebar_rect, None);
        assert_eq!(geometry.close_button_rect, None);
    }

    #[test]
    fn border_mode_expands_outer_rect_by_border_width() {
        let content = Rect::new(10.0, 20.0, 300.0, 200.0);
        let geometry =
            compute_decoration_geometry(content, DecorationMode::Border, 2.0, 28.0, 14.0);

        assert_eq!(geometry.outer_rect, Rect::new(8.0, 18.0, 304.0, 204.0));
    }

    #[test]
    fn titlebar_mode_adds_titlebar_height() {
        let content = Rect::new(10.0, 20.0, 300.0, 200.0);
        let geometry =
            compute_decoration_geometry(content, DecorationMode::Titlebar, 2.0, 28.0, 14.0);

        assert_eq!(geometry.outer_rect, Rect::new(8.0, -10.0, 304.0, 232.0));
        assert_eq!(
            geometry.titlebar_rect,
            Some(Rect::new(10.0, -8.0, 300.0, 28.0))
        );
    }

    #[test]
    fn border_rects_have_expected_positions() {
        let content = Rect::new(10.0, 20.0, 300.0, 200.0);
        let geometry =
            compute_decoration_geometry(content, DecorationMode::Border, 2.0, 28.0, 14.0);

        assert_eq!(geometry.border_rects.top, Rect::new(8.0, 18.0, 304.0, 2.0));
        assert_eq!(
            geometry.border_rects.right,
            Rect::new(310.0, 18.0, 2.0, 204.0)
        );
        assert_eq!(
            geometry.border_rects.bottom,
            Rect::new(8.0, 220.0, 304.0, 2.0)
        );
        assert_eq!(geometry.border_rects.left, Rect::new(8.0, 18.0, 2.0, 204.0));
    }

    #[test]
    fn close_button_is_inside_titlebar() {
        let content = Rect::new(10.0, 20.0, 300.0, 200.0);
        let geometry =
            compute_decoration_geometry(content, DecorationMode::Titlebar, 2.0, 28.0, 14.0);
        let titlebar = geometry.titlebar_rect.unwrap();
        let close = geometry.close_button_rect.unwrap();

        assert!(titlebar.contains_rect(close));
    }

    #[test]
    fn zero_border_width_is_handled() {
        let content = Rect::new(10.0, 20.0, 300.0, 200.0);
        let geometry =
            compute_decoration_geometry(content, DecorationMode::Border, 0.0, 28.0, 14.0);

        assert_eq!(geometry.outer_rect, content);
        assert_eq!(geometry.border_rects.top.height(), 0.0);
    }

    #[test]
    fn hit_test_returns_none_outside_outer_rect() {
        let geometry = compute_decoration_geometry(
            Rect::new(10.0, 20.0, 300.0, 200.0),
            DecorationMode::Border,
            2.0,
            28.0,
            14.0,
        );

        assert_eq!(geometry.hit_test(Point::new(0.0, 0.0)), DecorationHit::None);
    }

    #[test]
    fn hit_test_returns_content_inside_content_rect() {
        let geometry = compute_decoration_geometry(
            Rect::new(10.0, 20.0, 300.0, 200.0),
            DecorationMode::Border,
            2.0,
            28.0,
            14.0,
        );

        assert_eq!(
            geometry.hit_test(Point::new(20.0, 30.0)),
            DecorationHit::Content
        );
    }

    #[test]
    fn hit_test_returns_border_hits() {
        let geometry = compute_decoration_geometry(
            Rect::new(10.0, 20.0, 300.0, 200.0),
            DecorationMode::Border,
            2.0,
            28.0,
            14.0,
        );

        assert_eq!(
            geometry.hit_test(Point::new(20.0, 19.0)),
            DecorationHit::BorderTop
        );
        assert_eq!(
            geometry.hit_test(Point::new(311.0, 30.0)),
            DecorationHit::BorderRight
        );
    }

    #[test]
    fn hit_test_returns_titlebar() {
        let geometry = compute_decoration_geometry(
            Rect::new(10.0, 20.0, 300.0, 200.0),
            DecorationMode::Titlebar,
            2.0,
            28.0,
            14.0,
        );

        assert_eq!(
            geometry.hit_test(Point::new(20.0, 0.0)),
            DecorationHit::Titlebar
        );
    }

    #[test]
    fn close_button_has_priority_over_titlebar() {
        let geometry = compute_decoration_geometry(
            Rect::new(10.0, 20.0, 300.0, 200.0),
            DecorationMode::Titlebar,
            2.0,
            28.0,
            14.0,
        );
        let close = geometry.close_button_rect.unwrap();

        assert_eq!(
            geometry.hit_test(close.center()),
            DecorationHit::CloseButton
        );
    }
}
