pub mod animation;
pub mod compositor;
pub mod config;
pub mod effects;
pub mod input;
pub mod ipc;
pub mod layout;
pub mod logger;
pub mod output;
pub mod plugin;
pub mod renderer;
pub mod utils;
pub mod wayland;
pub mod window;
pub mod workspace;

use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Mutex, OnceLock};
use std::thread::JoinHandle;
use std::time::Duration;

use compositor::CompositorState;
use output::OutputManager;
use renderer::SoftwareRenderer;

pub const COMPOSITOR_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const TARGET_OS: &str = env!("WAYDRI_TARGET_OS");
pub const TARGET_ARCH: &str = env!("WAYDRI_TARGET_ARCH");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunCommand {
    Start,
    Pause,
    Resume,
    Stop,
}

pub struct WaydriState {
    pub compositor: CompositorState,
    pub renderer: SoftwareRenderer,
    pub outputs: OutputManager,
    pub config: config::Config,
    pub layout: layout::LayoutManager,
    pub window_manager: window::WindowManager,
    pub workspace_manager: workspace::WorkspaceManager,
    pub input: input::InputManager,
    pub ipc: Option<ipc::IpcServer>,
    pub running: bool,
}

impl WaydriState {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let cfg = config::ConfigLoader::new()
            .load()
            .unwrap_or_else(|_| config::Config::default());
        let mut outputs = OutputManager::new();
        outputs.add_headless("default".to_string(), 1920, 1080, 120);
        Ok(WaydriState {
            compositor: CompositorState::new(),
            renderer: SoftwareRenderer::new_with_fallback(1920, 1080),
            outputs,
            config: cfg,
            layout: layout::LayoutManager::with_default(),
            window_manager: window::WindowManager::new(),
            workspace_manager: workspace::WorkspaceManager::new(5),
            input: input::InputManager::new(),
            ipc: None,
            running: false,
        })
    }

    pub fn tick(&mut self, elapsed: Duration) {
        self.animation_tick(elapsed);
        self.input.dispatch();
        self.apply_layout();
    }

    fn animation_tick(&mut self, _elapsed: Duration) {}

    fn apply_layout(&mut self) {
        let active = self.workspace_manager.active_workspace();
        let windows = active.window_ids();
        let work = self.outputs.focused_geometry();
        let rects = self.layout.arrange(active.layout_name(), work, &windows);
        for (window_id, rect) in windows.iter().zip(rects.iter()) {
            self.window_manager.set_placement(*window_id, *rect);
        }
    }

    pub fn compose(&mut self) {
        self.outputs.compose(
            &mut self.renderer,
            &self.compositor,
            &self.window_manager,
        );
    }
}

pub struct Runtime {
    pub handle: Option<JoinHandle<()>>,
    pub tx: Option<Sender<RunCommand>>,
}

static RUNTIME: OnceLock<Mutex<Runtime>> = OnceLock::new();

fn runtime() -> &'static Mutex<Runtime> {
    RUNTIME.get_or_init(|| {
        Mutex::new(Runtime {
            handle: None,
            tx: None,
        })
    })
}

fn spawn_runner(rx: Receiver<RunCommand>) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let mut state = match WaydriState::new() {
            Ok(state) => state,
            Err(err) => {
                eprintln!("waydri: failed to initialize: {err}");
                return;
            }
        };
        state.running = true;
        let mut last = std::time::Instant::now();
        loop {
            while let Ok(command) = rx.try_recv() {
                match command {
                    RunCommand::Pause => state.running = false,
                    RunCommand::Resume => state.running = true,
                    RunCommand::Stop => {
                        state.compose();
                        return;
                    }
                    RunCommand::Start => state.running = true,
                }
            }
            if !state.running {
                std::thread::sleep(Duration::from_millis(50));
                continue;
            }
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(last);
            last = now;
            state.tick(elapsed);
            state.compose();
            let budget = Duration::from_millis(1000 / state.config.general.max_fps.max(1) as u64);
            let spent = now.elapsed();
            if spent < budget {
                std::thread::sleep(budget - spent);
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn rust_compositor_start() {
    let mut rt = runtime().lock().unwrap();
    if rt.handle.is_some() {
        return;
    }
    let (tx, rx) = channel();
    rt.tx = Some(tx);
    rt.handle = Some(spawn_runner(rx));
}

#[no_mangle]
pub extern "C" fn rust_compositor_stop() {
    let mut rt = runtime().lock().unwrap();
    if let Some(tx) = rt.tx.take() {
        let _ = tx.send(RunCommand::Stop);
    }
    if let Some(handle) = rt.handle.take() {
        let _ = handle.join();
    }
}

#[no_mangle]
pub extern "C" fn rust_compositor_pause() {
    let rt = runtime().lock().unwrap();
    if let Some(tx) = &rt.tx {
        let _ = tx.send(RunCommand::Pause);
    }
}

#[no_mangle]
pub extern "C" fn rust_compositor_resume() {
    let rt = runtime().lock().unwrap();
    if let Some(tx) = &rt.tx {
        let _ = tx.send(RunCommand::Resume);
    }
}

#[no_mangle]
pub extern "C" fn rust_compositor_is_running() -> bool {
    runtime().lock().unwrap().handle.is_some()
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn android_main() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_tag("WaydriCore")
            .with_max_level(log::LevelFilter::Debug),
    );
    log::info!("waydri-core {COMPOSITOR_VERSION} android runtime booting");
    rust_compositor_start();
}

#[cfg(not(target_os = "android"))]
fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("waydri-core {COMPOSITOR_VERSION} linux runtime booting");
    rust_compositor_start();
    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}