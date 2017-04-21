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
mod explosion;
mod game;
mod physics;
mod projectile;
mod tank;
mod terrain;

use draw::{ColorFormat, DepthFormat};
use game::{TankControls, TanksGame};

fn main() {
    configure_logging();
    debug!("Starting up....");

    let (game, controls) = TanksGame::new();
    engine::run::<ColorFormat, DepthFormat, TanksGame, TankControls>("Tanks", game, controls);
}

fn configure_logging() {
    use log::LogLevelFilter;
    use log4rs::append::console::ConsoleAppender;
    use log4rs::encode::pattern::PatternEncoder;
    use log4rs::config::{Appender, Config, Logger, Root};
    if log4rs::init_file("src/log4rs.yaml", Default::default()).is_ok() {
        return;
    }
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{M} {l} - {m}{n}")))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build("gfx_device_gl", LogLevelFilter::Info))
        .logger(Logger::builder().build("ticketed_lock", LogLevelFilter::Info))
        .build(Root::builder()
                   .appender("stdout")
                   .build(LogLevelFilter::Debug))
        .unwrap();

    if let Err(e) = log4rs::init_config(config) {
        println!("Unable to configure logging! {}", e);
    }
}
