use game::{Player, Players};
use projectile::Projectile;
use specs::{self, Join};

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
enum GameState {
    TankFiring,
    ProjectilesTravelling,
    CalculateNextPlayer,
    WinnerDeclared,
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
}

impl<C> specs::System<C> for GameStateSystem {
    fn run(&mut self, arg: specs::RunArg, _: C) {
        use self::GameState::*;
        match self.state {
            TankFiring => self.firing(arg),
            ProjectilesTravelling => self.projectiles(arg),
            CalculateNextPlayer => self.calculate_next(arg),
            _ => (),
        }
    }
}

impl GameStateSystem {
    pub fn new() -> GameStateSystem {
        GameStateSystem {
            state: GameState::CalculateNextPlayer,
            turn: Turn::first(),
        }
    }

    fn calculate_next(&mut self, arg: specs::RunArg) {
        arg.fetch(|w| {
            let players = w.read_resource_now::<Players>();
            let players = players.get_remaining(w);
            if players.len() <= 1 {
                // TODO: Handle case where all tanks are gone without a winner
                self.state = GameState::WinnerDeclared;
                debug!("A winner has been declared!");
            }
            let mut active = w.write_resource_now::<ActivePlayer>();
            let next_tank = self.turn.next(players);
            active.player = next_tank;
            debug!("Next tank to fire is {:?}", next_tank);
            if next_tank.is_some() {
                self.state = GameState::TankFiring;
            } else {
                warn!("Unable to determine next tank to fire");
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
            self.state = GameState::CalculateNextPlayer;
            debug!("Projectiles are done travelling, waiting for next tank to be determined");
        }
    }
}
