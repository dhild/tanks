use draw::*;
use engine::{EncoderQueue, GameFunctions, RunStatus};
use gfx;
use physics::*;
use projectile;
use rand::{self, Rng};
use specs;
use tank;
use terrain;

mod state;

pub use self::state::ActivePlayer;

#[derive(Debug)]
pub struct TanksGame {
    width: usize,
    height: usize,
}

const COLORS: [[f32; 3]; 4] = [
    [1.0, 0.0, 0.0], // red
    [0.0, 0.0, 1.0], // blue
    [1.0, 1.0, 0.0], // yellow
    [0.8, 0.0, 0.8], // purple
];

impl TanksGame {
    pub fn new() -> TanksGame {
        TanksGame {
            width: 1000,
            height: 500,
        }
    }

    fn create_terrain<D, F>(&self,
                            planner: &mut Planner,
                            factory: &mut F,
                            draw: &mut DrawSystem<D>)
        where D: gfx::Device,
              F: gfx::Factory<D::Resources>
    {
        let world = planner.mut_world();
        let terrain = terrain::generate(self.width, self.height, 10);
        let drawable = draw.create_terrain(factory, &terrain);

        world.add_resource(terrain);
        world.create().with(drawable).build();
    }

    fn create_tanks(&self, planner: &mut Planner) {
        let world = planner.mut_world();
        let dx = self.width as f32 / ((COLORS.len() + 1) as f32);
        let mut rng = rand::thread_rng();
        for (i, color) in COLORS.iter().enumerate() {
            let x = (i as f32 * dx) + rng.gen_range(dx / 2.0, 3.0 * dx / 2.0);
            let drawable = tank::Drawable::new(*color);

            let terrain = world.read_resource_now::<terrain::Terrain>();
            let terrain_height = terrain.get_height(x);
            let normal_dir = terrain.get_normal_dir(x);

            world
                .create()
                .with(tank::Tank::new(i as u8 + 1))
                .with(drawable)
                .with(Position::new(x, terrain_height, normal_dir, 20.0))
                .with(Velocity::new())
                .build();
        }
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
        world.register::<tank::Tank>();
        world.register::<tank::Drawable>();
        world.register::<terrain::Drawable>();
        world.register::<projectile::Drawable>();

        world.add_resource(Dimensions {
                               width: self.width,
                               height: self.height,
                           });
        world.add_resource(ActivePlayer::new());
    }

    fn setup_planner(&mut self,
                     planner: &mut Planner,
                     encoder_queue: EncoderQueue<D>,
                     factory: &mut F,
                     rtv: gfx::handle::RenderTargetView<D::Resources, ColorFormat>) {
        let mut draw = DrawSystem::new(factory, rtv, encoder_queue);

        self.create_terrain(planner, factory, &mut draw);
        self.create_tanks(planner);

        planner.add_system(draw, "drawing", 10);
        planner.add_system(terrain::PreDrawSystem::new(), "draw-prep-terrain", 15);
        planner.add_system(tank::PreDrawSystem::new(), "draw-prep-tank", 15);
        planner.add_system(projectile::PreDrawSystem::new(), "draw-prep-projectile", 15);
        planner.add_system(InertiaSystem::new(), "inertia", 20);
        planner.add_system(SettleSystem::new(), "settle", 25);
        planner.add_system(state::GameStateSystem::new(), "game-state", 50);
    }
    fn check_status(&mut self, _world: &mut specs::World) -> RunStatus {
        RunStatus::Running
    }
}
