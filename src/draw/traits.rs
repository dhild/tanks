use draw::ColorFormat;
use game::{QuitStatus, TankControls};
use gfx;

pub trait Window<D: gfx::Device, F: gfx::Factory<D::Resources>> {
    fn swap_window(&mut self);
    fn poll_events(&mut self) -> Option<QuitStatus>;

    fn create_buffers(&mut self, count: usize) -> Vec<D::CommandBuffer>;
    fn set_controls(&mut self, controls: TankControls);

    fn get_viewport_size(&mut self) -> (u32, u32);
    fn get_device(&mut self) -> &mut D;
    fn get_factory(&mut self) -> &mut F;
    fn get_rtv(&mut self) -> gfx::handle::RenderTargetView<D::Resources, ColorFormat>;
}
