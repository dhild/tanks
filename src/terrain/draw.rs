use cgmath::Matrix4;
use draw::ColorFormat;
use gfx;
use physics::Dimensions;
use specs;
use terrain::Terrain;

#[derive(Debug)]
pub struct Drawable {
    bounds: Bounds,
}

impl Drawable {
    pub fn new() -> Drawable {
        Drawable { bounds: Bounds { transform: [[0.0; 4]; 4] } }
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

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        bounds: gfx::ConstantBuffer<Bounds> = "Bounds",
        out: gfx::RenderTarget<ColorFormat> = "out_color",
    }
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Vertex {
        Vertex { pos: [x, y] }
    }

    fn generate(terrain: &Terrain) -> Vec<Vertex> {
        terrain
            .heightmap
            .iter()
            .enumerate()
            .map(|(i, h)| {
                     vec![Vertex::new(i as f32, *h as f32),
                          Vertex::new(i as f32, 0.0)]
                 })
            .flat_map(|v| v.into_iter())
            .collect()
    }
}

const SHADER_VERT: &'static [u8] = include_bytes!("terrain.v.glsl");
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

        let vertices = Vertex::generate(terrain);

        let program = factory.link_program(SHADER_VERT, SHADER_FRAG).unwrap();

        let pso = factory
            .create_pipeline_from_program(&program,
                                          gfx::Primitive::TriangleStrip,
                                          gfx::state::Rasterizer::new_fill(),
                                          pipe::new())
            .unwrap();

        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertices[..], ());
        let data = pipe::Data {
            vbuf: vbuf,
            bounds: factory.create_constant_buffer(1),
            out: rtv,
        };

        DrawSystem { bundle: gfx::Bundle::new(slice, pso, data) }
    }

    pub fn draw<C>(&self, drawable: &Drawable, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>
    {
        encoder.update_constant_buffer(&self.bundle.data.bounds, &drawable.bounds);
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
