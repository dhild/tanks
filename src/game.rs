use draw::*;
use engine::{EncoderQueue, GameFunctions, RunStatus};
use gfx;
use rand::{self, Rng};
use specs;
use tank::Tank;
use terrain::Terrain;

pub type Delta = f32;
pub type Planner = specs::Planner<Delta>;

#[derive(Debug)]
pub struct TanksGame;

fn create_terrain<D, F>(planner: &mut Planner,
                        factory: &mut F,
                        ds: &mut DrawSystem<D>)
                        -> specs::Entity
    where D: gfx::Device,
          F: gfx::Factory<D::Resources>
{
    let terrain = Terrain::generate(1000, 500, 10);
    let drawable = ds.create_terrain(factory, &terrain);

    planner
        .mut_world()
        .create_now()
        .with(terrain)
        .with(drawable)
        .build()
}

const COLORS: [[f32; 3]; 4] = [
    [1.0, 0.0, 0.0], // red
    [0.0, 0.0, 1.0], // blue
    [1.0, 1.0, 0.0], // yellow
    [0.8, 0.0, 0.8], // purple
];

fn create_tanks<D, F>(planner: &mut Planner, factory: &mut F, ds: &mut DrawSystem<D>)
    where D: gfx::Device,
          F: gfx::Factory<D::Resources>
{
    let dx = 1000.0 / (COLORS.len() as f32);
    let mut rng = rand::thread_rng();
    for (i, color) in COLORS.iter().enumerate() {
        let x = (i as f32 * dx) + rng.gen_range(dx / 4.0, 7.0 * dx / 4.0);
        let drawable = ds.create_tank(factory, *color);

        planner
            .mut_world()
            .create_now()
            .with(Tank::new())
            .with(drawable)
            .with(Position::new(x, 450.0))
            .build();
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
        world.register::<Terrain>();
        world.register::<Tank>();
    }

    fn setup_planner(&mut self,
                     planner: &mut Planner,
                     encoder_queue: EncoderQueue<D>,
                     factory: &mut F,
                     rtv: gfx::handle::RenderTargetView<D::Resources, ColorFormat>) {
        let mut ds = DrawSystem::new(rtv, encoder_queue);
        // Several system bounds are closely related:
        let pds = PreDrawSystem::new(1000, 500);

        let terrain_entity = create_terrain(planner, factory, &mut ds);

        create_tanks(planner, factory, &mut ds);

        planner.add_system(pds, "draw-prep", 15);
        planner.add_system(ds, "drawing", 10);
    }
    fn check_status(&mut self, _world: &mut specs::World) -> RunStatus {
        RunStatus::Running
    }
}
