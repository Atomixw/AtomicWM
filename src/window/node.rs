use crate::{
    geometry::{Point, Rect, Size, Vector},
    window::DecorationMode,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowId(pub u64);

impl WindowId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WindowNode {
    pub id: WindowId,
    pub title: String,
    pub app_id: String,
    pub rect: Rect,
    pub focused: bool,
    pub decoration_mode: DecorationMode,
}

impl WindowNode {
    pub fn new(
        id: WindowId,
        title: impl Into<String>,
        app_id: impl Into<String>,
        rect: Rect,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            app_id: app_id.into(),
            rect,
            focused: false,
            decoration_mode: DecorationMode::Border,
        }
    }

    pub fn move_by(&mut self, delta: Vector) {
        self.rect = self.rect.translate(delta.dx, delta.dy);
    }

    pub fn resize(&mut self, size: Size) {
        self.rect = Rect::from_origin_size(self.rect.origin, size);
    }

    pub fn rect(&self) -> Rect {
        self.rect
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    pub fn center(&self) -> Point {
        self.rect.center()
    }
}
