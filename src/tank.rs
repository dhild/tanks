use cgmath;
use specs;

#[derive(Debug)]
pub struct Tank {
    color: u32,
    barrel_orient: cgmath::Rad<f32>,
}

impl specs::Component for Tank {
    type Storage = specs::HashMapStorage<Tank>;
}
