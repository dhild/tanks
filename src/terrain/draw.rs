use cgmath::Matrix4;
use draw::ColorFormat;
use gfx;
use physics::Dimensions;
use specs;
use terrain::Terrain;

#[derive(Debug)]
pub struct Drawable {
    bounds: Bounds,
    base: Base,
}

impl Drawable {
    pub fn new() -> Drawable {
        Drawable {
            bounds: Bounds { transform: [[0.0; 4]; 4] },
            base: Base { base_y: -1.0 },
        }
    }

    pub fn update(&mut self, world_to_clip: &Matrix4<f32>) {
        self.bounds.transform = (*world_to_clip).into();
    }
}

impl specs::Component for Drawable {
    type Storage = specs::HashMapStorage<Drawable>;
}

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "position",
    }

    constant Bounds {
        transform: [[f32; 4]; 4] = "transform",
    }

    constant Base {
        base_y: f32 = "base_y",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        base: gfx::ConstantBuffer<Base> = "Base",
        bounds: gfx::ConstantBuffer<Bounds> = "Bounds",
        out: gfx::RenderTarget<ColorFormat> = "out_color",
    }
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Vertex {
        Vertex { pos: [x, y] }
    }
}

const SHADER_VERT: &'static [u8] = include_bytes!("terrain.v.glsl");
const SHADER_GEOM: &'static [u8] = include_bytes!("terrain.g.glsl");
const SHADER_FRAG: &'static [u8] = include_bytes!("terrain.f.glsl");

pub struct DrawSystem<R: gfx::Resources> {
    bundle: gfx::pso::bundle::Bundle<R, pipe::Data<R>>,
}

impl<R: gfx::Resources> DrawSystem<R> {
    pub fn new<F>(factory: &mut F,
                  rtv: gfx::handle::RenderTargetView<R, ColorFormat>,
                  terrain: &Terrain)
                  -> DrawSystem<R>
        where F: gfx::Factory<R>
    {
        use gfx::traits::FactoryExt;
        let vertices: Vec<Vertex> = terrain
            .heightmap
            .iter()
            .enumerate()
            .map(|(i, h)| Vertex::new(i as f32, *h as f32))
            .collect();

        let shaders = {
            let vert = factory.create_shader_vertex(SHADER_VERT).unwrap();
            let geom = factory.create_shader_geometry(SHADER_GEOM).unwrap();
            let frag = factory.create_shader_pixel(SHADER_FRAG).unwrap();
            gfx::ShaderSet::Geometry(vert, geom, frag)
        };

        let pso = factory
            .create_pipeline_state(&shaders,
                                   gfx::Primitive::LineStrip,
                                   gfx::state::Rasterizer::new_fill(),
                                   pipe::new())
            .unwrap();

        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertices[..], ());
        let data = pipe::Data {
            vbuf: vbuf,
            base: factory.create_constant_buffer(1),
            bounds: factory.create_constant_buffer(1),
            out: rtv,
        };

        DrawSystem { bundle: gfx::Bundle::new(slice, pso, data) }
    }

    pub fn draw<C>(&self, drawable: &Drawable, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>
    {
        encoder.update_constant_buffer(&self.bundle.data.bounds, &drawable.bounds);
        encoder.update_constant_buffer(&self.bundle.data.base, &drawable.base);
        self.bundle.encode(encoder);
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
        let (mut terrain, dim) =
            arg.fetch(|w| (w.write::<Drawable>(), w.read_resource::<Dimensions>()));

        let world_to_clip = dim.world_to_clip();

        for t in (&mut terrain).join() {
            t.update(&world_to_clip);
        }
    }
}
