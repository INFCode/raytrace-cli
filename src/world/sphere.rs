use super::intersectable::{IntersectRecord, Intersectable};
use crate::{materials::MaterialRef, ray::Ray, utils::Interval};
use glam::DVec3;

pub struct Sphere<'a> {
    center: DVec3,
    radius: f64,
    material: MaterialRef<'a>,
}

impl<'a> Sphere<'a> {
    pub fn new(center: DVec3, radius: f64, material: MaterialRef<'a>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<'a> Intersectable for Sphere<'a> {
    fn hit(&self, ray: &Ray, avaliable_range: &Interval) -> Option<IntersectRecord> {
        // Solve the quadratic equation based on vector math.
        // Find the nearer intersection
        let oc = ray.origin - self.center;

        let longest_reach = self.radius + avaliable_range.upper * ray.direction.length();

        if oc.length_squared() > longest_reach * longest_reach {
            // The sphere is farther than the farthest point the ray may ever reach
            return None;
        }

        let a = ray.direction.length_squared();
        let b_half = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

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

        Some(IntersectRecord::new(ray, normal_vec, root, self.material))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::material::DummyMaterial;
    use crate::{ray::Ray, utils::Interval};

    #[test]
    fn test_hit_from_outside_sphere() {
        let sphere_center = DVec3::ZERO;
        let sphere_radius = 1.0;
        let material = DummyMaterial::new_boxed();
        let sphere = Sphere::new(sphere_center, sphere_radius, &material);

        let ray_origin = DVec3::new(2.0, 0.0, 0.0);
        let ray_direction = DVec3::NEG_X;
        let ray = Ray::new(ray_origin, ray_direction);

        let available_range = Interval::new(0.0, 5.0);

        assert!(sphere.hit(&ray, &available_range).is_some());
    }

    #[test]
    fn test_miss_from_outside_sphere() {
        let sphere_center = DVec3::ZERO;
        let sphere_radius = 1.0;
        let material = DummyMaterial::new_boxed();
        let sphere = Sphere::new(sphere_center, sphere_radius, &material);

        let ray_origin = DVec3::new(2.0, 2.0, 0.0);
        let ray_direction = DVec3::NEG_X;
        let ray = Ray::new(ray_origin, ray_direction);

        let available_range = Interval::new(0.0, 5.0);

        assert!(sphere.hit(&ray, &available_range).is_none());
    }

    #[test]
    fn test_hit_from_inside_sphere() {
        let sphere_center = DVec3::ZERO;
        let sphere_radius = 2.0;
        let material = DummyMaterial::new_boxed();
        let sphere = Sphere::new(sphere_center, sphere_radius, &material);

        let ray_origin = DVec3::ZERO;
        let ray_direction = DVec3::X;
        let ray = Ray::new(ray_origin, ray_direction);

        let available_range = Interval::new(0.0, 5.0);

        assert!(sphere.hit(&ray, &available_range).is_some());
    }

    #[test]
    fn test_hit_with_limited_range() {
        let sphere_center = DVec3::ZERO;
        let sphere_radius = 1.0;
        let material = DummyMaterial::new_boxed();
        let sphere = Sphere::new(sphere_center, sphere_radius, &material);

        let ray_origin = DVec3::new(2.0, 0.0, 0.0);
        let ray_direction = DVec3::NEG_X;
        let ray = Ray::new(ray_origin, ray_direction);

        let available_range = Interval::new(0.0, 0.5);

        assert!(sphere.hit(&ray, &available_range).is_none());
    }
}
