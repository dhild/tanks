use cgmath;
use specs;

pub type Delta = f32;
pub type Point = cgmath::Point2<f32>;
pub type Rad = cgmath::Rad<f32>;
pub type Matrix4 = cgmath::Matrix4<f32>;

#[derive(Debug,Clone)]
pub struct Position {
    pub position: Point,
    pub orient: Rad,
    pub scale: f32,
}

impl Position {
    pub fn to_translation(&self) -> Matrix4 {
        let translate = cgmath::Vector3::new(self.position.x, self.position.y, 0.0);
        let trans: Matrix4 = Matrix4::from_translation(translate);
        let scale: Matrix4 = Matrix4::from_scale(self.scale);
        let rot: Matrix4 = Matrix4::from_angle_z(self.orient);
        trans * scale * rot
    }
}

impl specs::Component for Position {
    type Storage = specs::VecStorage<Position>;
}
