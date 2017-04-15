use super::RunStatus;
use draw::{ColorFormat, DepthFormat};
use game::{Game, GameFunctions, WindowFunctions};
use gfx_device_gl;
use gfx_window_sdl;
use sdl2;

struct SDLWindow {
    window: sdl2::video::Window,
    event_pump: sdl2::EventPump,
    event_handler: Box<Fn(sdl2::event::Event) -> RunStatus>,
}

impl WindowFunctions<gfx_window_sdl::Device, gfx_window_sdl::Factory> for SDLWindow {
    fn create_command_buffer(&mut self,
                             factory: &mut gfx_window_sdl::Factory)
                             -> gfx_device_gl::CommandBuffer {
        factory.create_command_buffer()
    }
    fn swap_window(&mut self) {
        self.window.gl_swap_window();
    }
    fn poll_events(&mut self) -> RunStatus {
        for event in self.event_pump.poll_iter() {
            let status = (self.event_handler)(event);
            if status != RunStatus::Running {
                return status;
            }
        }
        RunStatus::Running
    }
}


pub fn run<E, GF>(title: &str, event_handler: E, game_functions: GF) -> RunStatus
    where E: 'static + Fn(sdl2::event::Event) -> RunStatus,
          GF: 'static + Send + GameFunctions
{
    let sdl_context = sdl2::init().expect("Unable to create SDL context");
    let video = sdl_context
        .video()
        .expect("Unable to initialize SDL video subsystem");
    let event_pump = sdl_context
        .event_pump()
        .expect("Unable to obtain event pump");

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 0);
    let mut builder = video.window(title, 1024, 728);
    builder.opengl();

    let (window, _gl_context, device, factory, rtv, _dsv) =
                                gfx_window_sdl::init::<ColorFormat, DepthFormat>(builder)
                                .expect("Unable to create a window");

    let window = SDLWindow {
        window: window,
        event_pump: event_pump,
        event_handler: Box::new(event_handler),
    };

    Game::new(device, factory, rtv).run(game_functions, window)
}
