use cgmath::Matrix4;
use cgmath::prelude::*;
use draw::ColorFormat;
use gfx;
use physics::Position;
use specs;
use tank::Tank;

#[derive(Debug,Clone)]
pub struct Drawable {
    body: Locals,
    barrel: Locals,
}

impl Drawable {
    pub fn new(color: [f32; 3]) -> Drawable {
        Drawable {
            body: Locals {
                transform: Matrix4::identity().into(),
                color: color,
            },
            barrel: Locals {
                transform: Matrix4::identity().into(),
                color: color,
            },
        }
    }

    pub fn update(&mut self, world_to_clip: &Matrix4<f32>, pos: &Position, tank: &Tank) {
        self.body.transform = (world_to_clip * tank.body_to_world(pos)).into();
        self.barrel.transform = (world_to_clip * tank.barrel_to_world(pos)).into();
    }
}

impl specs::Component for Drawable {
    type Storage = specs::VecStorage<Drawable>;
}

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "position",
    }

    constant Locals {
        transform: [[f32; 4]; 4] = "transform",
        color: [f32; 3] = "color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        out: gfx::RenderTarget<ColorFormat> = "out_color",
    }
}

static VERTICES_BODY: [Vertex; 4] = [Vertex { pos: [-0.8, 0.5] },
                                     Vertex { pos: [0.8, 0.5] },
                                     Vertex { pos: [-1.0, 0.0] },
                                     Vertex { pos: [1.0, 0.0] }];
static VERTICES_BARREL: [Vertex; 4] = [Vertex { pos: [-0.1, 1.2] },
                                       Vertex { pos: [0.1, 1.2] },
                                       Vertex { pos: [-0.1, 0.25] },
                                       Vertex { pos: [0.1, 0.25] }];
const SHADER_VERT: &'static [u8] = include_bytes!("tank.v.glsl");
const SHADER_FRAG: &'static [u8] = include_bytes!("tank.f.glsl");

pub struct DrawSystem<R: gfx::Resources> {
    slice_body: gfx::Slice<R>,
    slice_barrel: gfx::Slice<R>,
    pso: gfx::pso::PipelineState<R, pipe::Meta>,
    data: pipe::Data<R>,
}

impl<R: gfx::Resources> DrawSystem<R> {
    pub fn new<F>(factory: &mut F,
                  rtv: gfx::handle::RenderTargetView<R, ColorFormat>)
                  -> DrawSystem<R>
        where F: gfx::Factory<R>
    {
        use gfx::traits::FactoryExt;
        let program = factory.link_program(SHADER_VERT, SHADER_FRAG).unwrap();
        let pso = factory
            .create_pipeline_from_program(&program,
                                          gfx::Primitive::TriangleStrip,
                                          gfx::state::Rasterizer::new_fill(),
                                          pipe::new())
            .unwrap();
        let vertices = {
            let mut v = Vec::new();
            v.extend_from_slice(&VERTICES_BODY);
            v.extend_from_slice(&VERTICES_BARREL);
            v
        };
        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertices[..], ());
        let (slice_body, slice_barrel) = slice.split_at(4);
        let data = pipe::Data {
            vbuf: vbuf,
            locals: factory.create_constant_buffer(1),
            out: rtv,
        };
        DrawSystem {
            slice_body: slice_body,
            slice_barrel: slice_barrel,
            pso: pso,
            data: data,
        }
    }

    pub fn draw<C: gfx::CommandBuffer<R>>(&self,
                                          drawable: &Drawable,
                                          encoder: &mut gfx::Encoder<R, C>) {
        encoder.update_constant_buffer(&self.data.locals, &drawable.body);
        encoder.draw(&self.slice_body, &self.pso, &self.data);
        encoder.update_constant_buffer(&self.data.locals, &drawable.barrel);
        encoder.draw(&self.slice_barrel, &self.pso, &self.data);
    }
}
