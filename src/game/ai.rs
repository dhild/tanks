use cgmath::{Deg, Point2};
use cgmath::prelude::*;
use game::{ActivePlayer, Player, TankControls};
use physics::{GRAVITY, Position};
use projectile::{POWER_MIN, POWER_SCALE};
use specs::{self, Join};
use tank::Tank;

#[derive(Debug)]
enum TargetingState {
    WaitForTurn,
    SelectTarget,
    AlignToTarget(Point2<f32>),
    Fire,
}

#[derive(Debug)]
pub struct AiController {
    player: Player,
    controls: TankControls,
    state: TargetingState,
}

impl AiController {
    pub fn new(player: Player, controls: TankControls) -> AiController {
        AiController {
            player: player,
            controls: controls,
            state: TargetingState::WaitForTurn,
        }
    }

    fn wait_for_turn(&mut self, args: specs::RunArg) {
        let (tanks, active) = args.fetch(|w| (w.read::<Tank>(), w.read_resource::<ActivePlayer>()));
        if active.player() == Some(self.player) && tanks.get(self.player.id()).is_some() {
            self.state = TargetingState::SelectTarget;
        }
    }

    fn select_target(&mut self, args: specs::RunArg) {
        let (tanks, positions, entities) =
            args.fetch(|w| (w.read::<Tank>(), w.read::<Position>(), w.entities()));
        if tanks.get(self.player.id()).is_some() {
            // Find a tank to target. For now this is simply the first tank we can find.
            for (_, p, e) in (&tanks.check(), &positions, &entities).join() {
                if self.player.id() == e {
                    continue;
                }
                debug!("Picking target {:?}", e);
                self.state = TargetingState::AlignToTarget(p.position);
            }
            return;
        }
        self.state = TargetingState::WaitForTurn;
    }

    fn align_to_target(&mut self, args: specs::RunArg, target: Point2<f32>) {
        let (tanks, positions) = args.fetch(|w| (w.read::<Tank>(), w.read::<Position>()));
        let ai_tank = match tanks.get(self.player.id()) {
            Some(t) => t,
            None => return, // This tank doesn't exist anymore...
        };
        let ai_position = match positions.get(self.player.id()) {
            Some(t) => t,
            None => return, // This tank doesn't exist anymore...
        };

        let distance = target - ai_position.position;
        trace!("Aiming for {:?}", distance);

        if ai_tank.barrel_orient <= Deg::zero() && distance.x > 0.0 {
            trace!("Reversing target angle");
            self.controls.angle_increase();
            return;
        } else if ai_tank.barrel_orient >= Deg::zero() && distance.x < 0.0 {
            trace!("Reversing target angle");
            self.controls.angle_decrease();
            return;
        }

        let (y_end, peaked) =
            calc_end_y(distance.x, ai_tank.power_level, ai_tank.barrel_orient, 75.0);
        let y_diff = y_end - distance.y;

        trace!("y_diff: {}, peaked? {}", y_diff, peaked);

        if !peaked || y_end < -10000.0 {
            trace!("Won't make it to target, changing angles");
            if distance.x < 0.0 {
                self.controls.angle_decrease();
            } else {
                self.controls.angle_increase();
            }
            self.controls.power_increase();
            return;
        }

        let mut changed = false;

        let (y2, p2) = calc_end_y(distance.x,
                                  ai_tank.power_level + 0.05,
                                  ai_tank.barrel_orient,
                                  75.0);
        if p2 && (y2 - distance.y).abs() < y_diff.abs() && ai_tank.power_level < 1.0 {
            self.controls.power_increase();
            changed = true;
        } else {
            let (y2, p2) = calc_end_y(distance.x,
                                      ai_tank.power_level - 0.05,
                                      ai_tank.barrel_orient,
                                      75.0);
            if p2 && (y2 - distance.y).abs() < y_diff.abs() && ai_tank.power_level > 0.0 {
                self.controls.power_decrease();
                changed = true;
            } else {
                self.controls.power_stop();
            }
        }

        let (y2, p2) = calc_end_y(distance.x,
                                  ai_tank.power_level,
                                  ai_tank.barrel_orient + Deg(0.5),
                                  75.0);
        if !changed && p2 && (y2 - distance.y).abs() < y_diff.abs() {
            self.controls.angle_increase();
            changed = true;
        } else {
            let (y2, p2) = calc_end_y(distance.x,
                                      ai_tank.power_level,
                                      ai_tank.barrel_orient - Deg(0.5),
                                      75.0);
            if !changed && p2 && (y2 - distance.y).abs() < y_diff.abs() {
                self.controls.angle_decrease();
                changed = true;
            } else {
                self.controls.angle_stop();
            }
        }

        if !changed {
            self.state = TargetingState::Fire;
        }
    }

    fn fire(&mut self, args: specs::RunArg) {
        let (tanks, active) = args.fetch(|w| (w.read::<Tank>(), w.read_resource::<ActivePlayer>()));
        // Double-check that it is indeed our turn to fire
        if active.player() == Some(self.player) && tanks.get(self.player.id()).is_some() {
            self.controls.fire();
        }
        self.state = TargetingState::WaitForTurn;
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
        match self.state {
            TargetingState::WaitForTurn => self.wait_for_turn(args),
            TargetingState::SelectTarget => self.select_target(args),
            TargetingState::AlignToTarget(pos) => self.align_to_target(args, pos),
            TargetingState::Fire => self.fire(args),
        }
    }
}
