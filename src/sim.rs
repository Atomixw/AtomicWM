use std::{error::Error, io::Write};

use crate::{
    canvas::{Camera, World},
    config::Config,
    geometry::{Point, Rect, Size, Vector},
    input::Action,
    window::{Direction, WindowId, WindowNode},
};

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

    world.add_window(WindowNode::new(
        WindowId::new(1),
        "Terminal",
        "Alacritty",
        Rect::new(100.0, 100.0, 800.0, 500.0),
    ));
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
        camera: Camera::default_for_viewport(Size::new(1920.0, 1080.0)),
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

    for action in actions {
        apply_action(&mut writer, state, config, action)?;
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
            if let Some(window) = state.world.focused_window() {
                state.camera.center_on(window.center());
            }
        }
        Action::FitAll => {
            if let Some(bounds) = state.world.bounds() {
                state.camera.fit_rect(bounds, config.appearance.gap);
            }
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
    use crate::{config::Config, input::Action, window::WindowId};

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
        assert_eq!(focused[0].id, WindowId::new(2));
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
}
