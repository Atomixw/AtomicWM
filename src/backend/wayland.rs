use std::{ffi::OsString, sync::Arc, time::Duration};

use smithay::{
    backend::renderer::utils::on_commit_buffer_handler,
    delegate_compositor, delegate_seat, delegate_xdg_shell,
    input::{Seat, SeatHandler, SeatState, pointer::CursorImageStatus},
    reexports::{
        calloop::EventLoop,
        wayland_server::{
            Client, Display,
            backend::{ClientData, ClientId, DisconnectReason},
            protocol::wl_surface::WlSurface,
        },
    },
    wayland::{
        compositor::{CompositorClientState, CompositorHandler, CompositorState},
        shell::xdg::XdgShellState,
        socket::ListeningSocketSource,
    },
};

use crate::{backend::state::BackendWindowState, canvas::Camera, geometry::Size};

use super::runtime::BackendError;

pub struct WaylandBackend {
    event_loop: EventLoop<'static, LoopState>,
    state: LoopState,
    socket_name: OsString,
}

impl WaylandBackend {
    pub fn new(output_size: Size, placement_gap: f64) -> Result<Self, BackendError> {
        let event_loop = EventLoop::try_new().map_err(|error| {
            BackendError::Wayland(format!("event loop creation failed: {error}"))
        })?;
        let display = Display::<WaylandState>::new().map_err(|error| {
            BackendError::Wayland(format!("Wayland display creation failed: {error}"))
        })?;
        let display_handle = display.handle();
        let compositor_state = CompositorState::new::<WaylandState>(&display_handle);
        let xdg_shell_state = XdgShellState::new::<WaylandState>(&display_handle);
        let window_state =
            BackendWindowState::new(Camera::default_for_viewport(output_size), placement_gap);
        let wayland = WaylandState {
            compositor_state,
            xdg_shell_state,
            seat_state: SeatState::new(),
            window_state,
            xdg_toplevels: Vec::new(),
        };

        let socket = ListeningSocketSource::new_auto().map_err(|error| {
            BackendError::Wayland(format!("Wayland socket creation failed: {error}"))
        })?;
        let socket_name = socket.socket_name().to_os_string();

        event_loop
            .handle()
            .insert_source(socket, |client_stream, _, state: &mut LoopState| {
                let _ = state
                    .display
                    .handle()
                    .insert_client(client_stream, Arc::new(ClientState::default()));
            })
            .map_err(|error| {
                BackendError::Wayland(format!("Wayland socket registration failed: {error}"))
            })?;

        Ok(Self {
            event_loop,
            state: LoopState { display, wayland },
            socket_name,
        })
    }

    pub fn socket_name(&self) -> &std::ffi::OsStr {
        &self.socket_name
    }

    pub(crate) fn dispatch_once(&mut self, timeout: Duration) -> Result<(), BackendError> {
        self.event_loop
            .dispatch(timeout, &mut self.state)
            .map_err(|error| {
                BackendError::Wayland(format!("event loop dispatch failed: {error}"))
            })?;
        self.state
            .display
            .dispatch_clients(&mut self.state.wayland)
            .map_err(|error| BackendError::Wayland(format!("client dispatch failed: {error}")))?;
        self.state
            .display
            .flush_clients()
            .map_err(|error| BackendError::Wayland(format!("client flush failed: {error}")))?;

        Ok(())
    }
}

pub struct LoopState {
    display: Display<WaylandState>,
    wayland: WaylandState,
}

pub struct WaylandState {
    pub(crate) compositor_state: CompositorState,
    pub(crate) xdg_shell_state: XdgShellState,
    pub(crate) seat_state: SeatState<Self>,
    pub(crate) window_state: BackendWindowState,
    pub(crate) xdg_toplevels: Vec<super::xdg_shell::TrackedToplevel>,
}

impl CompositorHandler for WaylandState {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
        &client
            .get_data::<ClientState>()
            .expect("AtomicWM client state missing")
            .compositor_state
    }

    fn commit(&mut self, surface: &WlSurface) {
        on_commit_buffer_handler::<Self>(surface);
        self.sync_toplevel_commit(surface);
    }
}

impl AsMut<CompositorState> for WaylandState {
    fn as_mut(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }
}

impl SeatHandler for WaylandState {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;
    type TouchFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }

    fn focus_changed(&mut self, _seat: &Seat<Self>, _focused: Option<&WlSurface>) {}

    fn cursor_image(&mut self, _seat: &Seat<Self>, _image: CursorImageStatus) {}
}

delegate_compositor!(WaylandState);
delegate_seat!(WaylandState);
delegate_xdg_shell!(WaylandState);

#[derive(Default)]
struct ClientState {
    compositor_state: CompositorClientState,
}

impl ClientData for ClientState {
    fn initialized(&self, _client_id: ClientId) {}

    fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) {}
}
