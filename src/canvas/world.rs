use crate::window::WindowNode;

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

    pub fn windows(&self) -> &[WindowNode] {
        &self.windows
    }
}
