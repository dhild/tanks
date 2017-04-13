#[macro_use]
extern crate log;
#[macro_use]
extern crate gfx;
extern crate gfx_window_sdl;
extern crate sdl2;

mod draw;
mod game;
mod window_sdl;

pub fn create(title: &str) {
    window_sdl::SDLHandle::new().window(title)
}
