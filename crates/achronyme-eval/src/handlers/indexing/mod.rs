/// Indexing and Slicing Handler
///
/// Handles indexing and slicing operations on tensors, vectors, and arrays.
/// Supports:
/// - Single element access: tensor[0, 1, 2]
/// - Range slicing: tensor[0, .., ..]
/// - Mixed indexing: tensor[0, 1..3]
///
/// # Module Structure
///
/// - `types`: Core types like `EvaluatedIndex`
/// - `utils`: Utility functions for index normalization
/// - `evaluation`: Main evaluation logic for index access
/// - `vector`: Vector indexing operations
/// - `string`: String indexing operations
/// - `tensor`: Tensor indexing operations (real and complex)

mod types;
mod utils;
mod evaluation;
mod vector;
mod string;
mod tensor;

// Re-export the public API
pub use evaluation::evaluate_index_access;
