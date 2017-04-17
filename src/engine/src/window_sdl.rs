use super::RunStatus;
use game::{Game, GameFunctions, WindowFunctions};
use gfx;
use gfx_window_sdl;
use sdl2;

struct SDLWindow {
    window: sdl2::video::Window,
    event_pump: sdl2::EventPump,
}

impl WindowFunctions<gfx_window_sdl::Device> for SDLWindow {
    fn swap_window(&mut self) {
        self.window.gl_swap_window();
    }
    fn poll_events(&mut self) -> RunStatus {
        use sdl2::event::Event;
        use sdl2::keyboard::Keycode;
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return RunStatus::Quit,
                _ => (),
            }
        }
        RunStatus::Running
    }
}

pub fn run<CF, DF, GF>(title: &str, game_functions: GF) -> RunStatus
    where GF: 'static + Send + GameFunctions<gfx_window_sdl::Device, gfx_window_sdl::Factory, CF>,
          CF: Clone + gfx::format::Formatted,
          DF: gfx::format::Formatted,
          <CF as gfx::format::Formatted>::Surface: gfx::format::RenderSurface,
          <CF as gfx::format::Formatted>::Channel: gfx::format::RenderChannel,
          <DF as gfx::format::Formatted>::Surface: gfx::format::DepthSurface,
          <DF as gfx::format::Formatted>::Channel: gfx::format::RenderChannel
{
    let sdl_context = sdl2::init().expect("Unable to create SDL context");
    let video = sdl_context
        .video()
        .expect("Unable to initialize SDL video subsystem");
    let event_pump = sdl_context
        .event_pump()
        .expect("Unable to obtain event pump");

    let gl_attr = video.gl_attr();
    #[cfg(debug_assertions)]
    gl_attr.set_context_flags().debug().set();

    //gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 2);
    let mut builder = video.window(title, 1024, 728);
    builder.opengl();

    let (window, _gl_context, device, factory, rtv, _dsv) =
        gfx_window_sdl::init::<CF, DF>(builder).expect("Unable to create a window");

    let window = SDLWindow {
        window: window,
        event_pump: event_pump,
    };

    Game::new(device, factory, rtv).run(game_functions, window, |f| f.create_command_buffer())
}
