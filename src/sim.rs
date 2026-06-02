use std::{error::Error, io::Write};

use crate::{
    canvas::{Camera, World},
    config::Config,
    geometry::{Point, Rect, Size, Vector},
    input::Action,
    window::{Direction, PlacementMode, PlacementRequest, WindowId, WindowNode},
};

const CLUSTER_TOLERANCE: f64 = 1.0;

#[derive(Debug)]
pub struct SimulationState {
    pub world: World,
    pub camera: Camera,
    pub done: bool,
}

pub fn run_simulation(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut state = build_initial_state();
    let mut stdout = std::io::stdout();

    write_simulation_output(&mut stdout, config, &mut state)
}

fn write_simulation_output(
    mut writer: impl Write,
    config: &Config,
    state: &mut SimulationState,
) -> Result<(), Box<dyn Error>> {
    writeln!(writer, "AtomicWM simulation")?;
    writeln!(writer, "Config:")?;
    writeln!(writer, "  terminal: {}", config.commands.terminal)?;
    writeln!(writer)?;

    writeln!(writer, "Initial camera:")?;
    print_camera(&mut writer, &state.camera)?;
    writeln!(writer)?;

    writeln!(writer, "Initial windows:")?;
    print_world_state(&mut writer, &state.world)?;
    writeln!(writer)?;

    writeln!(writer, "Actions:")?;
    apply_scripted_actions(&mut writer, state, config)?;
    print_focused_decoration(&mut writer, state, config)?;
    writeln!(writer)?;

    writeln!(writer, "Final camera:")?;
    print_camera(&mut writer, &state.camera)?;
    writeln!(writer)?;

    writeln!(writer, "Final windows:")?;
    print_world_state(&mut writer, &state.world)?;

    Ok(())
}

pub fn build_initial_state() -> SimulationState {
    let mut world = World::new();
    let camera = Camera::default_for_viewport(Size::new(1920.0, 1080.0));

    world.add_window_with_placement(
        WindowId::new(1),
        "Terminal",
        "Alacritty",
        &camera,
        PlacementRequest::new(
            Size::new(800.0, 500.0),
            PlacementMode::AtWorldPosition(Point::new(100.0, 100.0)),
            8.0,
        ),
    );
    world.add_window(WindowNode::new(
        WindowId::new(2),
        "Browser",
        "firefox",
        Rect::new(1000.0, 120.0, 1200.0, 800.0),
    ));
    world.add_window(WindowNode::new(
        WindowId::new(3),
        "Editor",
        "code",
        Rect::new(-700.0, 200.0, 900.0, 700.0),
    ));

    world.focus_window(WindowId::new(1));

    SimulationState {
        world,
        camera,
        done: false,
    }
}

pub fn apply_scripted_actions(
    mut writer: impl Write,
    state: &mut SimulationState,
    config: &Config,
) -> Result<(), Box<dyn Error>> {
    let actions = [
        Action::PanRight,
        Action::PanDown,
        Action::ZoomIn,
        Action::FocusRight,
        Action::CenterFocused,
        Action::FitAll,
    ];

    let center_request = PlacementRequest::new(
        Size::new(640.0, 360.0),
        PlacementMode::ViewportCenter,
        config.appearance.gap,
    );
    state.world.add_window_with_placement(
        WindowId::new(4),
        "Notes",
        "notes",
        &state.camera,
        center_request,
    );
    writeln!(writer, "  add window 4 with viewport_center placement")?;

    let near_focused_request = PlacementRequest::new(
        Size::new(500.0, 400.0),
        PlacementMode::NearFocused,
        config.appearance.gap,
    );
    state.world.add_window_with_placement(
        WindowId::new(5),
        "Chat",
        "chat",
        &state.camera,
        near_focused_request,
    );
    writeln!(writer, "  add window 5 with near_focused placement")?;

    for action in actions {
        apply_action(&mut writer, state, config, action)?;
    }

    if config.snapping.enabled {
        demonstrate_snapping(&mut writer, state, config)?;
    }

    Ok(())
}

pub fn apply_action(
    mut writer: impl Write,
    state: &mut SimulationState,
    config: &Config,
    action: Action,
) -> Result<(), Box<dyn Error>> {
    writeln!(writer, "  {}", action.name())?;

    match action {
        Action::Quit => state.done = true,
        Action::SpawnTerminal => {
            writeln!(writer, "    action spawn_terminal ignored in simulation")?;
        }
        Action::ZoomIn => zoom_at_viewport_center(state, config.camera.zoom_step),
        Action::ZoomOut => zoom_at_viewport_center(state, 1.0 / config.camera.zoom_step),
        Action::ResetZoom => state.camera.reset_zoom(),
        Action::PanLeft => state.camera.pan(Vector::new(-config.camera.pan_step, 0.0)),
        Action::PanRight => state.camera.pan(Vector::new(config.camera.pan_step, 0.0)),
        Action::PanUp => state.camera.pan(Vector::new(0.0, -config.camera.pan_step)),
        Action::PanDown => state.camera.pan(Vector::new(0.0, config.camera.pan_step)),
        Action::FocusLeft => focus_in_direction(state, Direction::Left),
        Action::FocusRight => focus_in_direction(state, Direction::Right),
        Action::FocusUp => focus_in_direction(state, Direction::Up),
        Action::FocusDown => focus_in_direction(state, Direction::Down),
        Action::CenterFocused => {
            state.world.center_focused_window(&mut state.camera);
        }
        Action::FitAll => {
            state
                .world
                .fit_all(&mut state.camera, config.appearance.gap);
        }
        Action::MoveClusterLeft => move_focused_cluster(
            state,
            Vector::new(-config.camera.pan_step, 0.0),
            CLUSTER_TOLERANCE,
        ),
        Action::MoveClusterRight => move_focused_cluster(
            state,
            Vector::new(config.camera.pan_step, 0.0),
            CLUSTER_TOLERANCE,
        ),
        Action::MoveClusterUp => move_focused_cluster(
            state,
            Vector::new(0.0, -config.camera.pan_step),
            CLUSTER_TOLERANCE,
        ),
        Action::MoveClusterDown => move_focused_cluster(
            state,
            Vector::new(0.0, config.camera.pan_step),
            CLUSTER_TOLERANCE,
        ),
        Action::FitFocusedCluster => {
            state.world.fit_focused_cluster(
                &mut state.camera,
                config.appearance.gap,
                CLUSTER_TOLERANCE,
            );
        }
    }

    Ok(())
}

fn zoom_at_viewport_center(state: &mut SimulationState, zoom_factor: f64) {
    let viewport_center = Point::new(
        state.camera.viewport_size.width / 2.0,
        state.camera.viewport_size.height / 2.0,
    );

    state.camera.zoom_at(viewport_center, zoom_factor);
}

fn focus_in_direction(state: &mut SimulationState, direction: Direction) {
    if state.world.focus_in_direction(direction).is_some() {
        if let Some(window) = state.world.focused_window() {
            state.camera.center_on(window.center());
        }
    }
}

fn move_focused_cluster(state: &mut SimulationState, delta: Vector, tolerance: f64) {
    if let Some(cluster) = state.world.focused_cluster(tolerance) {
        state.world.move_cluster(cluster.id, delta, tolerance);
    }
}

fn demonstrate_snapping(
    mut writer: impl Write,
    state: &mut SimulationState,
    config: &Config,
) -> Result<(), Box<dyn Error>> {
    state.world.add_window(WindowNode::new(
        WindowId::new(6),
        "Snap Target",
        "snap-target",
        Rect::new(3000.0, 0.0, 400.0, 300.0),
    ));
    state.world.add_window(WindowNode::new(
        WindowId::new(7),
        "Snap Moving",
        "snap-moving",
        Rect::new(2500.0, 0.0, 400.0, 300.0),
    ));

    let before = state.world.window(WindowId::new(7)).unwrap().rect();
    writeln!(writer, "  snap window 7 before {}", format_rect(before))?;

    state.world.move_window_with_snapping(
        WindowId::new(7),
        Vector::new(76.0, 0.0),
        config.snapping.threshold,
        config.snapping.gap,
    );

    let after = state.world.window(WindowId::new(7)).unwrap().rect();
    writeln!(writer, "  snap window 7 after {}", format_rect(after))?;

    state.world.add_window(WindowNode::new(
        WindowId::new(8),
        "Cluster Separate",
        "cluster-separate",
        Rect::new(4200.0, 0.0, 300.0, 200.0),
    ));
    state.world.focus_window(WindowId::new(6));

    print_clusters(&mut writer, state, CLUSTER_TOLERANCE)?;
    writeln!(writer, "  move focused cluster right")?;
    apply_action(&mut writer, state, config, Action::MoveClusterRight)?;
    writeln!(writer, "  fit focused cluster")?;
    apply_action(&mut writer, state, config, Action::FitFocusedCluster)?;

    Ok(())
}

fn print_clusters(
    mut writer: impl Write,
    state: &SimulationState,
    tolerance: f64,
) -> Result<(), Box<dyn Error>> {
    let clusters = state.world.clusters(tolerance);
    writeln!(writer, "  clusters: {}", clusters.len())?;

    for cluster in clusters {
        writeln!(
            writer,
            "    cluster {} windows={}",
            cluster.id.0,
            format_window_ids(&cluster.windows)
        )?;
    }

    Ok(())
}

fn print_focused_decoration(
    mut writer: impl Write,
    state: &SimulationState,
    config: &Config,
) -> Result<(), Box<dyn Error>> {
    let Some(id) = state.world.focused_window_id() else {
        return Ok(());
    };
    let Some(geometry) = state
        .world
        .window_decoration_geometry(id, &config.appearance)
    else {
        return Ok(());
    };

    writeln!(
        writer,
        "  decoration content {}",
        format_rect(geometry.content_rect)
    )?;
    writeln!(
        writer,
        "  decoration outer {}",
        format_rect(geometry.outer_rect)
    )?;

    if let Some(titlebar) = geometry.titlebar_rect {
        writeln!(writer, "  decoration titlebar {}", format_rect(titlebar))?;
    }

    if let Some(close_button) = geometry.close_button_rect {
        writeln!(
            writer,
            "  decoration close_button {}",
            format_rect(close_button)
        )?;
    }

    Ok(())
}

fn format_window_ids(ids: &[WindowId]) -> String {
    ids.iter()
        .map(|id| id.value().to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn print_camera(mut writer: impl Write, camera: &Camera) -> Result<(), Box<dyn Error>> {
    writeln!(writer, "  position: {}", format_point(camera.position))?;
    writeln!(writer, "  zoom: {:.2}", camera.zoom)?;
    writeln!(writer, "  viewport: {}", format_size(camera.viewport_size))?;

    Ok(())
}

fn print_world_state(mut writer: impl Write, world: &World) -> Result<(), Box<dyn Error>> {
    for window in world.windows() {
        writeln!(
            writer,
            "  [{}] {} \"{}\" rect={} focused={}",
            window.id.value(),
            window.app_id,
            window.title,
            format_rect(window.rect),
            window.focused
        )?;
    }

    Ok(())
}

fn format_point(point: Point) -> String {
    format!("({:.2}, {:.2})", point.x, point.y)
}

fn format_size(size: Size) -> String {
    format!("{:.2} x {:.2}", size.width, size.height)
}

fn format_rect(rect: Rect) -> String {
    format!(
        "({:.2}, {:.2}, {:.2}, {:.2})",
        rect.origin.x, rect.origin.y, rect.size.width, rect.size.height
    )
}

#[cfg(test)]
mod tests {
    use super::{
        apply_action, apply_scripted_actions, build_initial_state, write_simulation_output,
    };
    use crate::{
        config::Config,
        geometry::{Point, Rect},
        input::Action,
        window::{WindowId, WindowNode},
    };

    #[test]
    fn simulation_can_run_without_panic() {
        let mut output = Vec::new();
        let mut state = build_initial_state();

        write_simulation_output(&mut output, &Config::default(), &mut state).unwrap();

        let output = String::from_utf8(output).unwrap();
        assert!(output.contains("AtomicWM simulation"));
        assert!(output.contains("Final windows:"));
    }

    #[test]
    fn simulation_world_ends_with_only_one_focused_window() {
        let mut state = build_initial_state();

        apply_scripted_actions(Vec::new(), &mut state, &Config::default()).unwrap();

        let focused: Vec<_> = state
            .world
            .windows()
            .iter()
            .filter(|window| window.focused)
            .collect();

        assert_eq!(focused.len(), 1);
        assert_eq!(focused[0].id, WindowId::new(6));
    }

    #[test]
    fn applying_pan_right_changes_camera_position() {
        let mut state = build_initial_state();

        apply_action(Vec::new(), &mut state, &Config::default(), Action::PanRight).unwrap();

        assert_eq!(state.camera.position.x, Config::default().camera.pan_step);
    }

    #[test]
    fn applying_zoom_in_changes_zoom() {
        let mut state = build_initial_state();

        apply_action(Vec::new(), &mut state, &Config::default(), Action::ZoomIn).unwrap();

        assert_eq!(state.camera.zoom, Config::default().camera.zoom_step);
    }

    #[test]
    fn applying_fit_all_changes_camera_to_show_world_bounds() {
        let mut state = build_initial_state();
        let bounds = state.world.bounds().unwrap();

        apply_action(Vec::new(), &mut state, &Config::default(), Action::FitAll).unwrap();

        assert!(state.camera.viewport_rect_world().contains_rect(bounds));
    }

    #[test]
    fn applying_center_focused_centers_camera_on_focused_window() {
        let mut state = build_initial_state();
        state.world.focus_window(WindowId::new(2));
        let center = state.world.focused_window().unwrap().center();

        apply_action(
            Vec::new(),
            &mut state,
            &Config::default(),
            Action::CenterFocused,
        )
        .unwrap();

        assert_eq!(state.camera.position, center);
    }

    #[test]
    fn applying_focus_right_changes_focus_when_right_window_exists() {
        let mut state = build_initial_state();

        apply_action(
            Vec::new(),
            &mut state,
            &Config::default(),
            Action::FocusRight,
        )
        .unwrap();

        assert_eq!(state.world.focused_window_id(), Some(WindowId::new(2)));
    }

    #[test]
    fn applying_focus_right_centers_camera_on_new_focus() {
        let mut state = build_initial_state();

        apply_action(
            Vec::new(),
            &mut state,
            &Config::default(),
            Action::FocusRight,
        )
        .unwrap();

        let focused_center = state.world.focused_window().unwrap().center();
        assert_eq!(state.camera.position, focused_center);
    }

    #[test]
    fn moving_cluster_changes_every_window_in_cluster() {
        let mut state = build_initial_state();
        state.world.add_window(WindowNode::new(
            WindowId::new(10),
            "Left",
            "test",
            Rect::new(3000.0, 3000.0, 100.0, 100.0),
        ));
        state.world.add_window(WindowNode::new(
            WindowId::new(11),
            "Right",
            "test",
            Rect::new(3100.0, 3000.0, 100.0, 100.0),
        ));
        state.world.focus_window(WindowId::new(10));

        apply_action(
            Vec::new(),
            &mut state,
            &Config::default(),
            Action::MoveClusterRight,
        )
        .unwrap();

        assert_eq!(
            state.world.window(WindowId::new(10)).unwrap().rect(),
            Rect::new(3080.0, 3000.0, 100.0, 100.0)
        );
        assert_eq!(
            state.world.window(WindowId::new(11)).unwrap().rect(),
            Rect::new(3180.0, 3000.0, 100.0, 100.0)
        );
    }

    #[test]
    fn fitting_focused_cluster_changes_camera() {
        let mut state = build_initial_state();
        state.world.add_window(WindowNode::new(
            WindowId::new(10),
            "Left",
            "test",
            Rect::new(3000.0, 3000.0, 100.0, 100.0),
        ));
        state.world.add_window(WindowNode::new(
            WindowId::new(11),
            "Right",
            "test",
            Rect::new(3100.0, 3000.0, 100.0, 100.0),
        ));
        state.world.focus_window(WindowId::new(10));

        apply_action(
            Vec::new(),
            &mut state,
            &Config::default(),
            Action::FitFocusedCluster,
        )
        .unwrap();

        assert_eq!(state.camera.position, Point::new(3100.0, 3050.0));
    }
}
