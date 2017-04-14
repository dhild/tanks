use super::ColorFormat;
use components::{Delta, Matrix4, Point, Position};
use gfx;
use specs;

#[derive(Debug,Clone)]
pub struct Drawable {
    id: usize,
    transform: Transform,
}

impl specs::Component for Drawable {
    type Storage = specs::VecStorage<Drawable>;
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

fn create_pso_data<F, R>(factory: &mut F,
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
    encoder_queue: super::EncoderQueue<D>,
}

impl<D: gfx::Device> DrawSystem<D> {
    pub fn new(rtv: gfx::handle::RenderTargetView<D::Resources, ColorFormat>,
               queue: super::EncoderQueue<D>)
               -> DrawSystem<D> {
        DrawSystem {
            render_target_view: rtv,
            bundles: Vec::new(),
            encoder_queue: queue,
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
        let bundle = create_pso_data(factory,
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

    fn draw<'a, I>(&mut self, iter: I, encoder: &mut gfx::Encoder<D::Resources, D::CommandBuffer>)
        where I: Iterator<Item = &'a Drawable>
    {
        encoder.clear(&self.render_target_view, [0.0, 0.0, 0.0, 1.0]);
        for d in iter {
            let bundle = &self.bundles[d.id];
            encoder.update_constant_buffer(&bundle.data.transform, &d.transform);
            bundle.encode(encoder);
        }
    }
}

impl<D> specs::System<Delta> for DrawSystem<D>
    where D: gfx::Device,
          D::CommandBuffer: Send
{
    fn run(&mut self, arg: specs::RunArg, _: Delta) {
        use specs::Join;
        let mut encoder = self.encoder_queue.receiver.recv().unwrap();
        let drawables = arg.fetch(|w| w.read::<Drawable>());
        self.draw((&drawables).join(), &mut encoder);
        if let Err(e) = self.encoder_queue.sender.send(encoder) {
            warn!("Disconnected, cannot return encoder to mpsc: {}", e);
        };
    }
}

pub struct PreDrawSystem {
    scale: Matrix4,
}

impl PreDrawSystem {
    pub fn new<P: Into<Point>>(extents: P) -> PreDrawSystem {
        let extents = extents.into();
        let scale = 1.0 / extents;
        let scale = Matrix4::from_nonuniform_scale(scale.x, scale.y, 1.0);
        PreDrawSystem { scale: scale }
    }
}

impl specs::System<Delta> for PreDrawSystem {
    fn run(&mut self, arg: specs::RunArg, _: Delta) {
        use specs::Join;
        let (mut drawables, positions) =
            arg.fetch(|w| (w.write::<Drawable>(), w.read::<Position>()));
        for (d, p) in (&mut drawables, &positions).join() {
            d.transform = (self.scale * p.to_translation()).into();
        }
    }
}

impl From<Matrix4> for Transform {
    fn from(mat: Matrix4) -> Transform {
        Transform {
            transform: [[mat.x.x, mat.y.x, mat.z.x, mat.w.x],
                        [mat.x.y, mat.y.y, mat.z.y, mat.w.y],
                        [mat.x.z, mat.y.z, mat.z.z, mat.w.z],
                        [mat.x.w, mat.y.w, mat.z.w, mat.w.w]],
        }
    }
}
