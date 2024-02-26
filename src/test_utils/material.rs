use crate::{
    materials::{Material, ScatterRecord, SharedMaterial},
    ray::Ray,
    world::IntersectRecord,
};

pub struct DummyMaterial {}

impl DummyMaterial {
    pub fn new_shared() -> SharedMaterial {
        Self::make_shared(DummyMaterial {})
    }
}

impl Material for DummyMaterial {
    fn scatter(&self, _ray: &Ray, _hit: &IntersectRecord) -> Option<ScatterRecord> {
        unimplemented!("DummyMaterial for test use and scatter should never be called.");
    }
}
