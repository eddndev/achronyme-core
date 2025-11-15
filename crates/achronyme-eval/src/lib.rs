pub mod evaluator;
pub mod constants;
pub mod functions;
pub mod tco;
pub mod modules;
pub mod type_checker;
mod handlers;
mod function_modules;

// Re-exports for convenience
pub use achronyme_types::Environment;
pub use evaluator::Evaluator;
pub use modules::{Module, ModuleRegistry};
pub use type_checker::{check_type, is_assignable, check_type_detailed, infer_type};
