use crate::materials::Material;
use crate::ray::Ray;
use crate::world::HitRecord;
use nalgebra::vector;
use rand::random;

use super::ScatterRecord;

pub struct DielectricMaterial {
    ir: f64, // Index of Refraction
}

impl DielectricMaterial {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }

    fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for DielectricMaterial {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = vector![1.0, 1.0, 1.0];
        let refraction_ratio = if rec.is_front { 1.0 / self.ir } else { self.ir };

        let unit_direction = ray.direction.normalize();
        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || self.reflectance(cos_theta, refraction_ratio) > random() {
                unit_direction - 2f64 * unit_direction.dot(&rec.normal) * rec.normal
            } else {
                let cos_theta = f64::min(-unit_direction.dot(&rec.normal), 1.0);
                let r_out_perp = refraction_ratio * (unit_direction + cos_theta * rec.normal);
                let r_out_parallel = -((1.0 - r_out_perp.norm_squared()).abs().sqrt()) * rec.normal;
                r_out_perp + r_out_parallel
            };

        Some(ScatterRecord {
            scattered: Ray::new(rec.point, direction),
            attenuation_factor: attenuation,
        })
    }
}
