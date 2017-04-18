use super::{Delta, Position, Velocity};
use cgmath::{Deg, Rad, Point2, Vector2};
use cgmath::prelude::*;
use specs;
use terrain::Terrain;

#[derive(Debug)]
pub struct SettleSystem {
    gravity: f32,
    terrain: specs::Entity,
}

impl SettleSystem {
    pub fn new(terrain: specs::Entity) -> SettleSystem {
        SettleSystem {
            gravity: -9.8,
            terrain: terrain,
        }
    }

    pub fn target(p: &Position, v: &mut Velocity, terrain: &Terrain) -> (Point2<f32>, Rad<f32>) {
        let terrain_height = terrain.get_height(p.position.x);
        let normal_dir = terrain.get_normal_dir(p.position.x);
        (Point2::new(terrain_height, p.position.y), normal_dir)
    }

    pub fn settle(&self, p: &Position, v: &mut Velocity, terrain: &Terrain, time: Delta) {
        let terrain_height = terrain.get_height(p.position.x);
        if terrain_height < p.position.y {
            let gravity = Vector2::new(0.0, self.gravity * time);
            v.velocity += gravity;
        } else {
            v.velocity = Vector2::zero();
            let normal_dir = terrain.get_normal_dir(p.position.x);
            let diff = if p.orient > normal_dir {
                p.orient - normal_dir
            } else {
                normal_dir - p.orient
            };
            if diff < (v.angular_velocity * 5.0 * time) {
                v.angular_velocity *= 0.75;
            }
            if diff > Rad::from(Deg(0.005)) {
                let gravity = if p.orient > normal_dir {
                    self.gravity
                } else {
                    -self.gravity
                };
                v.angular_velocity += Rad(gravity * time * 0.05);
            }
        }
    }
}

impl specs::System<Delta> for SettleSystem {
    fn run(&mut self, arg: specs::RunArg, time: Delta) {
        use specs::Join;
        let (positions, mut velocities, terrain) =
            arg.fetch(|w| (w.read::<Position>(), w.write::<Velocity>(), w.read::<Terrain>()));
        let terrain = terrain.get(self.terrain).unwrap();
        for (p, v) in (&positions, &mut velocities).join() {
            self.settle(p, v, terrain, time);
        }
    }
}
