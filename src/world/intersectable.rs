use crate::materials::SharedMaterial;
use crate::ray::Ray;
use crate::utils::Interval;
use glam::DVec3;

pub trait Intersectable: Sync {
    fn hit(&self, ray: &Ray, avaliable_range: &Interval) -> Option<IntersectRecord>;
    fn into_box(self) -> Box<dyn Intersectable + 'static>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
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
