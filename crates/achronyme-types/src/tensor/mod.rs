mod core;
mod broadcast;
mod display;
mod conversions;

pub mod constructors;
pub mod arithmetic;
pub mod vector_ops;
pub mod matrix_ops;

#[cfg(test)]
mod tests;

// Re-export main types
pub use core::{Tensor, TensorError, RealTensor, ComplexTensor};
