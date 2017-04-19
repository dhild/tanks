use cgmath::{Deg, Matrix4, Rad, Vector3};
use physics::Position;
use rand::{self, Rng};
use specs;

mod draw;

pub use self::draw::{Drawable, DrawSystem};

#[derive(Debug)]
pub struct Tank {
    pub player_number: u8,
    pub barrel_orient: Rad<f32>,
}

impl Tank {
    pub fn new(player_number: u8) -> Tank {
        let mut rng = rand::thread_rng();
        Tank {
            player_number: player_number,
            barrel_orient: Rad::from(Deg(rng.gen_range(-45.0, 45.0))),
        }
    }

    pub fn body_to_world(&self, pos: &Position) -> Matrix4<f32> {
        Matrix4::from_translation(Vector3::new(pos.position.x, pos.position.y, 0.0)) *
        Matrix4::from_nonuniform_scale(pos.scale, pos.scale, 1.0) *
        Matrix4::from_angle_z(pos.orient)
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
