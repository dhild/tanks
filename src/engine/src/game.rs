use super::RunStatus;
use gfx;
use renderer::*;
use specs;
use std::time;
use traits::*;

pub struct GameLoop<D: gfx::Device, F: gfx::Factory<D::Resources>, ColorFormat> {
    device: D,
    factory: F,
    rtv: gfx::handle::RenderTargetView<D::Resources, ColorFormat>,
}

impl<D, F, ColorFormat> GameLoop<D, F, ColorFormat>
    where D: 'static + gfx::Device,
          D::CommandBuffer: 'static + Send,
          F: gfx::Factory<D::Resources>,
          ColorFormat: Clone
{
    pub fn new(device: D,
               factory: F,
               rtv: gfx::handle::RenderTargetView<D::Resources, ColorFormat>)
               -> GameLoop<D, F, ColorFormat> {
        GameLoop {
            device: device,
            factory: factory,
            rtv: rtv,
        }
    }

    pub fn run<GF, WF, CBF>(&mut self,
                            mut game_functions: GF,
                            mut window_functions: WF,
                            mut create_command_buffer: CBF,
                            viewport_size: (u32, u32))
                            -> RunStatus
        where GF: 'static + Send + GameFunctions<D, F, ColorFormat>,
              WF: WindowFunctions<D>,
              CBF: FnMut(&mut F) -> D::CommandBuffer
    {
        let (mut device_renderer, enc_queue) =
            DeviceRenderer::new(2, || create_command_buffer(&mut self.factory));

        let rtv = self.rtv.clone();
        let mut plan = {
            let mut w = specs::World::new();
            game_functions.setup_world(&mut w, viewport_size);
            let mut plan = specs::Planner::new(w);
            game_functions.setup_planner(&mut plan, enc_queue, &mut self.factory, rtv);
            plan
        };

        let mut last_time = time::Instant::now();
        let mut window_status = RunStatus::Running;
        let mut game_status = RunStatus::Running;
        while window_status == RunStatus::Running && game_status == RunStatus::Running {
            trace!("Dispatching systems");
            let elapsed = last_time.elapsed();
            let delta = elapsed.subsec_nanos() as f32 / 1e9 + elapsed.as_secs() as f32;
            last_time = time::Instant::now();
            plan.dispatch(delta);

            device_renderer.draw(&mut self.device);
            window_functions.swap_window();
            self.device.cleanup();

            window_status = window_functions.poll_events();
            game_status = game_functions.check_status(plan.mut_world());
        }
        self.device.cleanup();
        if game_status != RunStatus::Running {
            game_status
        } else {
            window_status
        }
    }
}
