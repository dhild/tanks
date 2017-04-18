use cgmath::{Matrix4, Vector3};
use cgmath::prelude::*;
use draw::{ColorFormat, Position};
use gfx;
use tank::Tank;

#[derive(Debug,Clone)]
pub struct Drawable {
    body: Locals,
    barrel: Locals,
}

impl Drawable {
    pub fn update(&mut self, world_to_clip: &Matrix4<f32>, pos: &Position, tank: &Tank) {
        let translate =
            Matrix4::from_translation(Vector3::new(pos.position.x, pos.position.y, 0.0));
        let scale = Matrix4::from_nonuniform_scale(20.0, 20.0, 1.0);
        let body_orient = Matrix4::from_angle_z(tank.tank_orient);
        let barrel_orient = Matrix4::from_angle_z(tank.barrel_orient);

        self.body.transform = (world_to_clip * translate * scale * body_orient).into();
        self.barrel.transform = (world_to_clip * translate * scale * barrel_orient).into();
    }
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

impl Vertex {
    pub fn new(x: f32, y: f32) -> Vertex {
        Vertex { pos: [x, y] }
    }
}

const SHADER_VERT: &'static [u8] = include_bytes!("tank.v.glsl");
const SHADER_FRAG: &'static [u8] = include_bytes!("tank.f.glsl");

struct TankBundle<R: gfx::Resources, Data: gfx::pso::PipelineData<R>> {
    slice_body: gfx::Slice<R>,
    slice_barrel: gfx::Slice<R>,
    pso: gfx::pso::PipelineState<R, Data::Meta>,
    data: Data,
}

impl<R: gfx::Resources> TankBundle<R, pipe::Data<R>> {
    pub fn new<F>(factory: &mut F,
                  rtv: gfx::handle::RenderTargetView<R, ColorFormat>)
                  -> TankBundle<R, pipe::Data<R>>
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
        let vertices = vec![// Main body:
                            Vertex::new(-0.8, 0.5),
                            Vertex::new(0.8, 0.5),
                            Vertex::new(-1.0, 0.0),
                            Vertex::new(1.0, 0.0),
                            // Barrel:
                            Vertex::new(-0.1, 1.2),
                            Vertex::new(0.1, 1.2),
                            Vertex::new(-0.1, 0.25),
                            Vertex::new(0.1, 0.25)];
        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertices[..], ());
        let (slice_body, slice_barrel) = slice.split_at(4);
        let data = pipe::Data {
            vbuf: vbuf,
            locals: factory.create_constant_buffer(1),
            out: rtv,
        };
        TankBundle {
            slice_body: slice_body,
            slice_barrel: slice_barrel,
            pso: pso,
            data: data,
        }
    }

    pub fn encode<C>(&self, encoder: &mut gfx::Encoder<R, C>, drawable: &Drawable)
        where C: gfx::CommandBuffer<R>
    {
        encoder.update_constant_buffer(&self.data.locals, &drawable.body);
        encoder.draw(&self.slice_body, &self.pso, &self.data);
        encoder.update_constant_buffer(&self.data.locals, &drawable.barrel);
        encoder.draw(&self.slice_barrel, &self.pso, &self.data);
    }
}

pub struct DrawSystem<R: gfx::Resources> {
    render_target_view: gfx::handle::RenderTargetView<R, ColorFormat>,
    tank_bundle: Option<TankBundle<R, pipe::Data<R>>>,
}

impl<R: gfx::Resources> DrawSystem<R> {
    pub fn new(rtv: gfx::handle::RenderTargetView<R, ColorFormat>) -> DrawSystem<R> {
        DrawSystem {
            render_target_view: rtv,
            tank_bundle: None,
        }
    }

    pub fn create_tank<F>(&mut self, factory: &mut F, color: [f32; 3]) -> Drawable
        where F: gfx::Factory<R>
    {
        if self.tank_bundle.is_none() {
            self.tank_bundle = Some(TankBundle::new(factory, self.render_target_view.clone()));
        }
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

    pub fn draw<C: gfx::CommandBuffer<R>>(&self,
                                          drawable: &Drawable,
                                          encoder: &mut gfx::Encoder<R, C>) {
        if let Some(ref bundle) = self.tank_bundle {
            bundle.encode(encoder, drawable);
        }
    }
}
