use crate::materials::SharedMaterial;
use crate::ray::Ray;
use crate::utils::Interval;
use glam::DVec3;
use std::boxed::Box;

pub trait Intersectable: Sync {
    fn hit(&self, ray: &Ray, avaliable_range: &Interval) -> Option<IntersectRecord>;
}

pub struct IntersectRecord {
    pub point: DVec3,
    // Note that normal should unit vector.
    // TODO: Apply the Unit<> wrapper
    pub normal: DVec3,
    pub t: f64,
    pub is_front: bool,
    pub mat: SharedMaterial,
}

impl IntersectRecord {
    pub fn new(ray: &Ray, outward_normal: DVec3, t: f64, mat: SharedMaterial) -> Self {
        let is_front = ray.direction.dot(outward_normal) < 0f64;
        let normal = if is_front {
            // dereference here
            outward_normal
        } else {
            -outward_normal
        };
        let point = ray.at(t);
        Self {
            point,
            normal,
            t,
            is_front,
            mat,
        }
    }
}

pub type World = Vec<Box<dyn Intersectable>>;

impl Intersectable for World {
    fn hit(&self, ray: &Ray, avaliable_range: &Interval) -> Option<IntersectRecord> {
        let mut nearest_record = None;
        let mut current_range = avaliable_range.clone();
        for h in self {
            if let Some(rec) = h.hit(ray, &current_range) {
                // Decrease the upperbound of the range to intersction test
                current_range.upper = rec.t;
                nearest_record = Some(rec);
            }
        }
        nearest_record
    }
}
