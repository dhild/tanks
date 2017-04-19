use gfx;

mod system;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub use self::system::{DrawSystem, PreDrawSystem};
