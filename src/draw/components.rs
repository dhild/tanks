use cgmath::{Matrix4, Point2, Rad, Vector3};
use draw;
use specs;

#[derive(Debug,Clone)]
pub struct Position {
    pub position: Point2<f32>,
    pub orient: Rad<f32>,
    pub scale: f32,
}

impl Position {
    pub fn to_translation(&self) -> Matrix4<f32> {
        let translate = Vector3::new(self.position.x, self.position.y, 0.0);
        let trans = Matrix4::from_translation(translate);
        let scale = Matrix4::from_scale(self.scale);
        let rot = Matrix4::from_angle_z(self.orient);
        trans * scale * rot
    }
}

impl specs::Component for Position {
    type Storage = specs::VecStorage<Position>;
}

#[derive(Debug,Clone)]
pub enum Drawable {
    Flat(draw::flat::Drawable),
}

impl specs::Component for Drawable {
    type Storage = specs::VecStorage<Drawable>;
}
