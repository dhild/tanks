use explosion::Explosion;
use game::{Player, Players, QuitStatus};
use projectile::Projectile;
use specs::{self, Join};
use std::sync::mpsc;

#[derive(Debug)]
pub struct ActivePlayer {
    player: Option<Player>,
}

impl ActivePlayer {
    pub fn new() -> ActivePlayer {
        ActivePlayer { player: None }
    }
    pub fn player(&self) -> Option<Player> {
        self.player
    }
}

#[derive(Debug)]
pub enum GameState {
    TankFiring,
    ProjectilesTravelling,
    ProjectilesImpacting,
    CalculateNextPlayer,
    GameOver,
}

#[derive(Debug)]
struct Turn {
    number: u32,
    remaining_players: Vec<Player>,
}

impl Turn {
    fn first() -> Turn {
        Turn {
            number: 0,
            remaining_players: Vec::new(),
        }
    }

    fn next(&mut self, mut remaining: Vec<Player>) -> Option<Player> {
        if self.remaining_players.is_empty() {
            // Sort so the player order is in reverse
            remaining.sort_by(|a, b| b.player_number().cmp(&a.player_number()));
            self.number += 1;
            self.remaining_players = remaining;
            debug!("All tanks have fired, transitioned to next turn {:?}", self);
            self.remaining_players.pop()
        } else {
            while let Some(player) = self.remaining_players.pop() {
                if remaining.contains(&player) {
                    return Some(player);
                } else {
                    debug!("Skipping tank {:?} since it has been destroyed", player);
                }
            }
            None
        }
    }
}

#[derive(Debug)]
pub struct GameStateSystem {
    state: GameState,
    turn: Turn,
    result: mpsc::Sender<QuitStatus>,
}

impl<C> specs::System<C> for GameStateSystem {
    fn run(&mut self, arg: specs::RunArg, _: C) {
        use self::GameState::*;
        match self.state {
            TankFiring => self.firing(arg),
            ProjectilesTravelling => self.projectiles(arg),
            ProjectilesImpacting => self.exploding(arg),
            CalculateNextPlayer => self.calculate_next(arg),
            GameOver => arg.fetch(|_| ()),
        }
    }
}

impl GameStateSystem {
    pub fn new() -> (GameStateSystem, mpsc::Receiver<QuitStatus>) {
        let (tx, rx) = mpsc::channel();

        (GameStateSystem {
             state: GameState::CalculateNextPlayer,
             turn: Turn::first(),
             result: tx,
         },
         rx)
    }

    fn calculate_next(&mut self, arg: specs::RunArg) {
        arg.fetch(|w| {
            let players = w.read_resource_now::<Players>();
            let players = players.get_remaining(w);
            if players.is_empty() {
                self.state = GameState::GameOver;
                info!("All players were destroyed!");
                self.result
                    .send(QuitStatus::Draw { turn: self.turn.number })
                    .expect("Unable to send final status");
            } else if players.len() == 1 {
                self.state = GameState::GameOver;
                let player = players.first().unwrap().player_number();
                info!("Player {} is the winner after {} turns!",
                      player,
                      self.turn.number);
                self.result
                    .send(QuitStatus::PlayerWon {
                              player: player,
                              turn: self.turn.number,
                          })
                    .expect("Unable to send final status");
            } else {
                {
                    use tank::Tank;
                    use specs::Gate;
                    let tanks = w.read::<Tank>().pass();
                    for &p in &players {
                        info!("Player {} is at health {}",
                              p.player_number(),
                              tanks.get(p.id()).unwrap().health);
                    }
                }
                let mut active = w.write_resource_now::<ActivePlayer>();
                let next_tank = self.turn.next(players);
                active.player = next_tank;
                info!("Next tank to fire is {:?}", next_tank);
                if next_tank.is_some() {
                    self.state = GameState::TankFiring;
                } else {
                    warn!("Unable to determine next tank to fire");
                }
            }
        });
    }

    fn firing(&mut self, arg: specs::RunArg) {
        // Once a projectile appears, move to next state
        let (projectiles, mut firing) =
            arg.fetch(|w| (w.read::<Projectile>(), w.write_resource::<ActivePlayer>()));
        if !(&projectiles.check()).join().next().is_none() {
            firing.player = None;
            self.state = GameState::ProjectilesTravelling;
            debug!("Projectiles are now travelling!");
        }
    }

    fn projectiles(&mut self, arg: specs::RunArg) {
        // Once all projectiles are gone, move to next state
        let projectiles = arg.fetch(|w| w.read::<Projectile>());
        if (&projectiles.check()).join().next().is_none() {
            self.state = GameState::ProjectilesImpacting;
            debug!("Projectiles are done travelling, waiting for any explosions to be resolved");
        }
    }

    fn exploding(&mut self, arg: specs::RunArg) {
        // Once all explosions are gone, move to next state
        let explosives = arg.fetch(|w| w.read::<Explosion>());
        if (&explosives.check()).join().next().is_none() {
            self.state = GameState::CalculateNextPlayer;
            debug!("Explosions are done, waiting for next tank to be determined");
        }
    }
}
