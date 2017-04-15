use super::ColorFormat;
use cgmath::{Matrix4, Point2};
use draw::components::{Drawable, Position};
use draw::flat::DrawSystem as FlatDrawSystem;
use engine::EncoderQueue;
use gfx;
use specs;

pub struct DrawSystem<D: gfx::Device> {
    render_target_view: gfx::handle::RenderTargetView<D::Resources, ColorFormat>,
    flat_system: FlatDrawSystem<D>,
    encoder_queue: EncoderQueue<D>,
}

impl<D: gfx::Device> DrawSystem<D> {
    pub fn new(rtv: gfx::handle::RenderTargetView<D::Resources, ColorFormat>,
               queue: EncoderQueue<D>)
               -> DrawSystem<D> {
        DrawSystem {
            render_target_view: rtv.clone(),
            flat_system: FlatDrawSystem::new(rtv.clone()),
            encoder_queue: queue,
        }
    }
}

impl<D, C> specs::System<C> for DrawSystem<D>
    where D: gfx::Device,
          D::CommandBuffer: Send
{
    fn run(&mut self, arg: specs::RunArg, _: C) {
        use specs::Join;
        let mut encoder = self.encoder_queue.receiver.recv().unwrap();
        let drawables = arg.fetch(|w| w.read::<Drawable>());

        encoder.clear(&self.render_target_view, [0.0, 0.0, 0.0, 1.0]);

        for d in (&drawables).join() {
            match *d {
                Drawable::Flat(ref d) => self.flat_system.draw(d, &mut encoder),
            }
        }
        // TODO: Render based on the type of drawable...

        if let Err(e) = self.encoder_queue.sender.send(encoder) {
            warn!("Disconnected, cannot return encoder to mpsc: {}", e);
        };
    }
}

pub struct PreDrawSystem {
    scale: Matrix4<f32>,
}

impl PreDrawSystem {
    pub fn new<P: Into<Point2<f32>>>(extents: P) -> PreDrawSystem {
        let extents = extents.into();
        let scale = 1.0 / extents;
        let scale = Matrix4::from_nonuniform_scale(scale.x, scale.y, 1.0);
        PreDrawSystem { scale: scale }
    }
}

impl<C> specs::System<C> for PreDrawSystem {
    fn run(&mut self, arg: specs::RunArg, _: C) {
        use specs::Join;
        let (mut drawables, positions) =
            arg.fetch(|w| (w.write::<Drawable>(), w.read::<Position>()));
        for (d, p) in (&mut drawables, &positions).join() {
            match *d {
                Drawable::Flat(ref mut d) => d.update(self.scale * p.to_translation()),
            }
        }
    }
}
