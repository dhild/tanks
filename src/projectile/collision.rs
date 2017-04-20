use physics::{Dimensions, Position};
use specs;

#[derive(Debug)]
pub struct CollisionSystem;

impl CollisionSystem {
    pub fn new() -> CollisionSystem {
        CollisionSystem {}
    }
}

impl<C> specs::System<C> for CollisionSystem {
    fn run(&mut self, arg: specs::RunArg, _: C) {
        use specs::Join;
        let (positions, dim, entities) =
            arg.fetch(|w| (w.read::<Position>(), w.read_resource::<Dimensions>(), w.entities()));

        let right_bounds = dim.width as f32;
        for (p, e) in (&positions, &entities).join() {
            if p.position.x < 0.0 || p.position.x > right_bounds || p.position.y < 0.0 {
                info!("Projectile went out of bounds at {:?}", p.position);
                arg.delete(e);
            }
        }
    }
}
