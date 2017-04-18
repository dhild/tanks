use super::{Delta, Position, Velocity};
use cgmath::prelude::*;
use specs;

#[derive(Debug)]
pub struct InertiaSystem;

impl InertiaSystem {
    pub fn new() -> InertiaSystem {
        InertiaSystem {}
    }
}

impl specs::System<Delta> for InertiaSystem {
    fn run(&mut self, arg: specs::RunArg, time: Delta) {
        use specs::Join;
        let (mut positions, mut velocities) =
            arg.fetch(|w| (w.write::<Position>(), w.write::<Velocity>()));
        for (p, v) in (&mut positions, &mut velocities).join() {
            p.position += v.velocity * time;
            p.orient = (p.orient + (v.angular_velocity * time)).normalize();
        }
    }
}
