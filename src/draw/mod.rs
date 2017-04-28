use gfx;

mod renderer;
mod text;
mod traits;
mod system;
mod window;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::Depth;

pub use self::renderer::{DeviceRenderer, EncoderQueue};
pub use self::system::DrawSystem;
pub use self::traits::*;
pub use self::window::GlutinWindow;
