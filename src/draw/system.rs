use super::ColorFormat;
use cgmath::{Matrix4, Vector3};
use draw::components::Drawable;
use draw::tank::DrawSystem as TankSystem;
use draw::terrain::DrawSystem as TerrainSystem;
use engine::EncoderQueue;
use gfx;
use physics::Position;
use specs;
use tank::Tank;
use terrain::Terrain;

pub struct DrawSystem<D: gfx::Device> {
    render_target_view: gfx::handle::RenderTargetView<D::Resources, ColorFormat>,
    tank_system: TankSystem<D::Resources>,
    terrain_system: TerrainSystem<D>,
    encoder_queue: EncoderQueue<D>,
}

impl<D: gfx::Device> DrawSystem<D> {
    pub fn new(rtv: gfx::handle::RenderTargetView<D::Resources, ColorFormat>,
               queue: EncoderQueue<D>)
               -> DrawSystem<D> {
        DrawSystem {
            render_target_view: rtv.clone(),
            tank_system: TankSystem::new(rtv.clone()),
            terrain_system: TerrainSystem::new(rtv.clone()),
            encoder_queue: queue,
        }
    }

    pub fn create_terrain<F>(&mut self, factory: &mut F, terrain: &Terrain) -> Drawable
        where F: gfx::Factory<D::Resources>
    {
        let d = self.terrain_system.create(factory, terrain);
        Drawable::Terrain(d)
    }

    pub fn create_tank<F>(&mut self, factory: &mut F, color: [f32; 3]) -> Drawable
        where F: gfx::Factory<D::Resources>
    {
        let d = self.tank_system.create_tank(factory, color);
        Drawable::Tank(d)
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
                Drawable::Tank(ref d) => self.tank_system.draw(d, &mut encoder),
                Drawable::Terrain(ref d) => self.terrain_system.draw(d, &mut encoder),
            }
        }
        // TODO: Render based on the type of drawable...

        if let Err(e) = self.encoder_queue.sender.send(encoder) {
            warn!("Disconnected, cannot return encoder to mpsc: {}", e);
        };
    }
}

pub struct PreDrawSystem {
    world_to_clip: Matrix4<f32>,
}

impl PreDrawSystem {
    pub fn new(width: usize, height: usize) -> PreDrawSystem {
        let width = width as f32;
        let height = height as f32;

        let mat = Matrix4::from_translation(Vector3::new(-1.0, -1.0, 0.0)) *
                  Matrix4::from_nonuniform_scale(2.0 / width, 2.0 / height, 1.0);

        PreDrawSystem { world_to_clip: mat }
    }
}

impl<C> specs::System<C> for PreDrawSystem {
    fn run(&mut self, arg: specs::RunArg, _: C) {
        use specs::Join;
        let (mut drawables, positions, tanks, entities) =
            arg.fetch(|w| {
                          (w.write::<Drawable>(),
                           w.read::<Position>(),
                           w.read::<Tank>(),
                           w.entities())
                      });
        for (d, p, e) in (&mut drawables, &positions, &entities).join() {
            match *d {
                Drawable::Tank(ref mut d) => {
                    if let Some(tank) = tanks.get(e) {
                        d.update(&self.world_to_clip, p, tank)
                    } else {
                        warn!("tank::Drawable without a Tank component");
                    }
                }
                Drawable::Terrain(ref mut d) => d.update(&self.world_to_clip),
            }
        }
    }
}
