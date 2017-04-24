use specs;

mod collision;
mod draw;
mod firing;

pub use self::collision::CollisionSystem;
pub use self::draw::{Drawable, DrawSystem, PreDrawSystem};
pub use self::firing::{FireControlSystem, POWER_MIN, POWER_SCALE};

#[derive(Debug)]
pub struct Projectile;

impl Projectile {
    pub fn new() -> Projectile {
        Projectile {}
    }
}

impl specs::Component for Projectile {
    type Storage = specs::HashMapStorage<Projectile>;
}
