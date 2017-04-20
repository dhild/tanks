extern crate cgmath;
#[macro_use]
extern crate gfx;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate rand;
extern crate specs;

extern crate engine;

mod draw;
mod game;
mod physics;
mod projectile;
mod tank;
mod terrain;

use draw::{ColorFormat, DepthFormat};
use game::{TankControls, TanksGame};

fn main() {
    log4rs::init_file("src/log4rs.yaml", Default::default()).unwrap();
    debug!("Starting up....");

    let (game, controls) = TanksGame::new();
    engine::run::<ColorFormat, DepthFormat, TanksGame, TankControls>("Tanks", game, controls);
}
