use super::RunStatus;
use gfx;
use renderer::EncoderQueue;
use specs;
use super::Delta;

pub trait GameFunctions<D: gfx::Device, F: gfx::Factory<D::Resources>, ColorFormat>
    : Send {
    fn setup_world(&mut self, world: &mut specs::World, viewport_size: (u32, u32));
    fn setup_planner(&mut self,
                     planner: &mut specs::Planner<Delta>,
                     encoder_queue: EncoderQueue<D>,
                     factory: &mut F,
                     render_target_view: gfx::handle::RenderTargetView<D::Resources, ColorFormat>);
    fn check_status(&mut self, _world: &mut specs::World) -> RunStatus {
        RunStatus::Running
    }
}

pub trait WindowFunctions<D: gfx::Device> {
    fn swap_window(&mut self);
    fn poll_events(&mut self) -> RunStatus;
}

pub trait GameControls {
    fn fire(&mut self);

    fn angle_decrease(&mut self);
    fn angle_increase(&mut self);
    fn angle_stop(&mut self);
    fn power_increase(&mut self);
    fn power_decrease(&mut self);
    fn power_stop(&mut self);
}
