use cgmath;
use specs;

#[derive(Debug)]
pub struct Terrain {}

impl specs::Component for Terrain {
    type Storage = specs::HashMapStorage<Terrain>;
}
