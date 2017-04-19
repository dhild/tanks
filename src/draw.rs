use engine::EncoderQueue;
use gfx;
use projectile;
use specs;
use tank;
use terrain;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;


pub struct DrawSystem<D: gfx::Device> {
    render_target_view: gfx::handle::RenderTargetView<D::Resources, ColorFormat>,
    tank_system: tank::DrawSystem<D::Resources>,
    terrain_system: terrain::DrawSystem<D>,
    projectile_system: projectile::DrawSystem<D::Resources>,
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
            projectile_system: projectile::DrawSystem::new(factory, rtv.clone()),
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
        let (tanks, terrain, projectiles) = arg.fetch(|w| {
                                                          (w.read::<tank::Drawable>(),
                                                           w.read::<terrain::Drawable>(),
                                                           w.read::<projectile::Drawable>())
                                                      });

        encoder.clear(&self.render_target_view, [0.0, 0.0, 0.0, 1.0]);

        for t in (&terrain).join() {
            self.terrain_system.draw(t, &mut encoder);
        }
        for t in (&tanks).join() {
            self.tank_system.draw(t, &mut encoder);
        }
        for p in (&projectiles).join() {
            self.projectile_system.draw(p, &mut encoder);
        }

        if let Err(e) = self.encoder_queue.sender.send(encoder) {
            warn!("Disconnected, cannot return encoder to mpsc: {}", e);
        };
    }
}
