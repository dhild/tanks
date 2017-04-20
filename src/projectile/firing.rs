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
        let (tanks,
             mut projectiles,
             mut drawables,
             mut positions,
             mut velocities,
             mut mass,
             firing) = arg.fetch(|w| {
            (w.read::<Tank>(),
             w.write::<Projectile>(),
             w.write::<Drawable>(),
             w.write::<Position>(),
             w.write::<Velocity>(),
             w.write::<Mass>(),
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
                let power = 500.0 * tank.power_level;
                let vx = power * -tank.barrel_orient.sin();
                let vy = power * tank.barrel_orient.cos();
                let velocity = Velocity::from([vx, vy]);
                let position = Position::new(tank_pos.position.x, tank_pos.position.y,
                    tank.barrel_orient, 7.0);

                debug!("Angle: {:?}, Initial velocity: {:?}", tank.barrel_orient, velocity);

                let eid = arg.create_pure();
                projectiles.insert(eid, Projectile::new());
                drawables.insert(eid, Drawable::new());
                velocities.insert(eid, velocity);
                mass.insert(eid, Mass { mass: 75.0 });
                (eid, position)
            }; // Borrow released here, now we can insert:
            positions.insert(eid, position);
        }
    }
}
