use cgmath::Deg;
use cgmath::prelude::*;
use specs::{self, Join};

mod draw;

pub use self::draw::{Drawable, DrawSystem, PreDrawSystem};

#[derive(Debug)]
pub struct Explosion {
    time_elapsed: f32,
    time_remaining: f32,
}

impl Explosion {
    pub fn new() -> Explosion {
        Explosion {
            time_elapsed: 0.0,
            time_remaining: 5.0,
        }
    }

    pub fn radius(&self) -> f32 {
        75.0 * Deg(self.time_elapsed / 10.0).sin()
    }
}

impl specs::Component for Explosion {
    type Storage = specs::HashMapStorage<Explosion>;
}

#[derive(Debug)]
pub struct ExplosionSystem;

impl ExplosionSystem {
    pub fn new() -> ExplosionSystem {
        ExplosionSystem {}
    }
}

impl specs::System<f32> for ExplosionSystem {
    fn run(&mut self, args: specs::RunArg, time: f32) {
        let (mut explosions, entities) = args.fetch(|w| (w.write::<Explosion>(), w.entities()));
        for (e, id) in (&mut explosions, &entities).join() {
            e.time_elapsed += time;
            e.time_remaining -= time;
            trace!("Explosion running for {}, has {} remaining",
                   e.time_elapsed,
                   e.time_remaining);
            if e.time_remaining <= 0.0 {
                args.delete(id);
            }
        }
    }
}
