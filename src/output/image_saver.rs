use std::{fs::File, io::Write, ops::Deref, path::Path};

use image::{imageops, EncodableLayout, GenericImageView, ImageBuffer, Pixel, PixelWithColorType};

use super::character::{GlyphPalette, GlyphPixel, BLURRED_SHAPE, BLURRED_SHAPE_SIZE};

pub trait ImageSaver {
    fn save_to<P, C>(&self, buff: ImageBuffer<P, C>, path: impl AsRef<Path>)
    where
        [P::Subpixel]: EncodableLayout,
        C: Deref<Target = [P::Subpixel]>,
        P: PixelWithColorType + 'static;
}

pub struct ImageFormatsSaver {}

impl ImageFormatsSaver {
    pub fn new() -> Self {
        Self {}
    }
}

impl ImageSaver for ImageFormatsSaver {
    fn save_to<P, C>(&self, buff: ImageBuffer<P, C>, path: impl AsRef<Path>)
    where
        [P::Subpixel]: EncodableLayout,
        C: Deref<Target = [P::Subpixel]>,
        P: PixelWithColorType,
    {
        buff.save(path).unwrap()
    }
}

pub struct AsciiArtSaver {
    palette: GlyphPalette,
}

impl AsciiArtSaver {
    pub fn new<P: AsRef<Path>>(font_path: P) -> Self {
        Self {
            palette: GlyphPalette::from_font(font_path),
        }
    }
}

fn pad_image<P, C>(
    img: &ImageBuffer<P, C>,
    width_align: u32,
    height_align: u32,
) -> ImageBuffer<P, Vec<P::Subpixel>>
where
    C: Deref<Target = [P::Subpixel]>,
    P: Pixel + 'static,
{
    let width = img.width();
    let height = img.height();

    // Calculate the new width and height that are multiples of character size
    let new_width = (width + width_align - 1) / width_align * width_align;
    let new_height = (height + height_align - 1) / height_align * height_align;

    // Pad the image to the new dimensions
    imageops::resize(img, new_width, new_height, imageops::Nearest)
}

fn subpixel_to_u8<T: image::Primitive>(subpixel: T) -> u8 {
    // Normalize the subpixel value to a floating point 0-1 range and scale to 0-255
    let normalized = subpixel.to_f32().unwrap() / T::max_value().to_f32().unwrap();
    (normalized * 255.0).round() as u8
}

impl ImageSaver for AsciiArtSaver {
    fn save_to<P, C>(&self, buff: ImageBuffer<P, C>, path: impl AsRef<Path>)
    where
        [P::Subpixel]: EncodableLayout,
        C: Deref<Target = [P::Subpixel]>,
        P: PixelWithColorType + 'static,
    {
        // Pad the image so its dimensions are multiples of character size
        let padded_img = pad_image(&buff, BLURRED_SHAPE.0, BLURRED_SHAPE.1);

        // Calculate the number of blocks that can fit horizontally and vertically
        let blocks_horizontal = padded_img.width() / BLURRED_SHAPE.0;
        let blocks_vertical = padded_img.height() / BLURRED_SHAPE.1;

        // Initialize a vector to hold the results
        let mut results: Vec<Vec<char>> =
            vec![vec![' '; blocks_horizontal as usize]; blocks_vertical as usize];

        // Process each block in parallel
        (0..blocks_vertical).into_iter().for_each(|i| {
            for j in 0..blocks_horizontal {
                // Define the top-left corner of the current block
                let x = j * BLURRED_SHAPE.0;
                let y = i * BLURRED_SHAPE.1;

                // Create a view in the image
                let sub_image =
                    imageops::crop_imm(&padded_img, x, y, BLURRED_SHAPE.0, BLURRED_SHAPE.1);

                // Initialize an array to hold the grayscale values
                let mut grayscale_array: GlyphPixel = [0; BLURRED_SHAPE_SIZE];

                // Convert each pixel to grayscale and insert it into the array
                for (k, (_, _, pixel)) in (*sub_image).pixels().enumerate() {
                    let grayscale = pixel.to_luma();
                    grayscale_array[k] = subpixel_to_u8(grayscale[0]);
                }

                // Process the grayscale array and store the result
                results[i as usize][j as usize] = self.palette.match_char(&grayscale_array);
                //if j == 50 {
                //    println!("map ");
                //    GlyphPalette::print_glyph_pixel(&grayscale_array);
                //    let mi = *grayscale_array.iter().min().unwrap();
                //    let ma = *grayscale_array.iter().max().unwrap();
                //    println!("range [{}, {}]", mi, ma);
                //    println!("into {}", results[i as usize][j as usize]);
                //}
            }
        });

        // Open a file in write mode
        let mut file = File::create(path).unwrap();

        // Iterate over the matrix and write each row as a line in the file
        for row in results {
            // Convert the character vector to a string
            let line: String = row.iter().collect();

            // Write the line to the file, add a newline character to separate rows
            writeln!(file, "{}", line).unwrap();
        }
    }
}
