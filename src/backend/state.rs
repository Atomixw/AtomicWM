use crate::{
    canvas::{Camera, World},
    geometry::Size,
    window::{PlacementMode, PlacementRequest, WindowId},
};

pub const DEFAULT_XDG_WINDOW_SIZE: Size = Size {
    width: 800.0,
    height: 600.0,
};

#[derive(Debug)]
pub struct BackendWindowState {
    world: World,
    camera: Camera,
    next_window_id: u64,
    placement_gap: f64,
}

impl BackendWindowState {
    pub fn new(camera: Camera, placement_gap: f64) -> Self {
        Self {
            world: World::new(),
            camera,
            next_window_id: 1,
            placement_gap: placement_gap.max(0.0),
        }
    }

    #[cfg(test)]
    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn add_xdg_toplevel(
        &mut self,
        title: Option<String>,
        app_id: Option<String>,
        requested_size: Option<Size>,
    ) -> WindowId {
        let id = self.allocate_window_id();
        let size = valid_size_or_default(requested_size);
        let request = PlacementRequest::new(size, PlacementMode::NearFocused, self.placement_gap);
        let title = non_empty_or_default(title, "Untitled");
        let app_id = non_empty_or_default(app_id, "unknown");

        self.world
            .add_window_with_placement(id, title, app_id, &self.camera, request);

        id
    }

    pub fn remove_xdg_toplevel(&mut self, id: WindowId) -> Option<WindowId> {
        self.world.remove_window(id)?;

        if self.world.focused_window_id().is_none() {
            self.world.focus_first();
        }

        Some(id)
    }

    pub fn update_title(&mut self, id: WindowId, title: Option<String>) -> bool {
        let Some(window) = self.world.window_mut(id) else {
            return false;
        };

        window.title = non_empty_or_default(title, "Untitled");
        true
    }

    pub fn update_app_id(&mut self, id: WindowId, app_id: Option<String>) -> bool {
        let Some(window) = self.world.window_mut(id) else {
            return false;
        };

        window.app_id = non_empty_or_default(app_id, "unknown");
        true
    }

    fn allocate_window_id(&mut self) -> WindowId {
        let id = WindowId::new(self.next_window_id);
        self.next_window_id += 1;
        id
    }
}

fn valid_size_or_default(size: Option<Size>) -> Size {
    match size {
        Some(size) if size.width > 0.0 && size.height > 0.0 => size,
        _ => DEFAULT_XDG_WINDOW_SIZE,
    }
}

fn non_empty_or_default(value: Option<String>, fallback: &str) -> String {
    match value {
        Some(value) if !value.trim().is_empty() => value,
        _ => fallback.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        canvas::Camera,
        geometry::{Point, Size},
    };

    use super::{BackendWindowState, DEFAULT_XDG_WINDOW_SIZE};

    fn state() -> BackendWindowState {
        BackendWindowState::new(Camera::default_for_viewport(Size::new(1920.0, 1080.0)), 8.0)
    }

    #[test]
    fn allocates_window_ids_in_order() {
        let mut state = state();

        assert_eq!(state.add_xdg_toplevel(None, None, None).value(), 1);
        assert_eq!(state.add_xdg_toplevel(None, None, None).value(), 2);
    }

    #[test]
    fn adds_mapped_logical_window() {
        let mut state = state();
        let id = state.add_xdg_toplevel(
            Some("Terminal".to_string()),
            Some("alacritty".to_string()),
            Some(Size::new(900.0, 500.0)),
        );
        let window = state.world().window(id).unwrap();

        assert_eq!(window.title, "Terminal");
        assert_eq!(window.app_id, "alacritty");
        assert_eq!(window.rect.size, Size::new(900.0, 500.0));
        assert!(window.mapped);
    }

    #[test]
    fn missing_title_and_app_id_use_fallbacks() {
        let mut state = state();
        let id = state.add_xdg_toplevel(None, None, None);
        let window = state.world().window(id).unwrap();

        assert_eq!(window.title, "Untitled");
        assert_eq!(window.app_id, "unknown");
        assert_eq!(window.rect.size, DEFAULT_XDG_WINDOW_SIZE);
    }

    #[test]
    fn placement_is_used_for_new_windows() {
        let mut state = state();
        let first = state.add_xdg_toplevel(None, None, Some(Size::new(400.0, 300.0)));
        let second = state.add_xdg_toplevel(None, None, Some(Size::new(400.0, 300.0)));
        let first_rect = state.world().window(first).unwrap().rect;
        let second_rect = state.world().window(second).unwrap().rect;

        assert_eq!(first_rect.center(), Point::new(0.0, 0.0));
        assert_eq!(second_rect.left(), first_rect.right() + 8.0);
        assert_eq!(second_rect.top(), first_rect.top());
    }

    #[test]
    fn removes_mapped_window() {
        let mut state = state();
        let id = state.add_xdg_toplevel(None, None, None);

        assert_eq!(state.remove_xdg_toplevel(id), Some(id));
        assert!(state.world().is_empty());
    }

    #[test]
    fn removing_focused_window_falls_back_to_first_window() {
        let mut state = state();
        let first = state.add_xdg_toplevel(Some("First".to_string()), None, None);
        let second = state.add_xdg_toplevel(Some("Second".to_string()), None, None);

        assert_eq!(state.world().focused_window_id(), Some(second));
        assert_eq!(state.remove_xdg_toplevel(second), Some(second));
        assert_eq!(state.world().focused_window_id(), Some(first));
    }
}
