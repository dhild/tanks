extern crate cgmath;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate sdl2;
extern crate specs;

extern crate engine;

use engine::{Delta, RunStatus};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    log4rs::init_file("src/log4rs.yaml", Default::default()).unwrap();
    debug!("Starting up....");

    let event_handler = |event| match event {
        Event::Quit { .. } |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => RunStatus::Quit,
        _ => RunStatus::Running,
    };

    engine::run("Tanks", event_handler, TanksGame {});
}

struct TanksGame;

impl engine::GameFunctions for TanksGame {
    fn extents(&mut self) -> cgmath::Point2<f32> {
        [1.0, 1.0].into()
    }
    fn setup_planner(&mut self, _planner: &mut specs::Planner<Delta>) {}
    fn check_status(&mut self, _world: &mut specs::World) -> RunStatus {
        RunStatus::Running
    }
}
