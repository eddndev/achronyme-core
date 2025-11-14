/// Tensor indexing and slicing module
///
/// Provides indexing and slicing operations for both real and complex tensors.

pub mod real;
pub mod complex;

// Re-export main functions
pub use real::index_tensor;
pub use complex::index_complex_tensor;
