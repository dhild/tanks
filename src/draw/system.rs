use draw::{ColorFormat, EncoderQueue};
use explosion;
use gfx;
use projectile;
use specs;
use tank;
use terrain;
use text;

pub struct DrawSystem<D: gfx::Device> {
    render_target_view: gfx::handle::RenderTargetView<D::Resources, ColorFormat>,
    tank_system: tank::DrawSystem<D::Resources>,
    terrain_system: terrain::DrawSystem<D::Resources>,
    projectile_system: projectile::DrawSystem<D::Resources>,
    explosion_system: explosion::DrawSystem<D::Resources>,
    text_system: text::DrawSystem<D::Resources>,
    encoder_queue: EncoderQueue<D>,
}

impl<D: gfx::Device> DrawSystem<D> {
    pub fn new<F>(factory: &mut F,
                  rtv: gfx::handle::RenderTargetView<D::Resources, ColorFormat>,
                  queue: EncoderQueue<D>,
                  terrain: &terrain::Terrain)
                  -> DrawSystem<D>
        where F: gfx::Factory<D::Resources>
    {
        DrawSystem {
            render_target_view: rtv.clone(),
            tank_system: tank::DrawSystem::new(factory, rtv.clone()),
            terrain_system: terrain::DrawSystem::new(factory, rtv.clone(), terrain),
            projectile_system: projectile::DrawSystem::new(factory, rtv.clone()),
            explosion_system: explosion::DrawSystem::new(factory, rtv.clone()),
            text_system: text::DrawSystem::new(factory, rtv.clone()),
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
        let (tanks, terrain, projectiles, explosives, texts) =
            arg.fetch(|w| {
                          (w.read::<tank::Drawable>(),
                           w.read::<terrain::Drawable>(),
                           w.read::<projectile::Drawable>(),
                           w.read::<explosion::Drawable>(),
                           w.read::<text::Drawable>())
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
        for e in (&explosives).join() {
            self.explosion_system.draw(e, &mut encoder);
        }
        for e in (&texts).join() {
            self.text_system.draw(e, &mut encoder);
        }

        if let Err(e) = self.encoder_queue.sender.send(encoder) {
            warn!("Disconnected, cannot return encoder to mpsc: {}", e);
        };
    }
}
