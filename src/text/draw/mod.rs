use cgmath::{Matrix4, Point2};
use cgmath::prelude::*;
use draw::ColorFormat;
use gfx;
use specs;
use text::Text;

mod font;

#[derive(Debug)]
pub struct Drawable {
    locals: Locals,
    indices: Vec<u16>,
}

impl Drawable {
    pub fn new(color: [f32; 3]) -> Drawable {
        Drawable {
            locals: Locals {
                transform: [[0f32; 4]; 4],
                color: color,
            },
            indices: Vec::new(),
        }
    }

    fn update(&mut self, text: &str, screen_position: &Point2<f32>, height: f32) {}
}

impl specs::Component for Drawable {
    type Storage = specs::VecStorage<Drawable>;
}

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "position",
        tex_coord: [f32; 2] = "texcoord",
    }

    constant Locals {
        transform: [[f32; 4]; 4] = "transform",
        color: [f32; 3] = "color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        font: gfx::TextureSampler<f32> = "font",
        out: gfx::RenderTarget<ColorFormat> = "out_color",
    }
}

const SHADER_VERT: &[u8] = include_bytes!("text.v.glsl");
const SHADER_FRAG: &[u8] = include_bytes!("text.f.glsl");
const MAX_VERTICES: usize = 64;

pub struct DrawSystem<R: gfx::Resources> {
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
                                          gfx::Primitive::TriangleList,
                                          gfx::state::Rasterizer::new_fill(),
                                          pipe::new())
            .unwrap();
        let vbuf = factory
            .create_buffer(MAX_VERTICES,
                           gfx::buffer::Role::Vertex,
                           gfx::memory::Usage::Upload,
                           gfx::Bind::empty())
            .unwrap();
        let (char_info, font_texture) = font::generate_texture(factory);
        let data = pipe::Data {
            vbuf: vbuf,
            locals: factory.create_constant_buffer(1),
            font: font_texture,
            out: rtv,
        };
        DrawSystem {
            pso: pso,
            data: data,
        }
    }

    pub fn draw<C: gfx::CommandBuffer<R>>(&self,
                                          drawable: &Drawable,
                                          encoder: &mut gfx::Encoder<R, C>) {
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
        let (mut drawables, texts) = arg.fetch(|w| (w.write::<Drawable>(), w.read::<Text>()));

        for (d, t) in (&mut drawables, &texts).join() {
            d.update(&t.text, &t.screen_position, t.height);
        }
    }
}
