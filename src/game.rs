use draw::*;
use engine::{EncoderQueue, GameFunctions, RunStatus};
use gfx;
use specs;

#[derive(Debug)]
pub struct TanksGame;

impl<D, F> GameFunctions<D, F, ColorFormat> for TanksGame
    where D: 'static + gfx::Device,
          D::CommandBuffer: Send,
          F: gfx::Factory<D::Resources>
{
    fn setup_world(&mut self, world: &mut specs::World) {
        world.register::<Drawable>();
        world.register::<Position>();
    }
    fn setup_planner(&mut self,
                     planner: &mut specs::Planner<f32>,
                     encoder_queue: EncoderQueue<D>,
                     _factory: &mut F,
                     rtv: gfx::handle::RenderTargetView<D::Resources, ColorFormat>) {
        let ds = DrawSystem::new(rtv, encoder_queue);
        // Several system bounds are closely related:
        let pds = PreDrawSystem::new([10.0, 10.0]);

        planner.add_system(pds, "draw-prep", 15);
        planner.add_system(ds, "drawing", 10);
    }
    fn check_status(&mut self, _world: &mut specs::World) -> RunStatus {
        RunStatus::Running
    }
}
