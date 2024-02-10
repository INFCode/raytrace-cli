pub mod intersectable;
pub mod plane;
pub mod sphere;
pub use intersectable::{IntersectRecord, Intersectable, World};
pub use plane::{InfinitePlane, Rectangle};
pub use sphere::Sphere;
