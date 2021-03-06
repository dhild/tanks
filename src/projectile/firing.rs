use cgmath::prelude::*;
use game::{ActivePlayer, Player};
use physics::*;
use projectile::{Drawable, Projectile};
use specs;
use std::sync::mpsc;
use tank::Tank;

pub const POWER_MIN: f32 = 150.0;
pub const POWER_SCALE: f32 = 100.0;

#[derive(Debug)]
pub struct FireControlSystem {
    player: Player,
    queue: mpsc::Receiver<()>,
}

impl FireControlSystem {
    pub fn new(player: Player) -> (FireControlSystem, mpsc::Sender<()>) {
        let (tx, rx) = mpsc::channel();
        (FireControlSystem {
             player: player,
             queue: rx,
         },
         tx)
    }
}

impl<C> specs::System<C> for FireControlSystem {
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
        let player = match firing.player() {
            Some(p) if p == self.player => p,
            _ => return,
        };
        let mut inserted = false;
        while let Ok(()) = self.queue.try_recv() {
            if inserted {
                continue; // Eat up any remaining signals
            }
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
                let power = POWER_MIN + (POWER_SCALE * tank.power_level);
                let vx = power * tank.barrel_orient.sin();
                let vy = power * tank.barrel_orient.cos();
                let velocity = Velocity::from([vx, vy]);
                let position = Position::new(tank_pos.position.x, tank_pos.position.y,
                    tank.barrel_orient, 7.0);

                trace!("Angle: {:?}, Initial velocity: {:?}", tank.barrel_orient, velocity);

                let eid = arg.create_pure();
                projectiles.insert(eid, Projectile::new());
                drawables.insert(eid, Drawable::new());
                velocities.insert(eid, velocity);
                mass.insert(eid, Mass { mass: 75.0 });
                (eid, position)
            }; // Borrow released here, now we can insert:
            positions.insert(eid, position);
            inserted = true;
        }
    }
}
