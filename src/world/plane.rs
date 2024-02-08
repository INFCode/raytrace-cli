use super::hittable::{HitRecord, Hittable};
use crate::{materials::MaterialRef, ray::Ray, utils::Interval};
use glam::{DQuat, DVec3};

fn plenary_hit() {}

pub struct Rectangle<'a> {
    position: DVec3,
    normal: DVec3,
    right: DVec3,
    down: DVec3,
    material: MaterialRef<'a>,
}

impl<'a> Rectangle<'a> {
    pub fn new(
        position: DVec3,
        rotation: DQuat,
        width: f64,
        height: f64,
        material: MaterialRef<'a>,
    ) -> Self {
        let normal = rotation * DVec3::Y;
        let right = rotation * DVec3::X * width;
        let down = rotation * DVec3::Z * height;
        println!(
            "finite plane with normal {}, right {}, down{}",
            normal, right, down
        );
        Self {
            position,
            normal,
            right,
            down,
            material,
        }
    }
}

impl<'a> Hittable for Rectangle<'a> {
    fn hit(&self, ray: &Ray, avaliable_range: &Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(ray.direction);
        if denom > -1e-6 {
            // parallel or from the back side, no intersection
            return None;
        }
        let p0l0 = self.position - ray.origin;
        let t = p0l0.dot(self.normal) / denom;
        if !avaliable_range.contains(t) {
            return None;
        }
        // Calculate the intersection point
        let intersection = ray.at(t);

        // Check if the intersection point is within the rectangle
        // Project the intersection point onto the rectangle plane
        let projected_point = intersection - self.position;

        // Calculate the projection factors along the rectangle's edges
        let factor1 = projected_point.dot(self.right) / self.right.dot(self.right);
        let factor2 = projected_point.dot(self.down) / self.down.dot(self.down);
        //dbg!("f1 {}, f2 {}", factor1, factor2);

        // Check if the factors are within the range [0, 1] for both edges
        if factor1 >= -0.5 && factor1 <= 0.5 && factor2 >= -0.5 && factor2 <= 0.5 {
            Some(HitRecord::new(ray, self.normal, t, self.material))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod finite_plane_tests {
    use super::*;
    use crate::materials::SimpleDiffuseMaterial;
    use crate::{materials::Material, ray::Ray, utils::Interval};
    use rand::{
        distributions::{Distribution, Uniform},
        rngs::StdRng,
        SeedableRng,
    };

    #[test]
    fn test_parallel() {
        let material = Box::new(SimpleDiffuseMaterial::new()) as Box<dyn Material>;
        let plane = Rectangle::new(DVec3::ZERO, DQuat::IDENTITY, 10f64, 10f64, &material);

        let ray = Ray::new(DVec3::Y, DVec3::X);

        assert!(plane.hit(&ray, &Interval::greater_than(0f64)).is_none());
    }

    #[test]
    fn test_from_back_side() {
        let material = Box::new(SimpleDiffuseMaterial::new()) as Box<dyn Material>;
        let plane = Rectangle::new(DVec3::ZERO, DQuat::IDENTITY, 10f64, 10f64, &material);

        // Upward vector
        let ray = Ray::new(DVec3::NEG_Y, DVec3::Y);

        assert!(plane.hit(&ray, &Interval::greater_than(0f64)).is_none());
    }

    #[test]
    fn test_outside_region() {
        let material = Box::new(SimpleDiffuseMaterial::new()) as Box<dyn Material>;
        let plane = Rectangle::new(DVec3::ZERO, DQuat::IDENTITY, 2f64, 2f64, &material);

        // almost inside the region
        let ray = Ray::new(DVec3::Y, DVec3::new(1.01, -1f64, 0f64));
        assert!(plane.hit(&ray, &Interval::greater_than(0f64)).is_none());

        // far from the region
        let ray = Ray::new(DVec3::Y, DVec3::new(10f64, -1f64, 0f64));
        assert!(plane.hit(&ray, &Interval::greater_than(0f64)).is_none());
    }

    #[test]
    fn test_inside_region() {
        let material = Box::new(SimpleDiffuseMaterial::new()) as Box<dyn Material>;
        let plane = Rectangle::new(DVec3::ZERO, DQuat::IDENTITY, 2f64, 2f64, &material);

        // almost outside the region
        let ray = Ray::new(DVec3::Y, DVec3::new(0.95, -1f64, 0f64));
        assert!(plane.hit(&ray, &Interval::greater_than(0f64)).is_some());

        // certainly inside the region
        let ray = Ray::new(DVec3::Y, DVec3::NEG_Y);
        assert!(plane.hit(&ray, &Interval::greater_than(0f64)).is_some());

        // random vectors in the region
        let mut rng = StdRng::from_seed([42; 32]);
        let uniform = Uniform::new_inclusive(-0.99, 0.99);
        for _ in 0..1000 {
            let offset = uniform.sample(&mut rng);
            let ray = Ray::new(DVec3::Y, DVec3::new(offset, -1f64, 0f64));
            assert!(
                plane.hit(&ray, &Interval::greater_than(0f64)).is_some(),
                "offset {}",
                offset
            );
        }
    }
}

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

#[cfg(test)]
mod infinite_plane_tests {
    use super::*;
    use crate::test_utils::material::DummyMaterial;

    #[test]
    fn ray_from_below_misses() {
        let material = DummyMaterial::new_box();
        let plane = InfinitePlane::new(DVec3::ZERO, DVec3::Y, &material);
        let ray = Ray {
            origin: DVec3::NEG_Y,
            direction: DVec3::Y,
        };
        let available_range = Interval::new(0f64, 2f64);
        assert!(
            plane.hit(&ray, &available_range).is_none(),
            "Ray from below should not hit the plane"
        );
    }

    #[test]
    fn parallel_ray_misses() {
        let material = DummyMaterial::new_box();
        let plane = InfinitePlane::new(DVec3::ZERO, DVec3::Y, &material);
        let ray_parallel = Ray {
            origin: DVec3::NEG_Y,
            direction: DVec3::X,
        };
        let available_range = Interval::new(0f64, 2f64);
        assert!(
            plane.hit(&ray_parallel, &available_range).is_none(),
            "Parallel ray should not hit the plane"
        );
    }

    #[test]
    fn ray_pointing_away_misses() {
        let material = DummyMaterial::new_box();
        let plane = InfinitePlane::new(DVec3::ZERO, DVec3::Y, &material);
        let ray_away = Ray {
            origin: DVec3::Y,
            direction: DVec3::Y,
        };
        let available_range = Interval::new(0f64, 2f64);
        assert!(
            plane.hit(&ray_away, &available_range).is_none(),
            "Ray pointing away should not hit the plane"
        );
    }

    #[test]
    fn plane_out_of_range_misses() {
        let material = DummyMaterial::new_box();
        let plane = InfinitePlane::new(DVec3::ZERO, DVec3::Y, &material);
        let ray_on_plane = Ray {
            origin: DVec3::new(0.0, 10.0, 0.0),
            direction: DVec3::NEG_Y,
        };
        let available_range = Interval::new(0f64, 2f64);
        assert!(
            plane.hit(&ray_on_plane, &available_range).is_none(),
            "Plane out of range should not be hit"
        );
    }

    #[test]
    fn hit_ray_towards_plane() {
        let material = DummyMaterial::new_box();
        let plane = InfinitePlane::new(DVec3::ZERO, DVec3::Y, &material);
        let ray_on_plane = Ray {
            origin: DVec3::Y,
            direction: DVec3::NEG_Y,
        };
        let available_range = Interval::new(0f64, 2f64);
        assert!(
            plane.hit(&ray_on_plane, &available_range).is_some(),
            "Ray should hit the plane when facing towards it"
        );
    }
}
