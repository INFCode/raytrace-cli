use fontdue::{Font, FontSettings};
use image::imageops;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

const SIZE: f32 = 20.0;

pub fn font_from_file<P: AsRef<Path>>(filename: P) -> Result<Font, String> {
    let mut file = File::open(filename).map_err(|e| e.to_string())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
    let settings = FontSettings {
        scale: SIZE,
        ..FontSettings::default()
    };
    Font::from_bytes(buffer, settings).map_err(|e| e.to_string())
}

pub fn font_to_ascii_vector(font: &Font) -> HashMap<char, Vec<u8>> {
    let mut map = HashMap::new();
    for i in 0..127 {
        if let Some(c) = std::char::from_u32(i) {
            if c == ' ' || c.is_ascii_graphic() {
                let v = characteristic_vector(font, c).unwrap();
                map.insert(c, v);
            }
        }
    }
    map
}

pub fn characteristic_vector(font: &Font, character: char) -> Result<Vec<u8>, String> {
    let (glyph_metrics, glyph_bitmap) = font.rasterize(character, SIZE);

    println!("Normal:");
    print_normal(glyph_metrics.height, glyph_metrics.width, &glyph_bitmap);

    let glyph_image = image::GrayImage::from_raw(
        glyph_metrics.width as u32,
        glyph_metrics.height as u32,
        glyph_bitmap,
    )
    .unwrap();

    // Scaling down to 4x4
    let small_blurred = imageops::resize(&glyph_image, 4, 4, image::imageops::FilterType::Gaussian);
    let vec_blurred = small_blurred.into_vec();

    println!("Blurred:");
    print_normal(4, 4, &vec_blurred);

    Ok(vec_blurred)
}

pub fn print_normal(height: usize, width: usize, bitmap: &Vec<u8>) {
    for y in 0..height {
        for x in 0..width {
            let char_s = bitmap[x + y * width];
            print!("\x1B[48;2;{};{};{}m   ", char_s, char_s, char_s);
        }
        println!("\x1B[0m");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_all() {
        let f = font_from_file("/usr/share/fonts/consolas-with-yahei/consnerd.ttf").unwrap();
        let m = font_to_ascii_vector(&f);
        println!(
            "{:?}",
            m.keys()
                .map(|x| char::escape_debug(x.clone()).to_string())
                .collect::<Vec<String>>()
        );
    }
}
