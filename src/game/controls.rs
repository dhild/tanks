
use engine::GameControls;
use projectile::FireControl;

#[derive(Debug,Clone)]
pub struct TankControls {
    fire_control: FireControl,
}

impl TankControls {
    pub fn new(fc: FireControl) -> TankControls {
        TankControls { fire_control: fc }
    }
}

impl GameControls for TankControls {
    fn fire(&mut self) {
        self.fire_control.fire()
    }
}
