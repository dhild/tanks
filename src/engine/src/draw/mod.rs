use gfx;
use std::sync::mpsc;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub struct EncoderQueue<D: gfx::Device> {
    pub sender: mpsc::Sender<gfx::Encoder<D::Resources, D::CommandBuffer>>,
    pub receiver: mpsc::Receiver<gfx::Encoder<D::Resources, D::CommandBuffer>>,
}

pub struct DeviceRenderer<D: gfx::Device> {
    queue: EncoderQueue<D>,
}
