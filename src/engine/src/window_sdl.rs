use draw::{ColorFormat, DepthFormat};
use gfx::handle::{DepthStencilView, RenderTargetView};
use gfx_window_sdl;
use sdl2;

pub type Device = gfx_window_sdl::Device;
pub type Factory = gfx_window_sdl::Factory;
pub type RTV = RenderTargetView<gfx_window_sdl::Resources, ColorFormat>;
pub type DSV = DepthStencilView<gfx_window_sdl::Resources, DepthFormat>;

pub struct SDLHandle {
    _sdl_context: sdl2::Sdl,
    video: sdl2::VideoSubsystem,
    event_pump: sdl2::EventPump,
}

impl SDLHandle {
    pub fn new() -> SDLHandle {
        let sdl_context = sdl2::init().expect("Unable to create SDL context");
        let video = sdl_context
            .video()
            .expect("Unable to initialize SDL video subsystem");
        let event_pump = sdl_context
            .event_pump()
            .expect("Unable to obtain event pump");
        SDLHandle {
            _sdl_context: sdl_context,
            video: video,
            event_pump: event_pump,
        }
    }

    pub fn window(&mut self, title: &str) {
        use gfx::Device;
        use sdl2::event::Event;
        use sdl2::keyboard::Keycode;

        let builder = self.window_builder(title);

        let (window, _gl_context, mut device, _factory, _rtv, _dsv) =
                            gfx_window_sdl::init::<ColorFormat, DepthFormat>(builder)
                            .expect("Unable to create a window");

        loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        return;
                    }
                    _ => {}
                }
            }
            window.gl_swap_window();
            device.cleanup();
        }
    }

    fn window_builder(&mut self, title: &str) -> sdl2::video::WindowBuilder {
        let (width, height) = {
            let display_count = self.video.num_video_displays().unwrap_or(1i32);
            if display_count == 1 {
                (1024, 768)
            } else {
                let bounds = self.video.display_bounds(1).unwrap();
                (bounds.width(), bounds.height())
            }
        };
        let gl_attr = self.video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 0);
        let mut builder = self.video.window(title, width, height);
        builder.opengl();
        builder
    }
}
