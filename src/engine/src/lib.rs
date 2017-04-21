extern crate gfx;
#[macro_use]
extern crate log;
extern crate specs;
#[cfg(feature = "gfx_device_gl")]
extern crate gfx_device_gl;
#[cfg(feature = "glutin")]
extern crate glutin;
#[cfg(feature = "gfx_window_glutin")]
extern crate gfx_window_glutin;
#[cfg(feature = "sdl2")]
extern crate sdl2;
#[cfg(feature = "gfx_window_sdl")]
extern crate gfx_window_sdl;

mod renderer;
mod game;

mod traits;

#[derive(Debug,PartialEq,Eq)]
pub enum RunStatus {
    Running,
    Quit,
}

pub type Delta = f32;

pub use renderer::EncoderQueue;
pub use traits::*;

#[cfg(feature = "window_sdl")]
mod window_sdl;
#[cfg(feature = "window_sdl")]
pub use window_sdl::run;

#[cfg(feature = "window_glutin")]
mod window_glutin;
#[cfg(feature = "window_glutin")]
pub use window_glutin::run;
