/// Index evaluation logic
///
/// Handles the evaluation of index access operations on various data types.

use achronyme_parser::ast::IndexArg;
use achronyme_types::value::Value;

use crate::Evaluator;

use super::types::EvaluatedIndex;
use super::vector::index_vector;
use super::string::index_string;
use super::tensor::{index_tensor, index_complex_tensor};

/// Evaluate an index access operation
pub fn evaluate_index_access(
    evaluator: &mut Evaluator,
    object: &achronyme_parser::ast::AstNode,
    indices: &[IndexArg],
) -> Result<Value, String> {
    // Evaluate the object being indexed
    let obj_value = evaluator.evaluate(object)?;

    // Evaluate all index arguments
    let mut evaluated_indices = Vec::new();
    for idx in indices {
        evaluated_indices.push(evaluate_index_arg(evaluator, idx)?);
    }

    // Apply the indexing based on the object type
    match obj_value {
        Value::Vector(ref vec) => index_vector(vec, &evaluated_indices),
        Value::Tensor(ref tensor) => index_tensor(tensor, &evaluated_indices),
        Value::ComplexTensor(ref tensor) => index_complex_tensor(tensor, &evaluated_indices),
        Value::String(ref s) => index_string(s, &evaluated_indices),
        _ => Err(format!("Cannot index into {:?}", obj_value)),
    }
}

/// Evaluate an index argument (single index or range)
fn evaluate_index_arg(
    evaluator: &mut Evaluator,
    arg: &IndexArg,
) -> Result<EvaluatedIndex, String> {
    match arg {
        IndexArg::Single(expr) => {
            let value = evaluator.evaluate(expr)?;
            match value {
                Value::Number(n) => {
                    let idx = n as isize;
                    Ok(EvaluatedIndex::Single(idx))
                }
                _ => Err("Index must be a number".to_string()),
            }
        }
        IndexArg::Range { start, end } => {
            let start_idx = if let Some(start_expr) = start {
                let value = evaluator.evaluate(start_expr)?;
                match value {
                    Value::Number(n) => Some(n as isize),
                    _ => return Err("Range start must be a number".to_string()),
                }
            } else {
                None
            };

            let end_idx = if let Some(end_expr) = end {
                let value = evaluator.evaluate(end_expr)?;
                match value {
                    Value::Number(n) => Some(n as isize),
                    _ => return Err("Range end must be a number".to_string()),
                }
            } else {
                None
            };

            Ok(EvaluatedIndex::Range { start: start_idx, end: end_idx })
        }
    }
}
