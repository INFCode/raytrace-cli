use super::hittable::{HitRecord, Hittable};
use crate::{ray::Ray, utils::Interval};
use nalgebra::Point3;

pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, avaliable_range: &Interval) -> Option<HitRecord> {
        // Solve the quadratic equation based on vector math.
        // Find the nearer intersection
        let oc = ray.origin - self.center;

        let longest_reach = self.radius + avaliable_range.upper * ray.direction.magnitude();

        if oc.magnitude_squared() > longest_reach * longest_reach {
            // The sphere is farther than the farthest point the ray may ever reach
            return None;
        }

        let a = ray.direction.magnitude_squared();
        let b_half = oc.dot(&ray.direction);
        let c = oc.magnitude_squared() - self.radius * self.radius;

        let delta = b_half * b_half - a * c;

        if delta < 0f64 {
            return None;
        }

        let mut root = (-b_half - delta.sqrt()) / a;
        if !avaliable_range.contains(root) {
            // Try the larger root if the smaller on is out of range
            root = (-b_half + delta.sqrt()) / a;
        }
        if !avaliable_range.contains(root) {
            // Still not in the range
            return None;
        }

        let normal_vec = (ray.at(root) - self.center) / self.radius;

        Some(HitRecord::new(ray, &normal_vec, root))
    }
}
