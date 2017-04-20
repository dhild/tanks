use engine::GameControls;
use std::sync::mpsc;
use tank::TankControl;

#[derive(Debug,Clone)]
pub struct TankControls {
    fire_control: mpsc::Sender<()>,
    tank_control: mpsc::Sender<TankControl>,
}

impl TankControls {
    pub fn new(fc: mpsc::Sender<()>, tc: mpsc::Sender<TankControl>) -> TankControls {
        TankControls {
            fire_control: fc,
            tank_control: tc,
        }
    }

    fn tc(&mut self, value: TankControl) {
        if self.tank_control.send(value).is_err() {
            warn!("Controls disconnected");
        }
    }
}

impl GameControls for TankControls {
    fn fire(&mut self) {
        if self.fire_control.send(()).is_err() {
            warn!("Controls disconnected");
        }
    }
    fn angle_increase(&mut self) {
        self.tc(TankControl::AngleIncreasing)
    }
    fn angle_decrease(&mut self) {
        self.tc(TankControl::AngleDecreasing)
    }
    fn angle_stop(&mut self) {
        self.tc(TankControl::AngleStop)
    }
    fn power_increase(&mut self) {
        self.tc(TankControl::PowerIncreasing)
    }
    fn power_decrease(&mut self) {
        self.tc(TankControl::PowerDecreasing)
    }
    fn power_stop(&mut self) {
        self.tc(TankControl::PowerStop)
    }
}
