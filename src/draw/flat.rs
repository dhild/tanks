use super::ColorFormat;
use cgmath::Matrix4;
use gfx;

#[derive(Debug,Clone)]
pub struct Drawable {
    id: usize,
    transform: Transform,
}

impl Drawable {
    pub fn update(&mut self, mat: Matrix4<f32>) {
        self.transform = mat.into()
    }
}

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [gfx::format::U8Norm; 4] = "a_Color",
    }

    constant Transform {
        transform: [[f32; 4]; 4] = "u_Transform",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

impl From<Matrix4<f32>> for Transform {
    fn from(mat: Matrix4<f32>) -> Transform {
        Transform {
            transform: [[mat.x.x, mat.y.x, mat.z.x, mat.w.x],
                        [mat.x.y, mat.y.y, mat.z.y, mat.w.y],
                        [mat.x.z, mat.y.z, mat.z.z, mat.w.z],
                        [mat.x.w, mat.y.w, mat.z.w, mat.w.w]],
        }
    }
}

impl Vertex {
    pub fn new(x: f32, y: f32, col: u32) -> Vertex {
        let col4 = [(col >> 24) as u8,
                    (col >> 16) as u8,
                    (col >> 8) as u8,
                    col as u8];
        Vertex {
            pos: [x, y],
            color: gfx::format::U8Norm::cast4(col4),
        }
    }
}

const SHADER_VERT: &'static [u8] = include_bytes!("shaders/flat.v.glsl");
const SHADER_FRAG: &'static [u8] = include_bytes!("shaders/flat.f.glsl");

fn create_pso_bundle<F, R>(factory: &mut F,
                           out: gfx::handle::RenderTargetView<R, ColorFormat>,
                           rast: gfx::state::Rasterizer,
                           primitive: gfx::Primitive,
                           vertices: &[Vertex])
                           -> gfx::pso::bundle::Bundle<R, pipe::Data<R>>
    where R: gfx::Resources,
          F: gfx::Factory<R>
{
    use gfx::traits::FactoryExt;
    let program = factory.link_program(SHADER_VERT, SHADER_FRAG).unwrap();
    let pso = factory
        .create_pipeline_from_program(&program, primitive, rast, pipe::new())
        .unwrap();
    let (vbuf, slice) = factory.create_vertex_buffer_with_slice(vertices, ());
    let data = pipe::Data {
        vbuf: vbuf,
        transform: factory.create_constant_buffer(1),
        out: out,
    };
    gfx::Bundle::new(slice, pso, data)
}

pub struct DrawSystem<D: gfx::Device> {
    render_target_view: gfx::handle::RenderTargetView<D::Resources, ColorFormat>,
    bundles: Vec<gfx::pso::bundle::Bundle<D::Resources, pipe::Data<D::Resources>>>,
}

impl<D: gfx::Device> DrawSystem<D> {
    pub fn new(rtv: gfx::handle::RenderTargetView<D::Resources, ColorFormat>) -> DrawSystem<D> {
        DrawSystem {
            render_target_view: rtv,
            bundles: Vec::new(),
        }
    }

    pub fn add_drawable<F>(&mut self,
                           factory: &mut F,
                           rast: gfx::state::Rasterizer,
                           primitive: gfx::Primitive,
                           vertices: &[Vertex])
                           -> Drawable
        where F: gfx::Factory<D::Resources>
    {
        let bundle = create_pso_bundle(factory,
                                       self.render_target_view.clone(),
                                       rast,
                                       primitive,
                                       vertices);
        let id = self.bundles.len();
        self.bundles.push(bundle);
        Drawable {
            id: id,
            transform: Transform { transform: [[0.0; 4]; 4] },
        }
    }

    pub fn draw(&mut self,
            drawable: &Drawable,
            encoder: &mut gfx::Encoder<D::Resources, D::CommandBuffer>) {
        let bundle = &self.bundles[drawable.id];
        encoder.update_constant_buffer(&bundle.data.transform, &drawable.transform);
        bundle.encode(encoder);
    }
}
