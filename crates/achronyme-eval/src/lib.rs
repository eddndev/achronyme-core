pub mod environment;
pub mod evaluator;
pub mod constants;
pub mod functions;
mod handlers;
mod function_modules;

// Re-exports for convenience
pub use environment::Environment;
pub use evaluator::Evaluator;
