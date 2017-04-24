use cgmath::Deg;
use cgmath::prelude::*;
use explosion;
use physics::{Dimensions, Position};
use projectile::Projectile;
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
    if p.position.x < 0.0 || p.position.x > (dim.game_width() as f32) || p.position.y < 0.0 {
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
        let (mut positions, projectiles, mut explosives, mut drawables, dim, entities, terrain) =
            arg.fetch(|w| {
                (w.write::<Position>(),
                 w.read::<Projectile>(),
                 w.write::<explosion::Explosion>(),
                 w.write::<explosion::Drawable>(),
                 w.read_resource::<Dimensions>(),
                 w.entities(),
                 w.read_resource::<Terrain>())
            });

        let mut to_create = Vec::new();
        for (_, p, e) in (&projectiles.check(), &positions, &entities).join() {
            match check_collision(p, &dim, &terrain) {
                Collision::None => (),
                Collision::OutOfBounds => arg.delete(e),
                Collision::Terrain => {
                    arg.delete(e);
                    to_create.push(p.position);
                }
            }
        }
        for p in to_create {
            let id = arg.create_pure();
            positions.insert(id, Position::new(p.x, p.y, Deg::zero(), 50.0));
            explosives.insert(id, explosion::Explosion::new());
            drawables.insert(id, explosion::Drawable::new());
        }
    }
}
