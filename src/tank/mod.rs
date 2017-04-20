use cgmath::{Deg, Matrix4, Rad, Vector3};
use physics::Position;
use rand::{self, Rng};
use specs;

mod control;
mod draw;

pub use self::control::{TankControl, TankControlSystem};
pub use self::draw::{Drawable, DrawSystem, PreDrawSystem};

#[derive(Debug)]
pub struct Tank {
    pub barrel_orient: Rad<f32>,
    pub power_level: f32,
}

impl Tank {
    pub fn new() -> Tank {
        let mut rng = rand::thread_rng();
        Tank {
            barrel_orient: Rad::from(Deg(rng.gen_range(-45.0, 45.0))),
            power_level: 0.5,
        }
    }

    pub fn barrel_to_world(&self, pos: &Position) -> Matrix4<f32> {
        Matrix4::from_translation(Vector3::new(pos.position.x, pos.position.y, 0.0)) *
        Matrix4::from_nonuniform_scale(pos.scale, pos.scale, 1.0) *
        Matrix4::from_angle_z(self.barrel_orient)
    }
}

impl specs::Component for Tank {
    type Storage = specs::HashMapStorage<Tank>;
}
