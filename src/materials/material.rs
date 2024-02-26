use glam::DVec3;
use std::sync::Arc;

use crate::ray::Ray;
use crate::world::intersectable::IntersectRecord;

pub struct ScatterRecord {
    pub attenuation_factor: DVec3,
    pub scattered: Ray,
}

pub trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &IntersectRecord) -> Option<ScatterRecord>;

    fn make_shared<Mat: Material + 'static>(material: Mat) -> SharedMaterial
    where
        Self: Sized,
    {
        Arc::new(material)
    }
}

pub type SharedMaterial = Arc<dyn Material>;
