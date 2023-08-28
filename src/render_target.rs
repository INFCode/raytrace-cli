use nalgebra::Point2;

pub struct RenderTarget {
    width: isize,
    height: isize,
    aspect_ratio: f64,
}

impl RenderTarget {
    pub fn new(width: isize, aspect_ratio: f64) -> RenderTarget {
        let mut height = (width as f64 / aspect_ratio).round() as isize;
        if height < 1 {
            height = 1;
        }
        RenderTarget {
            width,
            height,
            aspect_ratio,
        }
    }

    pub fn real_ratio(&self) -> f64 {
        return self.width as f64 / self.height as f64;
    }

    pub fn relative_position_of_pixel(&self, row: isize, col: isize) -> Point2<f64> {
        Point2::new(
            row as f64 / self.width as f64 + 0.5f64,
            col as f64 / self.height as f64 + 0.5f64,
        )
    }

    pub fn width(&self) -> isize {
        self.width
    }

    pub fn height(&self) -> isize {
        self.height
    }
}
