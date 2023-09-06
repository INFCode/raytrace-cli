use nalgebra::Vector3;

use crate::{
    ray::Ray,
    utils::{random_unit_vector, Interval},
    world::HitRecord,
};

use super::{Material, ScatterRecord};

pub struct MetalMaterial {
    albedo: Vector3<f64>,
    fuzz: f64,
}

impl MetalMaterial {
    pub fn new(albedo: Vector3<f64>, fuzz: f64) -> Self {
        let fuzz = Interval::new(0f64, 1f64).clamp(fuzz.abs());
        Self { albedo, fuzz }
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let unit_dir = ray.direction.normalize();
        let reflect = unit_dir - 2f64 * unit_dir.dot(&hit.normal) * hit.normal;
        let fuzz_reflect = reflect + self.fuzz * random_unit_vector();
        Some(ScatterRecord {
            attenuation_factor: self.albedo,
            scattered: Ray::new(hit.point, fuzz_reflect),
        })
    }
}
