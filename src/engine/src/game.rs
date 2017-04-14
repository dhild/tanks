use super::RunStatus;
use cgmath;
use components::*;
use draw::*;
use gfx;
use specs;
use std::{thread, time};
use std::sync::mpsc;

static DRAW_PRIORITY: i32 = 10;
static DRAW_PREP_PRIORITY: i32 = 15;

pub trait GameFunctions {
    fn extents(&mut self) -> cgmath::Point2<f32> {
        [1.0, 1.0].into()
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

pub struct Game<D: gfx::Device> {
    device: D,
    rtv: gfx::handle::RenderTargetView<D::Resources, ColorFormat>,
}

impl<D> Game<D>
    where D: 'static + gfx::Device,
          D::CommandBuffer: 'static + Send
{
    pub fn new(device: D,
               rtv: gfx::handle::RenderTargetView<D::Resources, ColorFormat>)
               -> Game<D> {
        Game {
            device: device,
            rtv: rtv,
        }
    }

    pub fn run<F, GF, WF>(mut self,
                          mut factory: F,
                          mut game_functions: GF,
                          mut window_functions: WF)
        where F: gfx::Factory<D::Resources>,
              GF: 'static + Send + GameFunctions,
              WF: WindowFunctions<D, F>
    {
        let (mut device_renderer, enc_queue) =
            DeviceRenderer::new(2, || window_functions.create_command_buffer(&mut factory));

        let rtv = self.rtv.clone();
        let extents = game_functions.extents();
        let create_planner = move || {
            let mut w = specs::World::new();
            w.register::<FlatDrawable>();
            w.register::<Position>();
            let mut plan = specs::Planner::new(w);

            let ds = flat::DrawSystem::new(rtv, enc_queue);
            let pds = flat::PreDrawSystem::new(extents);

            plan.add_system(pds, "draw-prep", DRAW_PREP_PRIORITY);
            plan.add_system(ds, "drawing", DRAW_PRIORITY);
            plan
        };

        let dispatcher = Dispatcher::new(create_planner, game_functions);
        loop {
            if window_functions.poll_events() != RunStatus::Running {
                break;
            }
            device_renderer.draw(&mut self.device);
            window_functions.swap_window();
            self.device.cleanup();
        }
        let guard = dispatcher.stop();
        // Draw, releasing a command buffer and avoiding deadlock.
        device_renderer.draw(&mut self.device);
        if guard.join().is_err() {
            warn!("Unclean exit from dispatcher thread");
        }
        self.device.cleanup();
    }
}

struct Dispatcher {
    stop_signal: mpsc::Sender<()>,
    guard: thread::JoinHandle<()>,
}

impl Dispatcher {
    fn new<CP, GF>(create_planner: CP, mut game_functions: GF) -> Dispatcher
        where CP: 'static + Send + FnOnce() -> specs::Planner<Delta>,
              GF: 'static + Send + GameFunctions
    {
        let (stop_send, stop_recv) = mpsc::channel();

        let handle = thread::Builder::new()
            .name("dispatcher".to_string())
            .spawn(move || {
                let mut last_time = time::Instant::now();
                let mut plan = create_planner();

                game_functions.setup_planner(&mut plan);

                while game_functions.check_status(plan.mut_world()) == RunStatus::Running {
                    if let Ok(()) = stop_recv.try_recv() {
                        debug!("Stop signal received");
                        break;
                    }
                    trace!("Dispatching systems");
                    let elapsed = last_time.elapsed();
                    let delta = elapsed.subsec_nanos() as f32 / 1e9 + elapsed.as_secs() as f32;
                    last_time = time::Instant::now();
                    plan.dispatch(delta);
                }
                debug!("Done with dispatch");
            })
            .unwrap();
        Dispatcher {
            stop_signal: stop_send,
            guard: handle,
        }
    }

    fn stop(self) -> thread::JoinHandle<()> {
        if self.stop_signal.send(()).is_err() {
            debug!("Dispatcher has already quit");
        }
        self.guard
    }
}
