mod renderer;
mod game;

mod traits;

#[derive(Debug,PartialEq,Eq)]
pub enum RunStatus {
    Running,
    Quit,
}

pub type Delta = f32;

pub use self::renderer::EncoderQueue;
pub use self::traits::*;

#[cfg(feature = "window_glutin")]
mod window_glutin;
#[cfg(feature = "window_glutin")]
pub use self::window_glutin::run;

#[cfg(feature = "window_sdl")]
mod window_sdl;
#[cfg(feature = "window_sdl")]
pub use self::window_sdl::run;
