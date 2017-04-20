use cgmath::prelude::*;
use game::ActivePlayer;
use physics::*;
use projectile::{Drawable, Projectile};
use specs;
use std::sync::mpsc;
use tank::Tank;

#[derive(Debug,Clone)]
pub struct FireControl {
    queue: mpsc::Sender<()>,
}

impl FireControl {
    pub fn new() -> (FireControl, FiringSystem) {
        let (tx, rx) = mpsc::channel();
        (FireControl { queue: tx }, FiringSystem { queue: rx })
    }

    pub fn fire(&mut self) {
        if let Err(e) = self.queue.send(()) {
            warn!("Disconnected fire control: {}", e);
        }
    }
}

#[derive(Debug)]
pub struct FiringSystem {
    queue: mpsc::Receiver<()>,
}

impl<C> specs::System<C> for FiringSystem {
    fn run(&mut self, arg: specs::RunArg, _: C) {
        let (tanks, mut projectiles, mut drawables, mut positions, mut velocities, firing) =
            arg.fetch(|w| {
                          (w.read::<Tank>(),
                           w.write::<Projectile>(),
                           w.write::<Drawable>(),
                           w.write::<Position>(),
                           w.write::<Velocity>(),
                           w.read_resource::<ActivePlayer>())
                      });
        while let Ok(()) = self.queue.try_recv() {
            let player = match firing.player() {
                None => continue,
                Some(p) => p,
            };
            let (eid, position) = {
                // Can't insert into positions while the positions borrow is active:
                let tank = match tanks.get(player.id()) {
                    None => continue,
                    Some(t) => t,
                };
                let tank_pos = match positions.get(player.id()) {
                    None => continue,
                    Some(p) => p,
                };
                let power = tank.power_level;
                let cos = tank.barrel_orient.cos();
                let sin = tank.barrel_orient.sin();
                let velocity = Velocity::from([power * cos, power * sin]);
                let position = Position::new(tank_pos.position.x, tank_pos.position.y,
                    tank.barrel_orient, 1.0);

                let eid = arg.create_pure();
                projectiles.insert(eid, Projectile::new());
                drawables.insert(eid, Drawable::new());
                velocities.insert(eid, velocity);
                (eid, position)
            }; // Borrow released here, now we can insert:
            positions.insert(eid, position);
        }
    }
}
