#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Quit,
    SpawnTerminal,
    ZoomIn,
    ZoomOut,
    ResetZoom,
    PanLeft,
    PanRight,
    PanUp,
    PanDown,
    FocusLeft,
    FocusRight,
    FocusUp,
    FocusDown,
    CenterFocused,
    FitAll,
    MoveClusterLeft,
    MoveClusterRight,
    MoveClusterUp,
    MoveClusterDown,
    FitFocusedCluster,
}

impl Action {
    pub fn name(self) -> &'static str {
        match self {
            Self::Quit => "quit",
            Self::SpawnTerminal => "spawn_terminal",
            Self::ZoomIn => "zoom_in",
            Self::ZoomOut => "zoom_out",
            Self::ResetZoom => "reset_zoom",
            Self::PanLeft => "pan_left",
            Self::PanRight => "pan_right",
            Self::PanUp => "pan_up",
            Self::PanDown => "pan_down",
            Self::FocusLeft => "focus_left",
            Self::FocusRight => "focus_right",
            Self::FocusUp => "focus_up",
            Self::FocusDown => "focus_down",
            Self::CenterFocused => "center_focused",
            Self::FitAll => "fit_all",
            Self::MoveClusterLeft => "move_cluster_left",
            Self::MoveClusterRight => "move_cluster_right",
            Self::MoveClusterUp => "move_cluster_up",
            Self::MoveClusterDown => "move_cluster_down",
            Self::FitFocusedCluster => "fit_focused_cluster",
        }
    }
}
