use fontdue::{Font, FontSettings};
use image::imageops;
use std::fs::File;
use std::io::Read;
use std::path::Path;

const SIZE: f32 = 20.0;
pub const BLURRED_SHAPE: (u32, u32) = (4, 8);
pub const BLURRED_SHAPE_SIZE: usize = (BLURRED_SHAPE.0 * BLURRED_SHAPE.1) as usize;

//type GlyphPixel = SVector<u8, BLURRED_SHAPE_SIZE>;
pub type GlyphPixel = [u8; BLURRED_SHAPE_SIZE];

pub struct GlyphPalette {
    glyph_to_char: Vec<(GlyphPixel, char)>,
}

impl GlyphPalette {
    pub fn from_font<P: AsRef<Path>>(filepath: P) -> GlyphPalette {
        let font = Self::load_font(filepath).unwrap();
        GlyphPalette {
            glyph_to_char: Self::font_to_glyph_pixels(&font),
        }
    }

    fn load_font<P: AsRef<Path>>(filename: P) -> Result<Font, String> {
        let mut file = File::open(filename).map_err(|e| e.to_string())?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
        let settings = FontSettings {
            scale: SIZE,
            ..FontSettings::default()
        };
        Font::from_bytes(buffer, settings).map_err(|e| e.to_string())
    }

    fn font_to_glyph_pixels(font: &Font) -> Vec<(GlyphPixel, char)> {
        let mut result = vec![];
        // Currently only use Ascii codes
        for i in 0..127 {
            if let Some(c) = std::char::from_u32(i) {
                if c == ' ' || c.is_ascii_graphic() {
                    let v = Self::get_glyph_pixel(font, c);
                    result.push((v, c));
                }
            }
        }
        //for (pix, c) in result.iter() {
        //    println!("{c} => ");
        //    Self::print_glyph_pixel(pix);
        //}
        result
    }

    fn get_glyph_pixel(font: &Font, character: char) -> GlyphPixel {
        let (glyph_metrics, glyph_bitmap) = font.rasterize(character, SIZE);

        //println!("Normal:");
        //Self::print_normal(glyph_metrics.height, glyph_metrics.width, &glyph_bitmap);

        let glyph_image = image::GrayImage::from_raw(
            glyph_metrics.width as u32,
            glyph_metrics.height as u32,
            glyph_bitmap,
        )
        .unwrap();

        // Down-scaling
        let small_blurred = imageops::resize(
            &glyph_image,
            BLURRED_SHAPE.0,
            BLURRED_SHAPE.1,
            image::imageops::FilterType::Gaussian,
        );
        let vec_blurred = small_blurred.into_vec();
        vec_blurred.try_into().unwrap()
    }

    pub fn print_glyph_pixel(glyph_pixel: &GlyphPixel) {
        for y in 0..BLURRED_SHAPE.1 {
            for x in 0..BLURRED_SHAPE.0 {
                let char_s = glyph_pixel[(x + y * BLURRED_SHAPE.0) as usize];
                print!("\x1B[48;2;{};{};{}m   ", char_s, char_s, char_s);
            }
            println!("\x1B[0m");
        }
    }

    pub fn match_char(&self, pattern: &GlyphPixel) -> char {
        let mut best_dist = f64::INFINITY;
        let mut best_char = self.glyph_to_char[0].1;
        for (v, c) in self.glyph_to_char.iter() {
            let sq_dist = Self::pixel_similarity(pattern, v);
            if sq_dist < best_dist {
                best_dist = sq_dist;
                best_char = *c;
            }
        }
        best_char
    }

    fn pixel_similarity(p1: &GlyphPixel, p2: &GlyphPixel) -> f64 {
        p1.iter()
            .zip(p2.iter()) // Combine the two arrays into an iterator of tuples
            .map(|(&x, &y)| {
                let diff = x as i32 - y as i32;
                diff.pow(2) as u32
            })
            .sum::<u32>() as f64
    }
}
