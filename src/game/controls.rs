use engine::GameControls;
use projectile::FireControl;
use tank::TankController;

#[derive(Debug,Clone)]
pub struct TankControls {
    fire_control: FireControl,
    tank_control: TankController,
}

impl TankControls {
    pub fn new(fc: FireControl, tc: TankController) -> TankControls {
        TankControls {
            fire_control: fc,
            tank_control: tc,
        }
    }
}

impl GameControls for TankControls {
    fn fire(&mut self) {
        self.fire_control.fire()
    }

    fn angle_increase(&mut self) {
        self.tank_control.angle_increase()
    }
    fn angle_decrease(&mut self) {
        self.tank_control.angle_decrease()
    }
    fn power_increase(&mut self) {
        self.tank_control.power_increase()
    }
    fn power_decrease(&mut self) {
        self.tank_control.power_decrease()
    }
}
