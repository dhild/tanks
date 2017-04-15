extern crate gfx;
extern crate gfx_window_sdl;
#[macro_use]
extern crate log;
extern crate sdl2;
extern crate specs;

mod renderer;
mod game;
mod window_sdl;

#[derive(Debug,PartialEq,Eq)]
pub enum RunStatus {
    Running,
    Quit,
}

pub type Delta = f32;

pub use game::GameFunctions;
pub use renderer::EncoderQueue;
pub use window_sdl::run;
