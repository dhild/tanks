use cgmath::{Matrix4, Point2, Rad, Vector2, Vector3};
use cgmath::prelude::*;
use specs;

#[derive(Debug,Clone)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

impl Dimensions {
    pub fn world_to_clip(&self) -> Matrix4<f32> {
        Matrix4::from_translation(Vector3::new(-1.0, -1.0, 0.0)) *
        Matrix4::from_nonuniform_scale(2.0 / (self.width as f32), 2.0 / (self.height as f32), 1.0)
    }
}

#[derive(Debug,Clone)]
pub struct Position {
    pub position: Point2<f32>,
    pub orient: Rad<f32>,
    pub scale: f32,
}

impl Position {
    pub fn new(x: f32, y: f32, angle: Rad<f32>, scale: f32) -> Position {
        Position {
            position: Point2::new(x, y),
            orient: angle,
            scale: scale,
        }
    }

    pub fn model_to_world(&self) -> Matrix4<f32> {
        Matrix4::from_translation(Vector3::new(self.position.x, self.position.y, 0.0)) *
        Matrix4::from_nonuniform_scale(self.scale, self.scale, 1.0) *
        Matrix4::from_angle_z(self.orient)
    }
}

impl specs::Component for Position {
    type Storage = specs::VecStorage<Position>;
}

#[derive(Debug)]
pub struct Velocity {
    pub velocity: Vector2<f32>,
    pub angular_velocity: Rad<f32>,
}

impl Velocity {
    pub fn new() -> Velocity {
        Velocity {
            velocity: Vector2::zero(),
            angular_velocity: Rad::zero(),
        }
    }
}

impl specs::Component for Velocity {
    type Storage = specs::VecStorage<Velocity>;
}
