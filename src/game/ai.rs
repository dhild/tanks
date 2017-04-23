use game::{ActivePlayer, Player, TankControls};
use physics::Position;
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
             _target_pos: &Position,
             _self_pos: &Position)
             -> bool {
        // TODO: Align to target
        true
    }
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

        let self_position = match positions.get(self.player.id()) {
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

        if self.align(target, target_pos, self_position) {
            self.controls.fire();
        }
    }
}
