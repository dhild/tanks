use projectile::Projectile;
use specs::{self, Join};
use tank::Tank;

#[derive(Debug)]
pub struct ActivePlayer {
    player: Option<u8>,
}

impl ActivePlayer {
    pub fn new() -> ActivePlayer {
        ActivePlayer { player: None }
    }
    pub fn player_number(&self) -> Option<u8> {
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
    remaining_players: Vec<u8>,
}

impl Turn {
    fn first() -> Turn {
        Turn {
            number: 0,
            remaining_players: Vec::new(),
        }
    }

    fn next<'a, I>(&mut self, tanks: I) -> Option<u8>
        where I: Iterator<Item = &'a Tank>
    {
        let mut v: Vec<u8> = tanks.map(|t| t.player_number).collect();
        if self.remaining_players.is_empty() {
            // Sort so the player order is in reverse
            v.sort_by(|a, b| b.cmp(a));
            self.number += 1;
            self.remaining_players = v;
            debug!("All tanks have fired, transitioned to next turn {:?}", self);
            self.remaining_players.pop()
        } else {
            while let Some(player_number) = self.remaining_players.pop() {
                if v.contains(&player_number) {
                    return Some(player_number);
                } else {
                    debug!("Skipping tank {} since it has been destroyed",
                           player_number);
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
        let (tanks, mut active) =
            arg.fetch(|w| (w.read::<Tank>(), w.write_resource::<ActivePlayer>()));
        let tank_count = (&tanks.check()).join().count();
        if tank_count <= 1 {
            // TODO: Handle case where all tanks are gone without a winner (tank_count == 0)
            self.state = GameState::WinnerDeclared;
            debug!("A winner has been declared!");
        } else {
            let next_tank = self.turn.next((&tanks).join());
            active.player = next_tank;
            debug!("Next tank to fire is {:?}", next_tank);
            if next_tank.is_some() {
                self.state = GameState::TankFiring;
            } else {
                warn!("Unable to determine next tank to fire");
            }
        }
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