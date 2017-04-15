use gfx;

mod components;
mod flat;
mod system;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub use self::components::{Drawable, Position};
pub use self::system::{DrawSystem, PreDrawSystem};
