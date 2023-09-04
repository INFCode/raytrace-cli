use super::hittable::{HitRecord, Hittable};
use crate::{materials::MaterialRef, ray::Ray, utils::Interval};
use nalgebra::Point3;

pub struct Sphere<'a> {
    center: Point3<f64>,
    radius: f64,
    material: MaterialRef<'a>,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Point3<f64>, radius: f64, material: MaterialRef<'a>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<'a> Hittable for Sphere<'a> {
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

        Some(HitRecord::new(ray, &normal_vec, root, self.material))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::materials::SimpleDiffuseMaterial;
    use crate::{materials::Material, ray::Ray, utils::Interval};
    use nalgebra::Vector3;

    #[test]
    fn test_hit_from_outside_sphere() {
        let sphere_center = Point3::new(0.0, 0.0, 0.0);
        let sphere_radius = 1.0;
        let material = Box::new(SimpleDiffuseMaterial::new()) as Box<dyn Material>;
        let sphere = Sphere::new(sphere_center, sphere_radius, &material);

        let ray_origin = Point3::new(2.0, 0.0, 0.0);
        let ray_direction = Vector3::new(-1.0, 0.0, 0.0);
        let ray = Ray::new(ray_origin, ray_direction);

        let available_range = Interval::new(0.0, 5.0);

        assert!(sphere.hit(&ray, &available_range).is_some());
    }

    #[test]
    fn test_miss_from_outside_sphere() {
        let sphere_center = Point3::new(0.0, 0.0, 0.0);
        let sphere_radius = 1.0;
        let material = Box::new(SimpleDiffuseMaterial::new()) as Box<dyn Material>;
        let sphere = Sphere::new(sphere_center, sphere_radius, &material);

        let ray_origin = Point3::new(2.0, 2.0, 0.0);
        let ray_direction = Vector3::new(-1.0, 0.0, 0.0);
        let ray = Ray::new(ray_origin, ray_direction);

        let available_range = Interval::new(0.0, 5.0);

        assert!(sphere.hit(&ray, &available_range).is_none());
    }

    #[test]
    fn test_hit_from_inside_sphere() {
        let sphere_center = Point3::new(0.0, 0.0, 0.0);
        let sphere_radius = 2.0;
        let material = Box::new(SimpleDiffuseMaterial::new()) as Box<dyn Material>;
        let sphere = Sphere::new(sphere_center, sphere_radius, &material);

        let ray_origin = Point3::new(0.0, 0.0, 0.0);
        let ray_direction = Vector3::new(1.0, 0.0, 0.0);
        let ray = Ray::new(ray_origin, ray_direction);

        let available_range = Interval::new(0.0, 5.0);

        assert!(sphere.hit(&ray, &available_range).is_some());
    }

    #[test]
    fn test_hit_with_limited_range() {
        let sphere_center = Point3::new(0.0, 0.0, 0.0);
        let sphere_radius = 1.0;
        let material = Box::new(SimpleDiffuseMaterial::new()) as Box<dyn Material>;
        let sphere = Sphere::new(sphere_center, sphere_radius, &material);

        let ray_origin = Point3::new(2.0, 0.0, 0.0);
        let ray_direction = Vector3::new(-1.0, 0.0, 0.0);
        let ray = Ray::new(ray_origin, ray_direction);

        let available_range = Interval::new(0.0, 0.5);

        assert!(sphere.hit(&ray, &available_range).is_none());
    }
}
