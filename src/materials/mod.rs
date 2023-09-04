pub mod diffuse_materials;
pub mod material;
pub use diffuse_materials::{LambertianMaterial, SimpleDiffuseMaterial};
pub use material::{Material, MaterialRef, ScatterRecord};
