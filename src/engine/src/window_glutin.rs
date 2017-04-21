use super::RunStatus;
use game::GameLoop;
use gfx;
use gfx_device_gl;
use gfx_window_glutin;
use glutin;
use traits::*;

struct GlutinWindow<G: GameControls> {
    window: glutin::Window,
    game_controls: G,
}

impl<G: GameControls> WindowFunctions<gfx_device_gl::Device> for GlutinWindow<G> {
    fn swap_window(&mut self) {
        self.window
            .swap_buffers()
            .expect("Unable to swap buffers")
    }
    fn poll_events(&mut self) -> RunStatus {
        use glutin::Event::*;
        use glutin::VirtualKeyCode::*;
        use glutin::ElementState::*;
        for event in self.window.poll_events() {
            match event {
                Closed |
                KeyboardInput(_, _, Some(Escape)) => return RunStatus::Quit,
                KeyboardInput(Pressed, _, Some(Space)) => self.game_controls.fire(),
                KeyboardInput(Pressed, _, Some(Left)) => self.game_controls.angle_increase(),
                KeyboardInput(Pressed, _, Some(Right)) => self.game_controls.angle_decrease(),
                KeyboardInput(Released, _, Some(Left)) |
                KeyboardInput(Released, _, Some(Right)) => self.game_controls.angle_stop(),
                KeyboardInput(Pressed, _, Some(Up)) => self.game_controls.power_increase(),
                KeyboardInput(Pressed, _, Some(Down)) => self.game_controls.power_decrease(),
                KeyboardInput(Released, _, Some(Up)) |
                KeyboardInput(Released, _, Some(Down)) => self.game_controls.power_stop(),
                _ => (),
            }
        }
        RunStatus::Running
    }
}

pub fn run<CF, DF, GF, GC>(title: &str, game_functions: GF, game_controls: GC) -> RunStatus
    where GC: GameControls,
          GF: 'static + Send + GameFunctions<gfx_device_gl::Device, gfx_device_gl::Factory, CF>,
          CF: Clone + gfx::format::Formatted,
          DF: gfx::format::Formatted,
          <CF as gfx::format::Formatted>::Surface: gfx::format::RenderSurface,
          <CF as gfx::format::Formatted>::Channel: gfx::format::RenderChannel,
          <DF as gfx::format::Formatted>::Surface: gfx::format::DepthSurface,
          <DF as gfx::format::Formatted>::Channel: gfx::format::RenderChannel
{
    let builder = glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title(title)
        .with_gl(glutin::GlRequest::GlThenGles {
                     opengl_version: (4, 0),
                     opengles_version: (3, 2),
                 })
        .with_vsync();

    let (window, device, factory, rtv, _dsv) = gfx_window_glutin::init::<CF, DF>(builder);

    let viewport_size = window.get_inner_size_pixels().unwrap_or((1024, 768));

    let window_adapter = GlutinWindow {
        window: window,
        game_controls: game_controls,
    };

    GameLoop::new(device, factory, rtv).run(game_functions,
                                            window_adapter,
                                            |f| f.create_command_buffer(),
                                            viewport_size)
}
