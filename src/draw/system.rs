use super::ColorFormat;
use cgmath::{Matrix4, Vector3};
use engine::EncoderQueue;
use gfx;
use physics::{Dimensions, Position};
use specs;
use tank;
use terrain;

pub struct DrawSystem<D: gfx::Device> {
    render_target_view: gfx::handle::RenderTargetView<D::Resources, ColorFormat>,
    tank_system: tank::DrawSystem<D::Resources>,
    terrain_system: terrain::DrawSystem<D>,
    encoder_queue: EncoderQueue<D>,
}

impl<D: gfx::Device> DrawSystem<D> {
    pub fn new<F>(factory: &mut F,
                  rtv: gfx::handle::RenderTargetView<D::Resources, ColorFormat>,
                  queue: EncoderQueue<D>)
                  -> DrawSystem<D>
        where F: gfx::Factory<D::Resources>
    {
        DrawSystem {
            render_target_view: rtv.clone(),
            tank_system: tank::DrawSystem::new(factory, rtv.clone()),
            terrain_system: terrain::DrawSystem::new(rtv.clone()),
            encoder_queue: queue,
        }
    }

    pub fn create_terrain<F>(&mut self,
                             factory: &mut F,
                             terrain: &terrain::Terrain)
                             -> terrain::Drawable
        where F: gfx::Factory<D::Resources>
    {
        self.terrain_system.create(factory, terrain)
    }
}

impl<D, C> specs::System<C> for DrawSystem<D>
    where D: gfx::Device,
          D::CommandBuffer: Send
{
    fn run(&mut self, arg: specs::RunArg, _: C) {
        use specs::Join;
        let mut encoder = self.encoder_queue.receiver.recv().unwrap();
        let (tanks, terrain) =
            arg.fetch(|w| (w.read::<tank::Drawable>(), w.read::<terrain::Drawable>()));

        encoder.clear(&self.render_target_view, [0.0, 0.0, 0.0, 1.0]);

        for t in (&terrain).join() {
            self.terrain_system.draw(t, &mut encoder);
        }
        for t in (&tanks).join() {
            self.tank_system.draw(t, &mut encoder);
        }

        if let Err(e) = self.encoder_queue.sender.send(encoder) {
            warn!("Disconnected, cannot return encoder to mpsc: {}", e);
        };
    }
}

#[derive(Debug)]
pub struct PreDrawSystem;

impl PreDrawSystem {
    pub fn new() -> PreDrawSystem {
        PreDrawSystem {}
    }
}

impl<C> specs::System<C> for PreDrawSystem {
    fn run(&mut self, arg: specs::RunArg, _: C) {
        use specs::Join;
        let (positions, mut terrains, mut dtanks, tanks, dim) =
            arg.fetch(|w| {
                          (w.read::<Position>(),
                           w.write::<terrain::Drawable>(),
                           w.write::<tank::Drawable>(),
                           w.read::<tank::Tank>(),
                           w.read_resource::<Dimensions>())
                      });

        let world_to_clip = Matrix4::from_translation(Vector3::new(-1.0, -1.0, 0.0)) *
                            Matrix4::from_nonuniform_scale(2.0 / (dim.width as f32),
                                                           2.0 / (dim.height as f32),
                                                           1.0);

        for t in (&mut terrains).join() {
            t.update(&world_to_clip);
        }
        for (p, d, t) in (&positions, &mut dtanks, &tanks).join() {
            d.update(&world_to_clip, p, t);
        }
    }
}
