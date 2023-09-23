pub mod dielectric;
pub mod diffuse_materials;
pub mod material;
pub mod metal;
pub use dielectric::DielectricMaterial;
pub use diffuse_materials::{LambertianMaterial, SimpleDiffuseMaterial};
pub use material::{Material, MaterialRef, ScatterRecord};
pub use metal::MetalMaterial;
