use super::{Delta, Mass, Velocity};
use specs;

pub const GRAVITY: f32 = -0.98;

#[derive(Debug)]
pub struct GravitySystem;

impl GravitySystem {
    pub fn new() -> GravitySystem {
        GravitySystem {}
    }
}

impl specs::System<Delta> for GravitySystem {
    fn run(&mut self, arg: specs::RunArg, time: Delta) {
        use specs::Join;
        let (mass, mut velocities) = arg.fetch(|w| (w.read::<Mass>(), w.write::<Velocity>()));
        for (m, v) in (&mass, &mut velocities).join() {
            let acc = m.mass * GRAVITY;
            v.velocity.y += acc * time;
        }
    }
}
