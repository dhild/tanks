use draw::*;
use explosion;
use gfx;
use physics::*;
use projectile;
use specs;
use std::sync::mpsc;
use std::time;
use tank;
use terrain;
use text;

mod ai;
mod controls;
mod player;
mod state;

pub use self::ai::AiController;
pub use self::controls::TankControls;
pub use self::player::{Player, Players};
pub use self::state::ActivePlayer;

#[derive(Debug,PartialEq,Eq)]
pub enum QuitStatus {
    PlayerWon { player: u8, turn: u32 },
    Draw { turn: u32 },
    Quit,
}

pub fn run<W, D, F>(window: &mut W) -> QuitStatus
    where W: Window<D, F>,
          D: gfx::Device + 'static,
          F: gfx::Factory<D::Resources>,
          D::CommandBuffer: Send
{
    let (mut device_renderer, enc_queue) = DeviceRenderer::new(window.create_buffers(2));

    let mut w = specs::World::new();
    setup_world(&mut w, window.get_viewport_size());
    let mut plan = specs::Planner::new(w);
    let mut receiver = setup_planner(window, &mut plan, enc_queue);

    dispatch_loop(window, &mut device_renderer, plan, &mut receiver)
}

fn setup_world(world: &mut specs::World, viewport_size: (u32, u32)) {
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Mass>();
    world.register::<tank::Tank>();
    world.register::<tank::Drawable>();
    world.register::<terrain::Drawable>();
    world.register::<projectile::Drawable>();
    world.register::<projectile::Projectile>();
    world.register::<explosion::Explosion>();
    world.register::<explosion::Drawable>();
    world.register::<text::Text>();
    world.register::<text::Drawable>();

    let dimensions = Dimensions::new(viewport_size.0, viewport_size.1);
    world.add_resource(terrain::generate(&dimensions, 10));
    world.add_resource(dimensions);
    world.add_resource(ActivePlayer::new());
    world.create().with(terrain::Drawable::new()).build();
    Players::create(world, 4);
}

fn setup_planner<W, D, F>(window: &mut W,
                          planner: &mut Planner,
                          encoder_queue: EncoderQueue<D>)
                          -> mpsc::Receiver<QuitStatus>
    where W: Window<D, F>,
          D: gfx::Device + 'static,
          F: gfx::Factory<D::Resources>,
          D::CommandBuffer: Send
{
    let draw = {
        let terrain = planner
            .mut_world()
            .read_resource_now::<terrain::Terrain>();
        let rtv = window.get_rtv();
        DrawSystem::new(window.get_factory(), rtv, encoder_queue, &terrain)
    };

    let (game_state_system, results_receiver) = state::GameStateSystem::new();

    planner.add_system(draw, "drawing", 10);
    planner.add_system(terrain::PreDrawSystem::new(), "draw-prep-terrain", 15);
    planner.add_system(tank::PreDrawSystem::new(), "draw-prep-tank", 15);
    planner.add_system(projectile::PreDrawSystem::new(), "draw-prep-projectile", 15);
    planner.add_system(explosion::PreDrawSystem::new(), "draw-prep-explosion", 15);
    planner.add_system(text::PreDrawSystem::new(), "draw-prep-text", 15);
    planner.add_system(projectile::CollisionSystem::new(), "collide-projectile", 20);
    planner.add_system(InertiaSystem::new(), "inertia", 30);
    planner.add_system(GravitySystem::new(), "gravity", 35);
    planner.add_system(explosion::ExplosionSystem::new(), "explosion", 35);
    planner.add_system(game_state_system, "game-state", 50);

    let human_controls = create_controls(planner);
    window.set_controls(human_controls);

    results_receiver
}

fn create_controls(planner: &mut Planner) -> TankControls {
    let players = {
        planner
            .mut_world()
            .read_resource_now::<Players>()
            .to_vec()
    };
    let mut human_control = None;
    for player in players {
        let number = player.player_number();

        let (fire_system, fire_control) = projectile::FireControlSystem::new(player);
        let (tank_system, tank_control) = tank::TankControlSystem::new(player);
        let controls = TankControls::new(fire_control, tank_control);

        planner.add_system(fire_system, &format!("firing-{}", number), 60);
        planner.add_system(tank_system, &format!("tank-control-{}", number), 61);

        if number == 1 {
            human_control = Some(controls);
        } else {
            info!("Player {} is computer-controlled", number);
            let ai = AiController::new(player, controls);
            planner.add_system(ai, &format!("ai-{}", number), 70);
        }
    }
    human_control.unwrap()
}

fn dispatch_loop<W, D, F>(window: &mut W,
                          device_renderer: &mut DeviceRenderer<D>,
                          mut planner: specs::Planner<f32>,
                          game_state: &mut mpsc::Receiver<QuitStatus>)
                          -> QuitStatus
    where W: Window<D, F>,
          D: gfx::Device + 'static,
          F: gfx::Factory<D::Resources>,
          D::CommandBuffer: Send
{
    let mut last_time = time::Instant::now();
    loop {
        trace!("Dispatching systems");
        let elapsed = last_time.elapsed();
        let delta = elapsed.subsec_nanos() as f32 / 1e9 + elapsed.as_secs() as f32;
        last_time = time::Instant::now();

        planner.dispatch(delta);

        device_renderer.draw(window.get_device());
        window.swap_window();

        if let Some(quit_status) = window.poll_events() {
            return quit_status;
        }
        planner.wait();
        if let Ok(quit_status) = game_state.try_recv() {
            return quit_status;
        }
    }
}
