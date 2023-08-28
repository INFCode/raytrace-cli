use crate::color::{self, Color};
use crate::ray::Ray;

pub fn ray_color(ray: &Ray) -> Color {
    let dir = ray.direction.normalize();
    let t = 0.5 * (dir.y + 1f64);
    color::lerp(
        &Color::new(1f64, 1f64, 1f64),
        &Color::new(0.5f64, 0.7f64, 1.0f64),
        t,
    )
}
