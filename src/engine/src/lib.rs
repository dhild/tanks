#[macro_use]
extern crate log;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_sdl;
extern crate sdl2;
extern crate cgmath;
extern crate specs;

mod components;
mod draw;
mod game;
mod window_sdl;

#[derive(Debug,PartialEq,Eq)]
pub enum RunStatus {
    Running,
    Quit,
}

pub use components::{Delta, Position};
pub use game::GameFunctions;
pub use window_sdl::run;
