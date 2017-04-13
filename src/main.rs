#[macro_use]
extern crate log;
extern crate log4rs;

extern crate engine;

fn main() {
    log4rs::init_file("src/log4rs.yaml", Default::default()).unwrap();
    debug!("Starting up....");

    engine::create("Example");
}
