use cgmath::Deg;
use cgmath::prelude::*;
use physics::*;
use specs::{self, Join};
use tank::Tank;

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
        let (mut explosions, positions, mut tanks, entities) =
            args.fetch(|w| {
                           (w.write::<Explosion>(),
                            w.read::<Position>(),
                            w.write::<Tank>(),
                            w.entities())
                       });
        let mut damage_areas = Vec::new();
        for (e, p, id) in (&mut explosions, &positions, &entities).join() {
            e.time_elapsed += time;
            e.time_remaining -= time;
            trace!("Explosion running for {}, has {} remaining",
                   e.time_elapsed,
                   e.time_remaining);
            let mut damage_time = time;
            if e.time_remaining <= 0.0 {
                args.delete(id);
                damage_time += e.time_remaining;
            }
            if damage_time > 0.0 {
                damage_areas.push((p.position, e.radius(), damage_time * 10.0));
            }
        }
        for (t, p, id) in (&mut tanks, &positions, &entities).join() {
            for &(center, radius, damage) in &damage_areas {
                if p.position.distance(center) <= (radius + p.scale) {
                    t.health -= damage;
                    debug!("Tank health: {}", t.health);
                    if t.health <= 0.0 {
                        info!("Tank destroyed!");
                        args.delete(id);
                    }
                }
            }
        }
    }
}
