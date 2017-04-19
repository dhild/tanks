
use game::ActivePlayer;
use specs;
use std::sync::mpsc;
use tank::Tank;

#[derive(Debug,Clone)]
pub struct FireControl {
    queue: mpsc::Sender<()>,
}

impl FireControl {
    pub fn fire(&mut self) {
        if let Err(e) = self.queue.send(()) {
            warn!("Disconnected fire control: {}", e);
        }
    }
}

#[derive(Debug)]
pub struct FiringSystem {
    queue: mpsc::Receiver<()>,
}

impl FiringSystem {
    pub fn new() -> (FiringSystem, FireControl) {
        let (tx, rx) = mpsc::channel();
        (FiringSystem { queue: rx }, FireControl { queue: tx })
    }

    fn fire(&self, player_number: u8) {

    }
}

impl<C> specs::System<C> for FiringSystem {
    fn run(&mut self, arg: specs::RunArg, _: C) {
        let (tanks, firing) = arg.fetch(|w| (w.read::<Tank>(), w.read_resource::<ActivePlayer>()));
        if let Ok(()) = self.queue.try_recv() {
            if let Some(player_number) = firing.player_number() {

            }
        }
    }
}
