use specs;

mod draw;
mod firing;

pub use self::draw::{Drawable, DrawSystem, PreDrawSystem};
pub use self::firing::FiringSystem;

#[derive(Debug)]
pub struct Projectile;

impl specs::Component for Projectile {
    type Storage = specs::HashMapStorage<Projectile>;
}
