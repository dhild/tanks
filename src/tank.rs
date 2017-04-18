use cgmath::{Deg, Rad};
use cgmath::prelude::*;
use rand::{self, Rng};
use specs;

#[derive(Debug)]
pub struct Tank {
    pub barrel_orient: Rad<f32>,
    pub tank_orient: Rad<f32>,
}

impl Tank {
    pub fn new() -> Tank {
        let mut rng = rand::thread_rng();
        Tank {
            barrel_orient: Rad::from(Deg(rng.gen_range(-45.0, 45.0))),
            tank_orient: Rad::zero(),
        }
    }
}

impl specs::Component for Tank {
    type Storage = specs::HashMapStorage<Tank>;
}
