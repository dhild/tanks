use draw;
use specs;

#[derive(Debug)]
pub enum Drawable {
    Tank(draw::tank::Drawable),
    Terrain(draw::terrain::Drawable),
}

impl specs::Component for Drawable {
    type Storage = specs::VecStorage<Drawable>;
}
