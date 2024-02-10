use crate::materials::{Material, ScatterRecord};
use crate::utils::random_unit_vector;
use crate::utils::random_unit_vector_on_hemisphere;
use crate::{ray::Ray, world::intersectable::IntersectRecord};
use glam::DVec3;

#[derive(Debug)]
pub struct SimpleDiffuseMaterial {}

impl SimpleDiffuseMaterial {
    pub fn new() -> Self {
        Self {}
    }
}

impl Material for SimpleDiffuseMaterial {
    fn scatter(&self, _ray: &Ray, hit: &IntersectRecord) -> Option<ScatterRecord> {
        let attenuation_factor = DVec3::splat(0.5);
        let scattered = Ray {
            origin: hit.point,
            direction: random_unit_vector_on_hemisphere(hit.normal),
        };
        Some(ScatterRecord {
            attenuation_factor,
            scattered,
        })
    }
}

#[derive(Debug)]
pub struct LambertianMaterial {
    albedo: DVec3,
}

impl LambertianMaterial {
    pub fn new(albedo: DVec3) -> Self {
        Self { albedo }
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, _ray: &Ray, hit: &IntersectRecord) -> Option<ScatterRecord> {
        let attenuation_factor = self.albedo;
        let mut random_dir = hit.normal + random_unit_vector();
        let epsilon = 1e-7;
        if random_dir.length_squared() < epsilon {
            random_dir = hit.normal;
        }
        let scattered = Ray {
            origin: hit.point,
            direction: random_dir,
        };
        Some(ScatterRecord {
            attenuation_factor,
            scattered,
        })
    }
}
