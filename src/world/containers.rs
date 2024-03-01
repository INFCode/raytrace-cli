use crate::{ray::Ray, utils::Interval};

use super::{IntersectRecord, Intersectable};

pub trait IntersectContainer: Intersectable {
    fn add<I: Intersectable + 'static>(&mut self, i: I);
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Box<dyn Intersectable>>;
}

pub type VecContainer = Vec<Box<dyn Intersectable>>;

impl Intersectable for VecContainer {
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

impl IntersectContainer for VecContainer {
    fn add<I: Intersectable + 'static>(&mut self, i: I) {
        self.push(Box::new(i) as Box<dyn Intersectable>);
    }

    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Box<dyn Intersectable>>,
    {
        iter.into_iter().collect()
    }
}
