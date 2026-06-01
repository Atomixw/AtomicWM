use crate::geometry::Rect;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowId(pub u64);

#[derive(Debug, Clone, PartialEq)]
pub struct WindowNode {
    pub id: WindowId,
    pub title: String,
    pub app_id: String,
    pub rect: Rect,
}
