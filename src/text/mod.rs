use cgmath::Point2;
use specs;

mod draw;

pub use self::draw::{Drawable, DrawSystem, PreDrawSystem};

pub struct Text {
    pub text: String,
    pub screen_position: Point2<f32>,
    pub height: f32,
}

impl specs::Component for Text {
    type Storage = specs::HashMapStorage<Text>;
}
