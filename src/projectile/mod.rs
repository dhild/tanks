use specs;

#[derive(Debug)]
pub struct Projectile;

impl specs::Component for Projectile {
    type Storage = specs::HashMapStorage<Projectile>;
}
