use std::iter::repeat_with;

use crate::ray::Ray;
use glam::DVec3;
use rand::random;

#[derive(Copy, Clone)]
pub struct ImageSize {
    pub width: u32,
    pub height: u32,
}

// Trait to define the rendering specifications
pub trait RenderSpec: Sync {
    // Function to get the image size
    fn image_size(&self) -> ImageSize;

    // Function to generate a ray for a given pixel position
    // Returns an iterator over Rays
    fn ray_for_pixel(&self, x: u32, y: u32) -> Box<dyn Iterator<Item = Ray> + '_>;
}

pub struct PinHoleSpec {
    sample_per_pixel: usize,
    pixel_tangent: f64,
    resolution: ImageSize,
}

impl PinHoleSpec {
    pub fn new(sample_per_pixel: usize, fov_in_degree: f64, resolution: ImageSize) -> Self {
        let diagnal_len_in_pix = ((resolution.width * resolution.width
            + resolution.height * resolution.height) as f64)
            .sqrt();
        let pixel_tangent = (fov_in_degree / 2f64).to_radians().tan() / (diagnal_len_in_pix / 2f64);
        Self {
            sample_per_pixel,
            pixel_tangent,
            resolution,
        }
    }
}

impl RenderSpec for PinHoleSpec {
    fn image_size(&self) -> ImageSize {
        self.resolution
    }

    fn ray_for_pixel(&self, x: u32, y: u32) -> Box<dyn Iterator<Item = Ray> + '_> {
        // Generate and return an iterator over Rays for the given pixel
        // This is where you implement your ray generation logic
        let base_vector = DVec3::new(
            x as f64 - self.resolution.width as f64 / 2f64 + 0.5f64,
            -(y as f64 - self.resolution.height as f64 / 2f64 + 0.5f64),
            0f64,
        ) * self.pixel_tangent
            + DVec3::new(0f64, 0f64, -1f64);
        let origin = DVec3::ZERO;

        Box::new(
            repeat_with(move || Ray {
                origin,
                direction: base_vector
                    + self.pixel_tangent
                        * DVec3::new(random::<f64>() - 0.5, random::<f64>() - 0.5, 0f64),
            })
            .take(self.sample_per_pixel),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pinhole() {
        let spec = PinHoleSpec::new(
            10,
            90f64,
            ImageSize {
                width: 2,
                height: 2,
            },
        );
        let half_sqrt_2 = 2f64.sqrt() / 2f64;
        assert!((spec.pixel_tangent - half_sqrt_2).abs() <= 1e-6);

        let rays = spec.ray_for_pixel(1, 1);
        let mut num_rays = 0;
        for ray in rays {
            num_rays += 1;
            assert_eq!(ray.origin, DVec3::ZERO);
            let dir = ray.direction;
            dbg!(dir);
            assert!(0f64 <= dir[0] && dir[0] <= half_sqrt_2);
            assert!(-half_sqrt_2 <= dir[1] && dir[1] <= 0f64);
            assert_eq!(ray.direction[2], -1f64);
        }
        assert_eq!(num_rays, 10);
    }
}
