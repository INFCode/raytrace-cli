use nalgebra::Vector3;
use std::boxed::Box;
use std::fmt::Debug;

use crate::ray::Ray;
use crate::world::hittable::HitRecord;

pub struct ScatterRecord {
    pub attenuation_factor: Vector3<f64>,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord>;
}

pub type MaterialRef<'a> = &'a Box<dyn Material>;

// Makes it simpler to use MaterialRef
impl<'a> Material for MaterialRef<'a> {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        (**self).scatter(ray, hit)
    }
}
