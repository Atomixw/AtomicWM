use smithay::{
    reexports::wayland_server::protocol::{wl_seat, wl_surface},
    utils::Serial,
    wayland::shell::xdg::{
        Configure, PopupSurface, PositionerState, ToplevelSurface, XdgShellHandler, XdgShellState,
    },
};

use crate::{geometry::Size, window::WindowId};

use super::wayland::WaylandState;

#[derive(Debug, Clone)]
pub(crate) struct TrackedToplevel {
    pub surface: ToplevelSurface,
    pub window_id: Option<WindowId>,
}

impl WaylandState {
    fn window_id_for_toplevel(&self, surface: &ToplevelSurface) -> Option<WindowId> {
        self.xdg_toplevels
            .iter()
            .find(|tracked| tracked.surface == *surface)
            .and_then(|tracked| tracked.window_id)
    }

    fn remove_toplevel(&mut self, surface: &ToplevelSurface) -> Option<WindowId> {
        let index = self
            .xdg_toplevels
            .iter()
            .position(|tracked| tracked.surface == *surface)?;
        let tracked = self.xdg_toplevels.remove(index);

        self.remove_mapped_window(tracked.window_id?)
    }

    pub(crate) fn sync_toplevel_commit(&mut self, wl_surface: &wl_surface::WlSurface) {
        let Some(index) = self
            .xdg_toplevels
            .iter()
            .position(|tracked| tracked.surface.wl_surface() == wl_surface)
        else {
            return;
        };

        let has_buffer =
            smithay::backend::renderer::utils::with_renderer_surface_state(wl_surface, |state| {
                state.buffer().is_some()
            })
            .unwrap_or(false);

        match (self.xdg_toplevels[index].window_id, has_buffer) {
            (None, true) => {
                let size = initial_size(&self.xdg_toplevels[index].surface);
                let id = self.window_state.add_xdg_toplevel(None, None, Some(size));
                self.xdg_toplevels[index].window_id = Some(id);
                println!(
                    "xdg-shell toplevel mapped: window_id={} title=\"Untitled\" app_id=\"unknown\"",
                    id.value()
                );
            }
            (Some(id), false) => {
                self.remove_mapped_window(id);
                self.xdg_toplevels[index].window_id = None;
                self.xdg_toplevels[index]
                    .surface
                    .reset_initial_configure_sent();
                println!("xdg-shell toplevel unmapped: window_id={}", id.value());
            }
            _ => {}
        }
    }

    fn remove_mapped_window(&mut self, id: WindowId) -> Option<WindowId> {
        self.window_state.remove_xdg_toplevel(id)
    }
}

impl XdgShellHandler for WaylandState {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        let size = initial_size(&surface);

        surface.with_pending_state(|state| {
            state.size = Some((size.width as i32, size.height as i32).into());
        });
        surface.send_configure();

        self.xdg_toplevels.push(TrackedToplevel {
            surface: surface.clone(),
            window_id: None,
        });

        println!("xdg-shell toplevel created: awaiting initial buffer commit");
    }

    fn new_popup(&mut self, surface: PopupSurface, _positioner: PositionerState) {
        let _ = surface.send_configure();
    }

    fn grab(&mut self, _surface: PopupSurface, _seat: wl_seat::WlSeat, _serial: Serial) {}

    fn reposition_request(
        &mut self,
        surface: PopupSurface,
        _positioner: PositionerState,
        token: u32,
    ) {
        surface.send_repositioned(token);
    }

    fn ack_configure(&mut self, _surface: wl_surface::WlSurface, _configure: Configure) {}

    fn toplevel_destroyed(&mut self, surface: ToplevelSurface) {
        if let Some(id) = self.remove_toplevel(&surface) {
            println!("xdg-shell toplevel destroyed: window_id={}", id.value());
        }
    }

    fn popup_destroyed(&mut self, _surface: PopupSurface) {}

    fn title_changed(&mut self, surface: ToplevelSurface) {
        if let Some(id) = self.window_id_for_toplevel(&surface) {
            self.window_state.update_title(id, None);
        }
    }

    fn app_id_changed(&mut self, surface: ToplevelSurface) {
        if let Some(id) = self.window_id_for_toplevel(&surface) {
            self.window_state.update_app_id(id, None);
        }
    }
}

fn initial_size(surface: &ToplevelSurface) -> Size {
    surface
        .current_state()
        .size
        .map(|size| {
            let (width, height): (i32, i32) = size.into();
            Size::new(width as f64, height as f64)
        })
        .filter(|size| size.width > 0.0 && size.height > 0.0)
        .unwrap_or(super::state::DEFAULT_XDG_WINDOW_SIZE)
}
