#[cfg(test)]
pub mod material;

// Re-export the contents of the test_specific submodule
// so they can be accessed directly through `test_utils::*`
#[cfg(test)]
pub use material::DummyMaterial;
