use nalgebra::Rotation3;
use nalgebra::Vector3;
use rand::{thread_rng, Rng};
use std::f64::consts::PI;

#[derive(Clone, Copy)]
pub struct Interval {
    pub lower: f64,
    pub upper: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self {
            lower: min,
            upper: max,
        }
    }

    pub fn empty() -> Self {
        Self {
            lower: f64::INFINITY,
            upper: f64::NEG_INFINITY,
        }
    }

    pub fn universe() -> Self {
        Self {
            lower: f64::NEG_INFINITY,
            upper: f64::INFINITY,
        }
    }

    pub fn non_neg() -> Self {
        Self {
            lower: 0f64,
            upper: f64::INFINITY,
        }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.lower <= x && x <= self.upper
    }

    pub fn surround(&self, x: f64) -> bool {
        self.lower < x && x < self.upper
    }

    pub fn clamp(&self, x: f64) -> f64 {
        x.min(self.upper).max(self.lower)
    }
}

fn unit_spherical_to_eucildian(theta: f64, phi: f64) -> Vector3<f64> {
    // Convert to Cartesian coordinates
    let x = f64::cos(phi);
    let y = f64::sin(phi) * f64::cos(theta);
    let z = f64::sin(phi) * f64::sin(theta);
    Vector3::new(x, y, z)
}

pub fn random_unit_vector() -> Vector3<f64> {
    let mut rng = thread_rng();
    // Generate random angles
    let theta = rng.gen_range(0.0..2.0 * PI);
    let phi = rng.gen_range(0.0..PI);

    unit_spherical_to_eucildian(theta, phi)
}

pub fn random_unit_vector_on_hemisphere(normal: &Vector3<f64>) -> Vector3<f64> {
    // first generate a rotation that moves vector (1,0,0) to normal vector
    let rotation = Rotation3::rotation_between(&Vector3::new(1.0, 0.0, 1.0), normal).unwrap();

    // now we can simply generate a vector on the hemisphere given by (1,0,0)
    let mut rng = thread_rng();
    // Generate random angles for the hemisphere
    let theta = rng.gen_range(0.0..PI);
    let phi = rng.gen_range(0.0..PI);

    // Create the vector
    rotation * unit_spherical_to_eucildian(theta, phi)
}
