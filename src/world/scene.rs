use crate::{color::LinearRgbColor, ray::Ray};

use super::{IntersectContainer, Intersectable};

pub trait Scene: Intersectable {
    fn miss(&self, ray: &Ray) -> LinearRgbColor;
}

pub struct LerpScene<C: IntersectContainer> {
    container: C,
    start_color: LinearRgbColor,
    end_color: LinearRgbColor,
}

impl<C: IntersectContainer> LerpScene<C> {
    pub fn new(container: C, start_color: LinearRgbColor, end_color: LinearRgbColor) -> Self {
        Self {
            container,
            start_color,
            end_color,
        }
    }
}

impl<C: IntersectContainer> Intersectable for LerpScene<C> {
    fn hit(
        &self,
        ray: &Ray,
        avaliable_range: &crate::utils::Interval,
    ) -> Option<super::IntersectRecord> {
        self.container.hit(ray, avaliable_range)
    }
}

impl<C: IntersectContainer> Scene for LerpScene<C> {
    fn miss(&self, ray: &Ray) -> LinearRgbColor {
        let dir = ray.direction.normalize();
        let t = 0.5 * (dir.y + 1f64);
        LinearRgbColor::lerp(
            //&,
            //&,
            &self.start_color,
            &self.end_color,
            t,
        )
    }
}
