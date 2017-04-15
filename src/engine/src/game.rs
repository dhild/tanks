use super::RunStatus;
use cgmath;
use components::*;
use draw::*;
use gfx;
use specs;
use std::time;

static DRAW_PRIORITY: i32 = 10;
static DRAW_PREP_PRIORITY: i32 = 15;

pub trait GameFunctions {
    fn extents(&mut self) -> cgmath::Point2<f32> {
        [1.0, 1.0].into()
    }
    fn setup_world(&mut self, _world: &mut specs::World) {}

    fn setup_flat_drawables(&mut self,
                            _factory: &mut FnMut(gfx::state::Rasterizer,
                                                 gfx::Primitive,
                                                 &[flat::Vertex])
                                                 -> flat::Drawable) {
    }

    fn setup_planner(&mut self, _planner: &mut specs::Planner<Delta>) {}
    fn check_status(&mut self, _world: &mut specs::World) -> RunStatus {
        RunStatus::Running
    }
}

pub trait WindowFunctions<D: gfx::Device, F: gfx::Factory<D::Resources>> {
    fn create_command_buffer(&mut self, factory: &mut F) -> D::CommandBuffer;
    fn swap_window(&mut self);
    fn poll_events(&mut self) -> RunStatus;
}

pub struct Game<D: gfx::Device, F: gfx::Factory<D::Resources>> {
    device: D,
    factory: F,
    rtv: gfx::handle::RenderTargetView<D::Resources, ColorFormat>,
}

impl<D, F> Game<D, F>
    where D: 'static + gfx::Device,
          D::CommandBuffer: 'static + Send,
          F: gfx::Factory<D::Resources>
{
    pub fn new(device: D,
               factory: F,
               rtv: gfx::handle::RenderTargetView<D::Resources, ColorFormat>)
               -> Game<D, F> {
        Game {
            device: device,
            factory: factory,
            rtv: rtv,
        }
    }

    pub fn run<GF, WF>(&mut self, mut game_functions: GF, mut window_functions: WF) -> RunStatus
        where GF: 'static + Send + GameFunctions,
              WF: WindowFunctions<D, F>
    {
        let (mut device_renderer, enc_queue) = DeviceRenderer::new(2, || {
            window_functions.create_command_buffer(&mut self.factory)
        });

        let rtv = self.rtv.clone();
        let extents = game_functions.extents();
        let mut plan = {
            let mut w = specs::World::new();
            w.register::<flat::Drawable>();
            w.register::<Position>();
            game_functions.setup_world(&mut w);
            let mut plan = specs::Planner::new(w);

            let mut ds = flat::DrawSystem::new(rtv, enc_queue);
            let pds = flat::PreDrawSystem::new(extents);

            game_functions.setup_flat_drawables(&mut |rast, prim, vert| {
                                                         ds.add_drawable(&mut self.factory,
                                                                         rast,
                                                                         prim,
                                                                         vert)
                                                     });
            game_functions.setup_planner(&mut plan);

            plan.add_system(pds, "draw-prep", DRAW_PREP_PRIORITY);
            plan.add_system(ds, "drawing", DRAW_PRIORITY);

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
