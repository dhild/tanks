use cgmath::{Deg, Rad};
use cgmath::prelude::*;
use game::ActivePlayer;
use specs;
use std::sync::mpsc;
use tank::Tank;

#[derive(Debug)]
enum TankControl {
    AngleDecrease,
    AngleIncrease,
    PowerDecrease,
    PowerIncrease,
}

#[derive(Debug,Clone)]
pub struct TankController {
    queue: mpsc::Sender<TankControl>,
}

impl TankController {
    pub fn new() -> (TankController, TankControlSystem) {
        let (tx, rx) = mpsc::channel();
        (TankController { queue: tx }, TankControlSystem { queue: rx })
    }

    pub fn angle_increase(&mut self) {
        if let Err(e) = self.queue.send(TankControl::AngleIncrease) {
            warn!("Disconnected fire control: {}", e);
        }
    }

    pub fn angle_decrease(&mut self) {
        if let Err(e) = self.queue.send(TankControl::AngleDecrease) {
            warn!("Disconnected fire control: {}", e);
        }
    }

    pub fn power_increase(&mut self) {
        if let Err(e) = self.queue.send(TankControl::PowerIncrease) {
            warn!("Disconnected fire control: {}", e);
        }
    }

    pub fn power_decrease(&mut self) {
        if let Err(e) = self.queue.send(TankControl::PowerDecrease) {
            warn!("Disconnected fire control: {}", e);
        }
    }
}

#[derive(Debug)]
pub struct TankControlSystem {
    queue: mpsc::Receiver<TankControl>,
}

impl<C> specs::System<C> for TankControlSystem {
    fn run(&mut self, arg: specs::RunArg, _: C) {
        let (mut tanks, active) =
            arg.fetch(|w| (w.write::<Tank>(), w.read_resource::<ActivePlayer>()));
        while let Ok(control) = self.queue.try_recv() {
            let player = match active.player() {
                None => continue,
                Some(p) => p,
            };
            let mut tank = match tanks.get_mut(player.id()) {
                None => continue,
                Some(t) => t,
            };
            match control {
                TankControl::AngleDecrease => tank.barrel_orient -= Rad::from(Deg(0.5)),
                TankControl::AngleIncrease => tank.barrel_orient += Rad::from(Deg(0.5)),
                TankControl::PowerDecrease => tank.power_level -= 0.05,
                TankControl::PowerIncrease => tank.power_level += 0.05,
            }
            tank.barrel_orient.normalize();
            tank.power_level = tank.power_level.min(1.0);
            tank.power_level = tank.power_level.max(0.0);

            debug!("Tank updated: {:?}", tank);
        }
    }
}
