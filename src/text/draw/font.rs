use gfx;
use rusttype::{Font, FontCollection, Point as FontPoint, Rect as FontRect, Scale};
use std::collections::HashMap;
use std::iter;

const FONT_DATA: &[u8] = include_bytes!("../../../assets/Inconsolata-Regular.ttf");

static SUPPORTED_CHARS: &[char] =
    &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
      's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
      'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1',
      '2', '3', '4', '5', '6', '7', '8', '9'];

type FontTextureFormat = (gfx::format::R8, gfx::format::Unorm);

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
    for g in font.glyphs_for(SUPPORTED_CHARS.iter().cloned()) {
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
    bounds
}

pub fn generate_texture<R: gfx::Resources, F: gfx::Factory<R>>
    (factory: &mut F)
     -> (HashMap<char, FontRect<f32>>,
(gfx::handle::ShaderResourceView<R, f32>, gfx::handle::Sampler<R>)){
    let font = FontCollection::from_bytes(FONT_DATA)
        .into_font()
        .unwrap();
    let scale = Scale::uniform(32.0);
    let bounds = get_bounds(&font, scale);
    let glyph_width = (bounds.max.x - bounds.min.x) as u32;
    let glyph_height = (bounds.max.y - bounds.min.y) as u32;
    let glyph_cols = (SUPPORTED_CHARS.len() as f32).sqrt() as u32;
    let glyph_rows = 1 + (SUPPORTED_CHARS.len() as u32 / glyph_cols);
    let total_width = glyph_width * glyph_cols;
    let total_height = glyph_height * glyph_rows;
    debug!("Glyph bounds: {} x {} -> {} x {}",
           glyph_width,
           glyph_height,
           glyph_cols,
           glyph_rows);
    debug!("Size of font bitmap: {}", total_width * total_height);
    let mut bitmap = iter::repeat(0)
        .take((glyph_width * glyph_height * glyph_cols * glyph_rows) as usize)
        .collect::<Vec<u8>>();
    let mut char_info = HashMap::new();
    let mut row = 0;
    let mut col = 0;
    for (c, g) in SUPPORTED_CHARS
            .iter()
            .zip(font.glyphs_for(SUPPORTED_CHARS.iter().cloned())) {
        let g = g.scaled(scale).positioned(FontPoint { x: 0.0, y: 0.0 });
        if let Some(bb) = g.pixel_bounding_box() {
            let origin_x = (bb.min.x - bounds.min.x) as u32 + (row * glyph_height);
            let origin_y = (bb.min.y - bounds.min.y) as u32 + (col * glyph_width);
            debug!("\torigin: {} => {}, {}", c, origin_x, origin_y);
            g.draw(|x, y, v| {
                       trace!("\t\t{:?}: {}, {} = {}", g.id(), x, y, v);
                       let x_abs = origin_x + x;
                       let y_abs = origin_y + y;
                       let index = (y_abs * total_width) + x_abs;
                       trace!("\t\t\t{} = {}", index, v);
                       bitmap[index as usize] = (v * 255.0) as u8;
                   });

            let left = origin_x;
            let right = origin_x + ((bb.max.x - bb.min.x) as u32);
            let top = total_height - origin_y;
            let bottom = total_height - origin_y + ((bb.max.y - bb.min.y) as u32);
            char_info.insert(*c,
                             FontRect {
                                 min: FontPoint {
                                     x: (left as f32) / (total_width as f32),
                                     y: (bottom as f32) / (total_height as f32),
                                 },
                                 max: FontPoint {
                                     x: (right as f32) / (total_width as f32),
                                     y: (top as f32) / (total_height as f32),
                                 },
                             });

            col += 1;
            if col == glyph_cols {
                col = 0;
                row += 1;
            }
        }
    }

    let (_, texture_view) = factory
   .create_texture_immutable::<FontTextureFormat>(gfx::texture::Kind::D2(total_width as u16,
                                                          total_height as u16,
                                                          gfx::texture::AaMode::Single),
                                   &[&bitmap[..]])
   .unwrap();
    let sampler =
        factory
            .create_sampler(gfx::texture::SamplerInfo::new(gfx::texture::FilterMethod::Trilinear,
                                                           gfx::texture::WrapMode::Clamp));

    (char_info, (texture_view, sampler))
}
