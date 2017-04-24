use cgmath::Deg;
use cgmath::prelude::*;
use game::{ActivePlayer, Player, TankControls};
use physics::{GRAVITY, Position};
use projectile::{POWER_MIN, POWER_SCALE};
use specs::{self, Join};
use tank::Tank;

#[derive(Debug)]
pub struct AiController {
    player: Player,
    controls: TankControls,
    current_target: Option<specs::Entity>,
}

impl AiController {
    pub fn new(player: Player, controls: TankControls) -> AiController {
        AiController {
            player: player,
            controls: controls,
            current_target: None,
        }
    }

    fn align(&mut self,
             _target: specs::Entity,
             target_pos: &Position,
             ai_tank: &Tank,
             ai_pos: &Position) {
        let distance = target_pos.position - ai_pos.position;
        debug!("Aiming for {:?}", distance);
        let (y_end, peaked) =
            calc_end_y(distance.x, ai_tank.power_level, ai_tank.barrel_orient, 75.0);

        if !peaked {
            if distance.x < 0.0 {
                self.controls.angle_increase();
            } else {
                self.controls.angle_decrease();
            }
            self.controls.power_increase();
            return;
        }

        let y_diff = y_end - distance.y;
        let mut changed = false;

        let (y2, p2) = calc_end_y(distance.x,
                                  ai_tank.power_level + 0.05,
                                  ai_tank.barrel_orient,
                                  75.0);
        if p2 && (y2 - distance.y).abs() < y_diff {
            self.controls.power_increase();
            changed = true;
        } else {
            let (y2, p2) = calc_end_y(distance.x,
                                      ai_tank.power_level - 0.05,
                                      ai_tank.barrel_orient,
                                      75.0);
            if p2 && (y2 - distance.y).abs() < y_diff {
                self.controls.power_decrease();
                changed = true;
            } else {
                self.controls.angle_stop();
            }
        }

        let (y2, p2) = calc_end_y(distance.x,
                                  ai_tank.power_level,
                                  ai_tank.barrel_orient + Deg(0.5),
                                  75.0);
        if p2 && (y2 - distance.y).abs() < y_diff {
            self.controls.angle_increase();
            changed = true;
        } else {
            let (y2, p2) = calc_end_y(distance.x,
                                      ai_tank.power_level,
                                      ai_tank.barrel_orient - Deg(0.5),
                                      75.0);
            if p2 && (y2 - distance.y).abs() < y_diff {
                self.controls.angle_decrease();
                changed = true;
            } else {
                self.controls.power_stop();
            }
        }

        if !changed {
            self.controls.fire();
        }
    }
}

fn calc_end_y(x: f32, power_level: f32, theta: Deg<f32>, mass: f32) -> (f32, bool) {
    let v = POWER_MIN + POWER_SCALE * power_level;
    let t = x / (v * theta.sin());
    let y_end = 0.5 * mass * GRAVITY * t * t + v * t * theta.cos();
    let v_y_end = mass * GRAVITY * t + v * theta.cos();
    (y_end, v_y_end < 0.0)
}

impl<C> specs::System<C> for AiController {
    fn run(&mut self, args: specs::RunArg, _: C) {
        let (tanks, positions, entities, active) =
            args.fetch(|w| {
                           (w.read::<Tank>(),
                            w.read::<Position>(),
                            w.entities(),
                            w.read_resource::<ActivePlayer>())
                       });
        match active.player() {
            Some(p) if p == self.player => (),
            _ => return, // Not our turn!
        }

        let ai_tank = match tanks.get(self.player.id()) {
            Some(t) => t,
            None => return, // This tank doesn't exist anymore...
        };
        let ai_position = match positions.get(self.player.id()) {
            Some(t) => t,
            None => return, // This tank doesn't exist anymore...
        };

        let target = match self.current_target {
            Some(id) if tanks.get(id).is_some() => id, // Stay on target if it is still alive
            _ => {
                // Find a tank to target. For now this is simply the first tank we can find.
                for (_, e) in (&tanks.check(), &entities).join() {
                    if self.player.id() == e {
                        continue;
                    }
                    self.current_target = Some(e);
                    break;
                }
                match self.current_target {
                    Some(id) => id,
                    None => return, // All targets are gone
                }
            }
        };
        let target_pos = positions
            .get(target)
            .expect("Missing Position component for Tank");
        debug!("Targeting {:?} at {:?}", target, target_pos);

        self.align(target, target_pos, ai_tank, ai_position)
    }
}
