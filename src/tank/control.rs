use cgmath::{Deg, Rad};
use cgmath::prelude::*;
use game::{ActivePlayer, Player};
use specs;
use std::sync::mpsc;
use tank::Tank;

#[derive(Debug)]
pub enum TankControl {
    AngleDecreasing,
    AngleIncreasing,
    AngleStop,
    PowerDecreasing,
    PowerIncreasing,
    PowerStop,
}

#[derive(Debug)]
pub struct TankControlSystem {
    player: Player,
    queue: mpsc::Receiver<TankControl>,
    angle_adjustment: Option<Rad<f32>>,
    power_adjustment: Option<f32>,
}

impl TankControlSystem {
    pub fn new(player: Player) -> (TankControlSystem, mpsc::Sender<TankControl>) {
        let (tx, rx) = mpsc::channel();
        (TankControlSystem {
             player: player,
             queue: rx,
             angle_adjustment: None,
             power_adjustment: None,
         },
         tx)
    }
}

impl<C> specs::System<C> for TankControlSystem {
    fn run(&mut self, arg: specs::RunArg, _: C) {
        let (mut tanks, active) =
            arg.fetch(|w| (w.write::<Tank>(), w.read_resource::<ActivePlayer>()));
        while let Ok(control) = self.queue.try_recv() {
            match control {
                TankControl::AngleDecreasing => self.angle_adjustment = Some(Rad::from(Deg(-0.5))),
                TankControl::AngleIncreasing => self.angle_adjustment = Some(Rad::from(Deg(0.5))),
                TankControl::AngleStop => self.angle_adjustment = None,
                TankControl::PowerDecreasing => self.power_adjustment = Some(-0.05),
                TankControl::PowerIncreasing => self.power_adjustment = Some(0.05),
                TankControl::PowerStop => self.power_adjustment = None,
            }
        }
        let player = match active.player() {
            Some(p) if p == self.player => p,
            _ => return, // Not our turn to fire
        };
        let mut tank = match tanks.get_mut(player.id()) {
            None => return,
            Some(t) => t,
        };
        if let Some(angle) = self.angle_adjustment {
            tank.barrel_orient += angle;
            tank.barrel_orient.normalize();
            debug!("Tank {} angle updated: {:?}",
                   player.player_number(),
                   tank.barrel_orient);
        }
        if let Some(power) = self.power_adjustment {
            tank.power_level += power;
            tank.power_level = tank.power_level.min(1.0);
            tank.power_level = tank.power_level.max(0.0);
            debug!("Tank {} power updated: {:?}",
                   player.player_number(),
                   tank.power_level);
        }

    }
}
