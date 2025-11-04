//! Linear Algebra module for Achronyme
//!
//! Provides advanced linear algebra operations using faer (100% Rust, WASM-compatible):
//! - Matrix decompositions (LU, QR, Cholesky, SVD)
//! - Eigenvalue decomposition
//! - Matrix inversion
//! - Linear system solving
//!
//! Migrated to faer for maximum performance and WASM compatibility

pub mod decompositions;
pub mod eigenvalues;
pub mod solvers;

// Re-exports for convenience
pub use decompositions::{lu_decomposition, qr_decomposition, cholesky_decomposition, svd_decomposition};
pub use eigenvalues::{eigenvalues, eigenvectors, power_iteration, qr_eigenvalues, eigen_symmetric};
pub use solvers::{inverse, solve_system, determinant_nd, is_symmetric, is_positive_definite};
