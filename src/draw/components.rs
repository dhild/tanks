use cgmath::Point2;
use draw;
use specs;

#[derive(Debug,Clone)]
pub struct Position {
    pub position: Point2<f32>,
}
impl Position {
    pub fn new(x: f32, y: f32) -> Position {
        Position { position: Point2::new(x, y) }
    }
}

impl specs::Component for Position {
    type Storage = specs::VecStorage<Position>;
}

#[derive(Debug)]
pub enum Drawable {
    Tank(draw::tank::Drawable),
    Terrain(draw::terrain::Drawable),
}

impl specs::Component for Drawable {
    type Storage = specs::VecStorage<Drawable>;
}
