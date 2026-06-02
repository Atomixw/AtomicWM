use std::{error::Error, io::Write};

use crate::{
    canvas::{Camera, World},
    config::Config,
    geometry::{Point, Rect, Size, Vector},
    window::{WindowId, WindowNode},
};

#[derive(Debug)]
pub struct SimulationState {
    pub world: World,
    pub camera: Camera,
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
    writeln!(
        writer,
        "  pan camera by {}",
        format_vector(Vector::new(120.0, 80.0))
    )?;
    writeln!(writer, "  zoom camera at viewport center by 1.10")?;
    writeln!(
        writer,
        "  move window 1 by {}",
        format_vector(Vector::new(40.0, 20.0))
    )?;
    writeln!(
        writer,
        "  resize window 2 to {}",
        format_size(Size::new(1280.0, 720.0))
    )?;
    writeln!(writer, "  focus window 3")?;
    writeln!(writer, "  fit camera to world bounds")?;

    apply_scripted_actions(state);
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
    }
}

pub fn apply_scripted_actions(state: &mut SimulationState) {
    state.camera.pan(Vector::new(120.0, 80.0));

    let viewport_center = Point::new(
        state.camera.viewport_size.width / 2.0,
        state.camera.viewport_size.height / 2.0,
    );
    state.camera.zoom_at(viewport_center, 1.1);

    state
        .world
        .move_window(WindowId::new(1), Vector::new(40.0, 20.0));
    state
        .world
        .resize_window(WindowId::new(2), Size::new(1280.0, 720.0));
    state.world.focus_window(WindowId::new(3));

    if let Some(bounds) = state.world.bounds() {
        state.camera.fit_rect(bounds, 80.0);
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

fn format_vector(vector: Vector) -> String {
    format!("({:.2}, {:.2})", vector.dx, vector.dy)
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
    use super::{apply_scripted_actions, build_initial_state, write_simulation_output};
    use crate::{config::Config, window::WindowId};

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

        apply_scripted_actions(&mut state);

        let focused: Vec<_> = state
            .world
            .windows()
            .iter()
            .filter(|window| window.focused)
            .collect();

        assert_eq!(focused.len(), 1);
        assert_eq!(focused[0].id, WindowId::new(3));
    }
}
