use draw::{ColorFormat, DepthFormat};
use draw::traits::*;
use game::{QuitStatus, TankControls};
use gfx;
use gfx_device_gl;
use gfx_window_glutin;
use glutin;

pub struct GlutinWindow {
    window: glutin::Window,
    device: gfx_device_gl::Device,
    factory: gfx_device_gl::Factory,
    rtv: gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>,
    _dsv: gfx::handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat>,
    controls: Option<TankControls>,
}

impl GlutinWindow {
    pub fn new() -> GlutinWindow {
        let builder = glutin::WindowBuilder::new()
            .with_title("Rusty Tanks!")
            .with_pixel_format(24, 8)
            .with_gl(glutin::GlRequest::GlThenGles {
                         opengles_version: (3, 0),
                         opengl_version: (4, 1),
                     });

        let (window, device, factory, rtv, _dsv) = gfx_window_glutin::init::<ColorFormat,
                                                                             DepthFormat>(builder);

        GlutinWindow {
            window: window,
            device: device,
            factory: factory,
            rtv: rtv,
            _dsv: _dsv,
            controls: None,
        }
    }
}

impl Window<gfx_device_gl::Device, gfx_device_gl::Factory> for GlutinWindow {
    fn swap_window(&mut self) {
        use gfx::Device;
        self.window
            .swap_buffers()
            .expect("Unable to swap buffers");
        self.device.cleanup();
    }
    fn create_buffers(&mut self, count: usize) -> Vec<gfx_device_gl::CommandBuffer> {
        let mut bufs = Vec::new();
        for _ in 0..count {
            bufs.push(self.factory.create_command_buffer());
        }
        bufs
    }
    fn set_controls(&mut self, controls: TankControls) {
        self.controls = Some(controls)
    }
    fn get_viewport_size(&mut self) -> (u32, u32) {
        self.window
            .get_inner_size_pixels()
            .unwrap_or((1024, 768))
    }
    fn get_device(&mut self) -> &mut gfx_device_gl::Device {
        &mut self.device
    }
    fn get_factory(&mut self) -> &mut gfx_device_gl::Factory {
        &mut self.factory
    }
    fn get_rtv(&mut self) -> gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat> {
        self.rtv.clone()
    }
    fn poll_events(&mut self) -> Option<QuitStatus> {
        use glutin::Event::*;
        use glutin::VirtualKeyCode::*;
        use glutin::ElementState::*;

        let controls = match self.controls {
            Some(ref mut c) => c,
            None => panic!("Controls have not been initialized"),
        };

        for event in self.window.poll_events() {
            match event {
                Closed |
                KeyboardInput(_, _, Some(Escape)) => return Some(QuitStatus::Quit),
                KeyboardInput(Pressed, _, Some(Space)) => controls.fire(),
                KeyboardInput(Pressed, _, Some(Left)) => controls.angle_increase(),
                KeyboardInput(Pressed, _, Some(Right)) => controls.angle_decrease(),
                KeyboardInput(Released, _, Some(Left)) |
                KeyboardInput(Released, _, Some(Right)) => controls.angle_stop(),
                KeyboardInput(Pressed, _, Some(Up)) => controls.power_increase(),
                KeyboardInput(Pressed, _, Some(Down)) => controls.power_decrease(),
                KeyboardInput(Released, _, Some(Up)) |
                KeyboardInput(Released, _, Some(Down)) => controls.power_stop(),
                _ => (),
            }
        }
        None
    }
}
