use physics::{Dimensions, Position};
use specs;
use terrain::Terrain;

#[derive(Debug)]
pub struct CollisionSystem;

impl CollisionSystem {
    pub fn new() -> CollisionSystem {
        CollisionSystem {}
    }
}

enum Collision {
    None,
    OutOfBounds,
    Terrain,
}
fn check_collision(p: &Position, dim: &Dimensions, terrain: &Terrain) -> Collision {
    if p.position.x < 0.0 || p.position.x > (dim.width as f32) || p.position.y < 0.0 {
        info!("Projectile went out of bounds at {:?}", p.position);
        Collision::OutOfBounds
    } else if terrain.get_height(p.position.x) > p.position.y {
        info!("Projectile went into terrain at {:?}", p.position);
        Collision::Terrain
    } else {
        Collision::None
    }
}

impl<C> specs::System<C> for CollisionSystem {
    fn run(&mut self, arg: specs::RunArg, _: C) {
        use specs::Join;
        let (positions, dim, entities, terrain) =
            arg.fetch(|w| {
                          (w.read::<Position>(),
                           w.read_resource::<Dimensions>(),
                           w.entities(),
                           w.read_resource::<Terrain>())
                      });

        for (p, e) in (&positions, &entities).join() {
            match check_collision(p, &dim, &terrain) {
                Collision::None => (),
                Collision::OutOfBounds => arg.delete(e),
                Collision::Terrain => {
                    arg.delete(e)
                    // TODO: Create an explosion at this point
                }
            }
        }
    }
}
