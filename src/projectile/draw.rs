use cgmath::Matrix4;
use cgmath::prelude::*;
use draw::ColorFormat;
use gfx;
use physics::{Dimensions, Position};
use specs;

#[derive(Debug,Clone)]
pub struct Drawable {
    locals: Locals,
}

impl Drawable {
    pub fn new() -> Drawable {
        Drawable {
            locals: Locals {
                transform: Matrix4::identity().into(),
                color: [1.0, 1.0, 1.0],
            },
        }
    }

    pub fn update(&mut self, world_to_clip: &Matrix4<f32>, pos: &Position) {
        self.locals.transform = (world_to_clip * pos.model_to_world()).into();
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

static VERTICES: [Vertex; 3] = [Vertex { pos: [-0.5, -0.5] },
                                Vertex { pos: [0.0, 0.5] },
                                Vertex { pos: [0.5, -0.5] }];
const SHADER_VERT: &'static [u8] = include_bytes!("projectile.v.glsl");
const SHADER_FRAG: &'static [u8] = include_bytes!("projectile.f.glsl");

pub struct DrawSystem<R: gfx::Resources> {
    bundle: gfx::pso::bundle::Bundle<R, pipe::Data<R>>,
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
            v.extend_from_slice(&VERTICES);
            v
        };
        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertices[..], ());
        let data = pipe::Data {
            vbuf: vbuf,
            locals: factory.create_constant_buffer(1),
            out: rtv,
        };
        DrawSystem { bundle: gfx::pso::bundle::Bundle::new(slice, pso, data) }
    }

    pub fn draw<C: gfx::CommandBuffer<R>>(&self,
                                          drawable: &Drawable,
                                          encoder: &mut gfx::Encoder<R, C>) {
        encoder.update_constant_buffer(&self.bundle.data.locals, &drawable.locals);
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
        let (positions, dim, mut projectiles) =
            arg.fetch(|w| {
                          (w.read::<Position>(),
                           w.read_resource::<Dimensions>(),
                           w.write::<Drawable>())
                      });

        let world_to_clip = dim.world_to_clip();
        for (p, d) in (&positions, &mut projectiles).join() {
            d.update(&world_to_clip, p);
        }
    }
}
