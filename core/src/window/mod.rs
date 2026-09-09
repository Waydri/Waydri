pub mod floating;
pub mod fullscreen;
pub mod group;
pub mod tiling;
pub mod window;
pub mod window_manager;

pub use floating::FloatingLayer;
pub use fullscreen::FullscreenState;
pub use group::{GroupRegistry, WindowGroup};
pub use tiling::{SplitAxis, SplitPos, TilingNode, TilingTree};
pub use window::{Window, WindowId};
pub use window_manager::WindowManager;