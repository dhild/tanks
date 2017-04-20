use draw::*;
use engine::{EncoderQueue, GameFunctions, RunStatus};
use gfx;
use physics::*;
use projectile;
use specs;
use tank;
use terrain;

mod controls;
mod player;
mod state;

pub use self::controls::TankControls;
pub use self::player::{Player, Players};
pub use self::state::ActivePlayer;

#[derive(Debug)]
pub struct TanksGame {
    width: usize,
    height: usize,

    fire_system: Option<projectile::FiringSystem>,
}

impl TanksGame {
    pub fn new() -> (TanksGame, TankControls) {
        let (fire_control, fire_system) = projectile::FireControl::new();
        (TanksGame {
             width: 1000,
             height: 500,

             fire_system: Some(fire_system),
         },
         TankControls::new(fire_control))
    }
}

impl<D, F> GameFunctions<D, F, ColorFormat> for TanksGame
    where D: 'static + gfx::Device,
          D::CommandBuffer: Send,
          F: gfx::Factory<D::Resources>
{
    fn setup_world(&mut self, world: &mut specs::World) {
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Mass>();
        world.register::<tank::Tank>();
        world.register::<tank::Drawable>();
        world.register::<terrain::Drawable>();
        world.register::<projectile::Drawable>();
        world.register::<projectile::Projectile>();

        world.add_resource(Dimensions {
                               width: self.width,
                               height: self.height,
                           });
        world.add_resource(ActivePlayer::new());
        world.add_resource(terrain::generate(self.width, self.height, 10));
        world.create().with(terrain::Drawable::new()).build();
        Players::create(world, 4);
    }

    fn setup_planner(&mut self,
                     planner: &mut Planner,
                     encoder_queue: EncoderQueue<D>,
                     factory: &mut F,
                     rtv: gfx::handle::RenderTargetView<D::Resources, ColorFormat>) {
        let terrain = planner
            .mut_world()
            .read_resource_now::<terrain::Terrain>();
        let draw = DrawSystem::new(factory, rtv, encoder_queue, &terrain);

        let mut firing = None;
        ::std::mem::swap(&mut self.fire_system, &mut firing);
        let firing = firing.expect("Firing system has already been consumed!");

        planner.add_system(draw, "drawing", 10);
        planner.add_system(terrain::PreDrawSystem::new(), "draw-prep-terrain", 15);
        planner.add_system(tank::PreDrawSystem::new(), "draw-prep-tank", 15);
        planner.add_system(projectile::PreDrawSystem::new(), "draw-prep-projectile", 15);
        planner.add_system(InertiaSystem::new(), "inertia", 20);
        planner.add_system(GravitySystem::new(), "gravity", 25);
        planner.add_system(projectile::CollisionSystem::new(), "collision-projectile", 35);
        planner.add_system(firing, "firing", 40);
        planner.add_system(state::GameStateSystem::new(), "game-state", 50);
    }
    fn check_status(&mut self, _world: &mut specs::World) -> RunStatus {
        RunStatus::Running
    }
}
