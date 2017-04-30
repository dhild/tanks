use cgmath::{Matrix4, Point2, Vector3};
use draw::ColorFormat;
use gfx;
use physics::Dimensions;
use specs;
use text::Text;

mod font;

#[derive(Debug)]
pub struct Drawable {
    locals: Locals,
    vertices: Vec<Vertex>,
}

impl Drawable {
    pub fn new(color: [f32; 3]) -> Drawable {
        Drawable {
            locals: Locals {
                transform: [[0f32; 4]; 4],
                color: color,
            },
            vertices: Vec::new(),
        }
    }

    fn update(&mut self, dim: &Dimensions, text: &str, screen_position: &Point2<f32>, scale: f32) {
        self.vertices = font::generate_vertices(text);
        let width = dim.game_width() as f32;
        let height = dim.game_height() as f32;
        let mat = Matrix4::from_nonuniform_scale(2.0 / width, 2.0 / height, 1.0) *
                  Matrix4::from_translation(Vector3::new(screen_position.x - width / 2.0,
                                                         screen_position.y - height / 2.0,
                                                         0.0)) *
                  Matrix4::from_nonuniform_scale(scale, scale, 1.0);
        self.locals.transform = mat.into();
    }
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
const MAX_VERTICES: usize = 64 * 6;

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
                           gfx::memory::Usage::Dynamic,
                           gfx::Bind::empty())
            .unwrap();
        let font_texture = font::generate_texture(factory);
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
        encoder.update_constant_buffer(&self.data.locals, &drawable.locals);
        encoder
            .update_buffer(&self.data.vbuf, &drawable.vertices, 0)
            .unwrap();
        let slice = gfx::Slice::new_match_vertex_buffer(&self.data.vbuf);
        encoder.draw(&slice, &self.pso, &self.data);
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
        let (mut drawables, texts, dim) =
            arg.fetch(|w| {
                          (w.write::<Drawable>(), w.read::<Text>(), w.read_resource::<Dimensions>())
                      });

        for (d, t) in (&mut drawables, &texts).join() {
            d.update(&dim, &t.text, &t.screen_position, t.scale);
        }
    }
}
