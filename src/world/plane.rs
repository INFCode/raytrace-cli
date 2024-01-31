use super::hittable::{HitRecord, Hittable};
use crate::{materials::MaterialRef, ray::Ray, utils::Interval};
use glam::DVec3;

pub struct InfinitePlane<'a> {
    position: DVec3,
    normal: DVec3,
    material: MaterialRef<'a>,
}

impl<'a> InfinitePlane<'a> {
    pub fn new(position: DVec3, normal: DVec3, material: MaterialRef<'a>) -> Self {
        Self {
            position,
            normal,
            material,
        }
    }
}

impl<'a> Hittable for InfinitePlane<'a> {
    fn hit(&self, ray: &Ray, avaliable_range: &Interval) -> Option<HitRecord> {
        let denom = -self.normal.dot(ray.direction);
        if denom > 1e-6 {
            let p0l0 = self.position - ray.origin;
            let t = -p0l0.dot(self.normal) / denom;
            if avaliable_range.contains(t) {
                Some(HitRecord::new(ray, self.normal, t, self.material))
            } else {
                None
            }
        } else {
            // parallel, no intersection
            None
        }
    }
}
