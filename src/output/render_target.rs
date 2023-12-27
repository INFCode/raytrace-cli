use glam::DVec2;
use std::{
    fmt::{Display, Formatter, Result},
    usize,
};

use crate::color::LinearRgbColor;

pub trait RenderTarget: Display {
    // Get the width of the image.
    fn width(&self) -> usize;

    // Get the height of the image.
    fn height(&self) -> usize;

    // Get the theoretical aspect ratio. This returns a tuple of the form (numerator, denominator).
    fn theoretical_aspect_ratio(&self) -> f64;

    // Get the actual aspect ratio as a floating-point number.
    fn actual_aspect_ratio(&self) -> f64 {
        let width = self.width() as f64;
        let height = self.height() as f64;
        width / height
    }

    // Set a specific pixel.
    fn set_pixel(&mut self, x: usize, y: usize, color: &LinearRgbColor);

    // Calculate the normalized or proportional position of a pixel.
    fn normalized_pixel_position(&self, col: f64, row: f64) -> DVec2 {
        DVec2::new(
            (col + 0.5f64) / self.width() as f64,
            (row + 0.5f64) / self.height() as f64,
        )
    }
}

pub struct ImageTarget {
    width: usize,
    height: usize,
    aspect_ratio: f64,
    buffer: Vec<LinearRgbColor>,
}

impl ImageTarget {
    pub fn new(width: usize, aspect_ratio: f64) -> ImageTarget {
        let mut height = (width as f64 / aspect_ratio).round() as usize;
        if height < 1 {
            height = 1;
        }
        let buffer = vec![LinearRgbColor::default(); width * height];
        ImageTarget {
            width,
            height,
            aspect_ratio,
            buffer,
        }
    }
}

impl Display for ImageTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "P3\n{} {}\n255\n", self.width(), self.height())?;
        for c in &self.buffer {
            writeln!(f, "{}", c)?;
        }
        Ok(())
    }
}

impl RenderTarget for ImageTarget {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn theoretical_aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: &LinearRgbColor) {
        // assume it's given in order
        self.buffer[x * self.width + y] = *color;
    }
}

pub struct TerminalTarget {
    width: usize,
    height: usize,
    aspect_ratio: f64,
    character_width: usize,
    character_height: usize,
    buffer: Vec<LinearRgbColor>,
}

impl TerminalTarget {
    pub fn new(
        width: usize,
        aspect_ratio: f64,
        character_width: usize,
        character_height: usize,
    ) -> TerminalTarget {
        let mut height = (width as f64 / aspect_ratio).round() as usize;
        if height < 1 {
            height = 1;
        }
        let buffer = vec![LinearRgbColor::default(); width * height];
        TerminalTarget {
            width,
            height,
            aspect_ratio,
            character_width,
            character_height,
            buffer,
        }
    }
}

impl Display for TerminalTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for c in &self.buffer {
            writeln!(f, "{}", c)?;
        }
        Ok(())
    }
}

impl RenderTarget for TerminalTarget {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn theoretical_aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: &LinearRgbColor) {
        self.buffer[x * self.width + y] = *color;
    }
}
