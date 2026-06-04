use std::time::Duration;

use crate::render::{ClearFrame, ClearRenderer, Color, OutputState};

use super::wayland::WaylandBackend;

#[derive(Debug, Clone, PartialEq)]
pub struct BackendConfig {
    pub max_dispatch_cycles: Option<usize>,
    pub output: OutputState,
    pub background: Color,
    pub placement_gap: f64,
}

impl BackendConfig {
    pub fn compositor(background: Color, placement_gap: f64) -> Self {
        Self {
            max_dispatch_cycles: None,
            output: OutputState::default_headless(),
            background,
            placement_gap,
        }
    }

    pub fn backend_test(background: Color, placement_gap: f64) -> Self {
        Self {
            max_dispatch_cycles: Some(3),
            output: OutputState::default_headless(),
            background,
            placement_gap,
        }
    }
}

pub struct Backend {
    wayland: WaylandBackend,
    renderer: ClearRenderer,
    config: BackendConfig,
    last_frame: Option<ClearFrame>,
}

impl Backend {
    pub fn new(config: BackendConfig) -> Result<Self, BackendError> {
        let renderer = ClearRenderer::new(config.background);

        Ok(Self {
            wayland: WaylandBackend::new(config.output.size, config.placement_gap)?,
            renderer,
            config,
            last_frame: None,
        })
    }

    pub fn run(&mut self) -> Result<(), BackendError> {
        println!(
            "AtomicWM Wayland socket: {}",
            self.wayland.socket_name().to_string_lossy()
        );
        println!(
            "AtomicWM output: {} {:.0}x{:.0} scale {:.1}",
            self.config.output.name,
            self.config.output.size.width,
            self.config.output.size.height,
            self.config.output.scale
        );

        match self.config.max_dispatch_cycles {
            Some(cycles) => self.run_for(cycles, Duration::from_millis(1)),
            None => self.run_forever(Duration::from_millis(16)),
        }
    }

    pub fn last_frame(&self) -> Option<ClearFrame> {
        self.last_frame
    }

    fn run_forever(&mut self, timeout: Duration) -> Result<(), BackendError> {
        loop {
            self.dispatch_frame(timeout)?;
        }
    }

    fn run_for(&mut self, cycles: usize, timeout: Duration) -> Result<(), BackendError> {
        for _ in 0..cycles {
            self.dispatch_frame(timeout)?;
        }

        Ok(())
    }

    fn dispatch_frame(&mut self, timeout: Duration) -> Result<(), BackendError> {
        self.wayland.dispatch_once(timeout)?;
        self.last_frame = Some(self.renderer.clear_frame(&self.config.output));

        Ok(())
    }
}

#[derive(Debug)]
pub enum BackendError {
    Wayland(String),
}

impl std::fmt::Display for BackendError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wayland(message) => write!(formatter, "{message}"),
        }
    }
}

impl std::error::Error for BackendError {}

impl From<Box<dyn std::error::Error>> for BackendError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        Self::Wayland(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        os::unix::fs::PermissionsExt,
        os::unix::net::UnixListener,
        path::PathBuf,
        sync::Mutex,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::{Backend, BackendConfig};
    use crate::render::Color;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn backend_config_can_be_created() {
        let background = test_background();

        assert_eq!(
            BackendConfig::backend_test(background, 8.0).max_dispatch_cycles,
            Some(3)
        );
        assert_eq!(
            BackendConfig::compositor(background, 8.0).max_dispatch_cycles,
            None
        );
    }

    #[test]
    fn backend_test_mode_initializes_and_shuts_down() {
        let _guard = ENV_LOCK.lock().unwrap();
        let previous_runtime_dir = std::env::var_os("XDG_RUNTIME_DIR");
        let runtime_dir = temp_runtime_dir();

        unsafe {
            std::env::set_var("XDG_RUNTIME_DIR", &runtime_dir);
        }

        if !can_bind_unix_socket(&runtime_dir) {
            restore_runtime_dir(previous_runtime_dir);
            let _ = fs::remove_dir_all(&runtime_dir);
            return;
        }

        {
            let mut backend =
                Backend::new(BackendConfig::backend_test(test_background(), 8.0)).unwrap();
            backend.run().unwrap();

            assert_eq!(backend.last_frame().unwrap().background, test_background());
        }

        restore_runtime_dir(previous_runtime_dir);
        let _ = fs::remove_dir_all(&runtime_dir);
    }

    fn restore_runtime_dir(previous_runtime_dir: Option<std::ffi::OsString>) {
        if let Some(previous_runtime_dir) = previous_runtime_dir {
            unsafe {
                std::env::set_var("XDG_RUNTIME_DIR", previous_runtime_dir);
            }
        } else {
            unsafe {
                std::env::remove_var("XDG_RUNTIME_DIR");
            }
        }
    }

    fn can_bind_unix_socket(runtime_dir: &std::path::Path) -> bool {
        let path = runtime_dir.join("atomicwm-test-socket");
        let result = UnixListener::bind(&path);
        let _ = fs::remove_file(&path);

        result.is_ok()
    }

    fn temp_runtime_dir() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("atomicwm-runtime-{unique}"));

        fs::create_dir_all(&path).unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o700)).unwrap();

        path
    }

    fn test_background() -> Color {
        Color::from_hex_rgb("#111111").unwrap()
    }
}
