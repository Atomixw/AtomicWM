use crate::{
    geometry::{Rect, Size, Vector},
    window::{WindowId, WindowNode},
};

#[derive(Debug, Default)]
pub struct World {
    windows: Vec<WindowNode>,
}

impl World {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
        }
    }

    pub fn add_window(&mut self, window: WindowNode) {
        self.windows.push(window);
    }

    pub fn is_empty(&self) -> bool {
        self.windows.is_empty()
    }

    pub fn remove_window(&mut self, id: WindowId) -> Option<WindowNode> {
        let index = self.windows.iter().position(|window| window.id == id)?;

        Some(self.windows.remove(index))
    }

    pub fn window(&self, id: WindowId) -> Option<&WindowNode> {
        self.windows.iter().find(|window| window.id == id)
    }

    pub fn window_mut(&mut self, id: WindowId) -> Option<&mut WindowNode> {
        self.windows.iter_mut().find(|window| window.id == id)
    }

    pub fn windows(&self) -> &[WindowNode] {
        &self.windows
    }

    pub fn focused_window(&self) -> Option<&WindowNode> {
        self.windows.iter().find(|window| window.focused)
    }

    pub fn focus_window(&mut self, id: WindowId) -> bool {
        if !self.windows.iter().any(|window| window.id == id) {
            return false;
        }

        for window in &mut self.windows {
            window.focused = window.id == id;
        }

        true
    }

    pub fn move_window(&mut self, id: WindowId, delta: Vector) -> bool {
        let Some(window) = self.window_mut(id) else {
            return false;
        };

        window.move_by(delta);
        true
    }

    pub fn resize_window(&mut self, id: WindowId, size: Size) -> bool {
        let Some(window) = self.window_mut(id) else {
            return false;
        };

        window.resize(size);
        true
    }

    pub fn bounds(&self) -> Option<Rect> {
        let mut windows = self.windows.iter();
        let first = windows.next()?.rect;

        Some(windows.fold(first, |bounds, window| bounds.union(window.rect)))
    }
}

#[cfg(test)]
mod tests {
    use super::World;
    use crate::{
        geometry::{Rect, Size, Vector},
        window::{WindowId, WindowNode},
    };

    fn window(id: u64, rect: Rect) -> WindowNode {
        WindowNode::new(WindowId::new(id), format!("Window {id}"), "test-app", rect)
    }

    #[test]
    fn adds_and_removes_window() {
        let mut world = World::new();
        let id = WindowId::new(1);

        world.add_window(window(1, Rect::new(0.0, 0.0, 100.0, 100.0)));

        assert_eq!(world.windows().len(), 1);
        assert!(world.window(id).is_some());
        assert_eq!(world.remove_window(id).map(|window| window.id), Some(id));
        assert!(world.is_empty());
    }

    #[test]
    fn focuses_only_one_window() {
        let mut world = World::new();
        let first = WindowId::new(1);
        let second = WindowId::new(2);

        world.add_window(window(1, Rect::new(0.0, 0.0, 100.0, 100.0)));
        world.add_window(window(2, Rect::new(200.0, 0.0, 100.0, 100.0)));

        assert!(world.focus_window(first));
        assert_eq!(world.focused_window().map(|window| window.id), Some(first));

        assert!(world.focus_window(second));
        assert_eq!(world.focused_window().map(|window| window.id), Some(second));
        assert!(!world.window(first).unwrap().focused);
        assert!(!world.focus_window(WindowId::new(99)));
    }

    #[test]
    fn moves_window() {
        let mut world = World::new();
        let id = WindowId::new(1);

        world.add_window(window(1, Rect::new(0.0, 0.0, 100.0, 100.0)));

        assert!(world.move_window(id, Vector::new(10.0, 20.0)));
        assert_eq!(
            world.window(id).unwrap().rect,
            Rect::new(10.0, 20.0, 100.0, 100.0)
        );
        assert!(!world.move_window(WindowId::new(99), Vector::zero()));
    }

    #[test]
    fn resizes_window() {
        let mut world = World::new();
        let id = WindowId::new(1);

        world.add_window(window(1, Rect::new(0.0, 0.0, 100.0, 100.0)));

        assert!(world.resize_window(id, Size::new(320.0, 240.0)));
        assert_eq!(
            world.window(id).unwrap().rect,
            Rect::new(0.0, 0.0, 320.0, 240.0)
        );
        assert!(!world.resize_window(WindowId::new(99), Size::new(1.0, 1.0)));
    }

    #[test]
    fn returns_bounds_of_multiple_windows() {
        let mut world = World::new();

        world.add_window(window(1, Rect::new(10.0, 10.0, 100.0, 100.0)));
        world.add_window(window(2, Rect::new(-50.0, 40.0, 25.0, 200.0)));

        assert_eq!(world.bounds(), Some(Rect::new(-50.0, 10.0, 160.0, 230.0)));
    }

    #[test]
    fn empty_world_has_no_bounds() {
        assert_eq!(World::new().bounds(), None);
    }
}
