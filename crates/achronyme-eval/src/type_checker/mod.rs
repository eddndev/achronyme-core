//! Type Checker Module for Achronyme's Gradual Type System
//!
//! This module provides runtime type checking for the gradual type system.
//! It bridges the gap between AST-level type annotations (from the parser)
//! and runtime values.
//!
//! Key features:
//! - Union type support (value must match ANY type in the union)
//! - Structural typing for Records (extra fields are allowed)
//! - Tensor shape checking (optional shape constraints)
//! - Any type (always matches - opt-out of type checking)
//! - Null type support for optional values
//! - Automatic dereferencing of MutableRef values

mod checker;
mod display;
mod error;
mod inference;
mod validators;

#[cfg(test)]
mod tests;

// Re-export public API
pub use checker::{check_type, check_type_detailed, is_assignable};
pub use error::TypeError;
pub use inference::infer_type;
