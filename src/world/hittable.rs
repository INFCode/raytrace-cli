use crate::ray::Ray;
use crate::utils::Interval;
use nalgebra::{Point3, Vector3};

pub trait Hittable {
    fn hit(&self, ray: &Ray, avaliable_range: &Interval) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub point: Point3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub is_front: bool,
}

impl HitRecord {
    pub fn new(ray: &Ray, outward_normal: &Vector3<f64>, t: f64) -> Self {
        let is_front = ray.direction.dot(outward_normal) < 0f64;
        let normal = if is_front {
            // dereference here
            *outward_normal
        } else {
            -outward_normal
        };
        let point = ray.at(t);
        Self {
            point,
            normal,
            t,
            is_front,
        }
    }
}

pub type World = Vec<Box<dyn Hittable>>;

impl Hittable for World {
    fn hit(&self, ray: &Ray, avaliable_range: &Interval) -> Option<HitRecord> {
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