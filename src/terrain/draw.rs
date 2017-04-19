
use cgmath::{Matrix4, Vector3};
use draw::ColorFormat;
use gfx;
use specs;
use terrain::Terrain;

#[derive(Debug)]
pub struct Drawable {
    bounds: Bounds,
    base: Base,
}

impl Drawable {
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

fn create_pso_bundle<F, R>(factory: &mut F,
                           out: gfx::handle::RenderTargetView<R, ColorFormat>,
                           rast: gfx::state::Rasterizer,
                           vertices: &[Vertex])
                           -> gfx::pso::bundle::Bundle<R, pipe::Data<R>>
    where R: gfx::Resources,
          F: gfx::Factory<R>
{
    use gfx::traits::FactoryExt;

    let shaders = {
        let vert = factory.create_shader_vertex(SHADER_VERT).unwrap();
        let geom = factory.create_shader_geometry(SHADER_GEOM).unwrap();
        let frag = factory.create_shader_pixel(SHADER_FRAG).unwrap();
        gfx::ShaderSet::Geometry(vert, geom, frag)
    };

    let pso = factory
        .create_pipeline_state(&shaders, gfx::Primitive::LineStrip, rast, pipe::new())
        .unwrap();

    let (vbuf, slice) = factory.create_vertex_buffer_with_slice(vertices, ());
    let data = pipe::Data {
        vbuf: vbuf,
        base: factory.create_constant_buffer(1),
        bounds: factory.create_constant_buffer(1),
        out: out,
    };
    gfx::Bundle::new(slice, pso, data)
}

pub struct DrawSystem<D: gfx::Device> {
    render_target_view: gfx::handle::RenderTargetView<D::Resources, ColorFormat>,
    bundle: Option<gfx::pso::bundle::Bundle<D::Resources, pipe::Data<D::Resources>>>,
}

impl<D: gfx::Device> DrawSystem<D> {
    pub fn new(rtv: gfx::handle::RenderTargetView<D::Resources, ColorFormat>) -> DrawSystem<D> {
        DrawSystem {
            render_target_view: rtv,
            bundle: None,
        }
    }

    pub fn create<F>(&mut self, factory: &mut F, terrain: &Terrain) -> Drawable
        where F: gfx::Factory<D::Resources>
    {
        let vertices: Vec<Vertex> = terrain
            .heightmap
            .iter()
            .enumerate()
            .map(|(i, h)| Vertex::new(i as f32, *h as f32))
            .collect();

        let rast = gfx::state::Rasterizer::new_fill();
        let bundle = create_pso_bundle(factory,
                                       self.render_target_view.clone(),
                                       rast,
                                       &vertices[..]);
        self.bundle = Some(bundle);

        let width = vertices.len() as f32;
        let height = terrain.max_height as f32;

        let mat = Matrix4::from_translation(Vector3::new(-1.0, -1.0, 0.0)) *
                  Matrix4::from_nonuniform_scale(2.0 / width, 2.0 / height, 1.0);

        Drawable {
            bounds: Bounds { transform: mat.into() },
            base: Base { base_y: -1.0 },
        }
    }

    pub fn draw(&self,
                drawable: &Drawable,
                encoder: &mut gfx::Encoder<D::Resources, D::CommandBuffer>) {
        if let Some(ref bundle) = self.bundle {
            encoder.update_constant_buffer(&bundle.data.bounds, &drawable.bounds);
            encoder.update_constant_buffer(&bundle.data.base, &drawable.base);
            bundle.encode(encoder);
        }
    }
}
