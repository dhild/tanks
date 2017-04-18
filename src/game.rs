use draw::*;
use engine::{EncoderQueue, GameFunctions, RunStatus};
use gfx;
use physics::*;
use rand::{self, Rng};
use specs;
use tank::Tank;
use terrain::Terrain;

#[derive(Debug)]
pub struct TanksGame {
    width: usize,
    height: usize,
    terrain_points: usize,
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
            terrain_points: 10,
        }
    }

    fn create_terrain<D, F>(&self,
                            planner: &mut Planner,
                            factory: &mut F,
                            draw: &mut DrawSystem<D>)
                            -> specs::Entity
        where D: gfx::Device,
              F: gfx::Factory<D::Resources>
    {
        let terrain = Terrain::generate(self.width, self.height, self.terrain_points);
        let drawable = draw.create_terrain(factory, &terrain);

        planner
            .mut_world()
            .create_now()
            .with(terrain)
            .with(drawable)
            .build()
    }
    fn create_tanks<D, F>(&self, planner: &mut Planner, factory: &mut F, draw: &mut DrawSystem<D>)
        where D: gfx::Device,
              F: gfx::Factory<D::Resources>
    {
        let dx = self.width as f32 / ((COLORS.len() + 1) as f32);
        let mut rng = rand::thread_rng();
        for (i, color) in COLORS.iter().enumerate() {
            let x = (i as f32 * dx) + rng.gen_range(dx / 2.0, 3.0 * dx / 2.0);
            let drawable = draw.create_tank(factory, *color);

            planner
                .mut_world()
                .create_now()
                .with(Tank::new())
                .with(drawable)
                .with(Position::new(x, self.height as f32 * 0.9))
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
        world.register::<Drawable>();
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Terrain>();
        world.register::<Tank>();
    }

    fn setup_planner(&mut self,
                     planner: &mut Planner,
                     encoder_queue: EncoderQueue<D>,
                     factory: &mut F,
                     rtv: gfx::handle::RenderTargetView<D::Resources, ColorFormat>) {
        let mut draw = DrawSystem::new(rtv, encoder_queue);
        let pds = PreDrawSystem::new(self.width, self.height);
        let inertia = InertiaSystem::new();

        let terrain_entity = self.create_terrain(planner, factory, &mut draw);
        self.create_tanks(planner, factory, &mut draw);

        let settle = SettleSystem::new(terrain_entity);

        planner.add_system(pds, "draw-prep", 15);
        planner.add_system(draw, "drawing", 10);
        planner.add_system(inertia, "inertia", 20);
        planner.add_system(settle, "settle", 25);
    }
    fn check_status(&mut self, _world: &mut specs::World) -> RunStatus {
        RunStatus::Running
    }
}
