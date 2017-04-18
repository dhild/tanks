use cgmath::{Rad, Point2, Vector2};
use cgmath::prelude::*;
use specs;

#[derive(Debug,Clone)]
pub struct Position {
    pub position: Point2<f32>,
    pub orient: Rad<f32>,
}
impl Position {
    pub fn new(x: f32, y: f32) -> Position {
        Position {
            position: Point2::new(x, y),
            orient: Rad::zero(),
        }
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
