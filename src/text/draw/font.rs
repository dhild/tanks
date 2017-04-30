use super::Vertex;
use gfx;
use gfx::texture::{AaMode, Kind, FilterMethod, SamplerInfo, WrapMode};
use rusttype::{Font, FontCollection, GlyphId, Point as FontPoint, Rect as FontRect, Scale};
use std::collections::HashMap;
use std::iter;

const FONT_DATA: &[u8] = include_bytes!("../../../assets/Inconsolata-Regular.ttf");

pub fn generate_texture<R: gfx::Resources, F: gfx::Factory<R>>
    (factory: &mut F)
     -> (gfx::handle::ShaderResourceView<R, f32>, gfx::handle::Sampler<R>) {
    FONT_INFO.generate_texture(factory)
}

pub fn generate_vertices(text: &str) -> Vec<Vertex> {
    FONT_INFO.generate_vertices(text)
}

type FontTextureFormat = (gfx::format::R32, gfx::format::Float);

lazy_static! {
    static ref FONT_INFO: FontInfo<'static> = {
        FontInfo::new()
    };
}

struct FontInfo<'a> {
    font: Font<'a>,
    scale: Scale,
    bitmap: Vec<u32>,
    char_info: HashMap<GlyphId, FontRect<f32>>,
    total_width: u16,
    total_height: u16,
}

impl<'a> FontInfo<'a> {
    fn new() -> FontInfo<'a> {
        let font = FontCollection::from_bytes(FONT_DATA)
            .into_font()
            .unwrap();
        let scale = Scale::uniform(32.0);
        let bounds = get_bounds(&font, scale);
        let glyph_width = (bounds.max.x - bounds.min.x) as u32;
        let glyph_height = (bounds.max.y - bounds.min.y) as u32;
        let glyph_cols = (font.glyph_count() as f32).sqrt() as u32;
        let glyph_rows = 1 + (font.glyph_count() as u32 / glyph_cols);
        let total_width = glyph_width * glyph_cols;
        let total_height = glyph_height * glyph_rows;
        debug!("Glyph bounds: {} x {} -> {} x {}",
               glyph_width,
               glyph_height,
               glyph_cols,
               glyph_rows);
        debug!("Size of font bitmap: {} x {} => {}",
               total_width,
               total_height,
               total_width * total_height);
        let mut bitmap = iter::repeat(0)
            .take((total_width * total_height) as usize)
            .collect::<Vec<u32>>();
        let mut char_info = HashMap::new();
        let mut row = 0;
        let mut col = 0;
        for gid in 0..font.glyph_count() {
            if let Some(g) = font.glyph(GlyphId(gid as u32)) {
                let g = g.scaled(scale)
                    .positioned(FontPoint {
                                    x: -bounds.min.x as f32,
                                    y: -bounds.min.y as f32,
                                });
                if let Some(bb) = g.pixel_bounding_box() {
                    let origin_x = col * glyph_width;
                    let origin_y = row * glyph_height;
                    trace!("bb: {:?}", bb);
                    trace!("\torigin: {} => {}, {}", gid, origin_x, origin_y);
                    g.draw(|x, y, v| {
                        trace!("\t\t{:?}: {}, {} = {}", g.id(), x, y, v);
                        let x_abs = origin_x + x;
                        let y_abs = origin_y + y;
                        let index = (y_abs * total_width) + x_abs;
                        trace!("\t\t\t{} = {}", index, v);
                        bitmap[index as usize] = unsafe { ::std::mem::transmute::<f32, u32>(v) };
                    });

                    let left = origin_x;
                    let right = origin_x + (bb.width() as u32);
                    let top = origin_y;
                    let bottom = origin_y + (bb.height() as u32);
                    trace!("top: {}\tbottom: {}\tleft: {}\tright:{}",
                           top,
                           bottom,
                           left,
                           right);
                    let rect = FontRect {
                        min: FontPoint {
                            x: (left as f32) / (total_width as f32),
                            y: (bottom as f32) / (total_height as f32),
                        },
                        max: FontPoint {
                            x: (right as f32) / (total_width as f32),
                            y: (top as f32) / (total_height as f32),
                        },
                    };
                    char_info.insert(g.id(), rect);
                    col += 1;
                    if col == glyph_cols {
                        col = 0;
                        row += 1;
                    }
                }
            }
        }

        FontInfo {
            font: font,
            scale: scale,
            bitmap: bitmap,
            char_info: char_info,
            total_width: total_width as u16,
            total_height: total_height as u16,
        }
    }

    fn generate_texture<R: gfx::Resources, F: gfx::Factory<R>>
        (&self,
         factory: &mut F)
         -> (gfx::handle::ShaderResourceView<R, f32>, gfx::handle::Sampler<R>) {
        let kind = Kind::D2(self.total_width, self.total_height, AaMode::Single);
        let (_, texture_view) = factory
            .create_texture_immutable::<FontTextureFormat>(kind, &[&self.bitmap[..]])
            .unwrap();
        let sampler = factory.create_sampler(SamplerInfo::new(FilterMethod::Trilinear,
                                                              WrapMode::Clamp));
        (texture_view, sampler)
    }

    fn generate_vertices(&self, text: &str) -> Vec<Vertex> {
        self.font
            .layout(text, self.scale, FontPoint { x: 0.0, y: 0.0 })
            .flat_map(|g| {
                if let Some(tc) = self.char_info.get(&g.id()) {
                    if let Some(bb) = g.pixel_bounding_box() {
                        return vec![Vertex {
                                        pos: [bb.min.x as f32, -bb.max.y as f32],
                                        tex_coord: [tc.min.x, tc.min.y],
                                    },
                                    Vertex {
                                        pos: [bb.min.x as f32, -bb.min.y as f32],
                                        tex_coord: [tc.min.x, tc.max.y],
                                    },
                                    Vertex {
                                        pos: [bb.max.x as f32, -bb.min.y as f32],
                                        tex_coord: [tc.max.x, tc.max.y],
                                    },
                                    Vertex {
                                        pos: [bb.min.x as f32, -bb.max.y as f32],
                                        tex_coord: [tc.min.x, tc.min.y],
                                    },
                                    Vertex {
                                        pos: [bb.max.x as f32, -bb.min.y as f32],
                                        tex_coord: [tc.max.x, tc.max.y],
                                    },
                                    Vertex {
                                        pos: [bb.max.x as f32, -bb.max.y as f32],
                                        tex_coord: [tc.max.x, tc.min.y],
                                    }]
                                       .into_iter();
                    }
                }
                Vec::new().into_iter()
            })
            .collect()
    }
}

fn get_bounds(font: &Font, scale: Scale) -> FontRect<i32> {
    let mut bounds = FontRect {
        min: FontPoint {
            x: i32::max_value(),
            y: i32::max_value(),
        },
        max: FontPoint {
            x: i32::min_value(),
            y: i32::min_value(),
        },
    };
    for gid in 0..font.glyph_count() {
        if let Some(g) = font.glyph(GlyphId(gid as u32)) {
            let g = g.scaled(scale).positioned(FontPoint { x: 0.0, y: 0.0 });
            if let Some(bb) = g.pixel_bounding_box() {
                if bounds.min.x > bb.min.x {
                    bounds.min.x = bb.min.x;
                }
                if bounds.max.x < bb.max.x {
                    bounds.max.x = bb.max.x;
                }
                if bounds.min.y > bb.min.y {
                    bounds.min.y = bb.min.y;
                }
                if bounds.max.y < bb.max.y {
                    bounds.max.y = bb.max.y;
                }
            }
        }
    }
    bounds
}
