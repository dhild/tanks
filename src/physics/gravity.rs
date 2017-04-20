use super::{Delta, Mass, Velocity};
use specs;

#[derive(Debug)]
pub struct GravitySystem {
    gravity: f32,
}

impl GravitySystem {
    pub fn new() -> GravitySystem {
        GravitySystem { gravity: -0.98 }
    }
}

impl specs::System<Delta> for GravitySystem {
    fn run(&mut self, arg: specs::RunArg, time: Delta) {
        use specs::Join;
        let (mass, mut velocities) = arg.fetch(|w| (w.read::<Mass>(), w.write::<Velocity>()));
        for (m, v) in (&mass, &mut velocities).join() {
            let acc = m.mass * self.gravity;
            v.velocity.y += acc * time;
        }
    }
}
