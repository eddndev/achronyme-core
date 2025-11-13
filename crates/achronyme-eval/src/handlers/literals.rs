use achronyme_parser::ast::AstNode;
use achronyme_types::complex::Complex;
use achronyme_types::tensor::RealTensor;
use achronyme_types::value::Value;

use crate::evaluator::Evaluator;

/// Evaluate a number literal
pub fn evaluate_number(n: f64) -> Result<Value, String> {
    Ok(Value::Number(n))
}

/// Evaluate a boolean literal
pub fn evaluate_boolean(b: bool) -> Result<Value, String> {
    Ok(Value::Boolean(b))
}

/// Evaluate a string literal
pub fn evaluate_string(s: &str) -> Result<Value, String> {
    Ok(Value::String(s.to_string()))
}

/// Evaluate a complex literal
pub fn evaluate_complex(re: f64, im: f64) -> Result<Value, String> {
    Ok(Value::Complex(Complex::new(re, im)))
}

/// Evaluate an array literal (unified handler for vectors, matrices, and N-D tensors)
/// Supports spread syntax: [1, ...vec, 2]
pub fn evaluate_array(evaluator: &mut Evaluator, elements: &[achronyme_parser::ast::ArrayElement]) -> Result<Value, String> {
    use achronyme_parser::ast::ArrayElement;

    if elements.is_empty() {
        return Ok(Value::Vector(vec![]));
    }

    // Evaluate ALL elements, expanding spreads
    let mut values = Vec::new();
    for element in elements {
        match element {
            ArrayElement::Single(node) => {
                let value = evaluator.evaluate(node)?;
                values.push(value);
            }
            ArrayElement::Spread(node) => {
                let spread_value = evaluator.evaluate(node)?;
                match spread_value {
                    Value::Vector(vec) => {
                        values.extend(vec);
                    }
                    Value::Tensor(tensor) => {
                        if tensor.shape().len() == 1 {
                            for &val in tensor.data() {
                                values.push(Value::Number(val));
                            }
                        } else {
                            return Err("Cannot spread multi-dimensional Tensor in array. Use concat() or reshape() instead.".to_string());
                        }
                    }
                    _ => {
                        return Err(format!("Cannot spread non-iterable value in array context. Got: {:?}", spread_value));
                    }
                }
            }
        }
    }

    // Handle empty result after spreading
    if values.is_empty() {
        return Ok(Value::Vector(vec![]));
    }

    // Check if all elements are tensors - if so, combine them into higher-dimensional tensor
    if values.iter().all(|v| matches!(v, Value::Tensor(_))) {
        return combine_tensors_to_higher_dimension(values);
    }

    // Check if all elements are numbers - create tensor directly
    if values.iter().all(|v| matches!(v, Value::Number(_))) {
        let nums: Vec<f64> = values.iter().map(|v| {
            if let Value::Number(n) = v {
                *n
            } else {
                unreachable!()
            }
        }).collect();
        return Ok(Value::Tensor(RealTensor::vector(nums)));
    }

    // Otherwise, validate type homogeneity and apply type promotion for generic vectors
    validate_and_promote_vector(values)
}

/// Validate that vector elements are type-compatible and apply type promotion
fn validate_and_promote_vector(values: Vec<Value>) -> Result<Value, String> {
    if values.is_empty() {
        return Ok(Value::Vector(vec![]));
    }

    // Special case: if all elements are tensors with the same shape, combine into higher-dimensional tensor
    if values.iter().all(|v| matches!(v, Value::Tensor(_))) {
        return combine_tensors_to_higher_dimension(values);
    }

    // Apply type promotion for numeric types if applicable
    let has_complex = values.iter().any(|v| matches!(v, Value::Complex(_)));

    // Only apply numeric promotion if ALL elements are numeric (Number or Complex)
    let all_numeric = values.iter().all(|v| matches!(v, Value::Number(_) | Value::Complex(_)));

    if all_numeric && has_complex {
        // Promote all numbers to complex for consistency
        let promoted: Vec<Value> = values.into_iter()
            .map(|v| match v {
                Value::Number(n) => Value::Complex(Complex::new(n, 0.0)),
                v => v,
            })
            .collect();
        Ok(Value::Vector(promoted))
    } else {
        // Heterogeneous vector - allow any mix of types
        Ok(Value::Vector(values))
    }
}

/// Combine tensors of the same shape into a higher-dimensional tensor
fn combine_tensors_to_higher_dimension(tensors: Vec<Value>) -> Result<Value, String> {
    if tensors.is_empty() {
        return Err("Cannot combine empty tensor list".to_string());
    }

    // Extract first tensor shape
    let first_shape = match &tensors[0] {
        Value::Tensor(t) => t.shape().to_vec(),
        _ => unreachable!(),
    };

    let n = tensors.len();

    // Collect all data and verify shapes
    let mut all_data = Vec::new();
    for (i, tensor_value) in tensors.into_iter().enumerate() {
        match tensor_value {
            Value::Tensor(t) => {
                if t.shape() != &first_shape[..] {
                    return Err(format!(
                        "Cannot combine tensors with different shapes: tensor 0 has shape {:?} but tensor {} has shape {:?}",
                        first_shape, i, t.shape()
                    ));
                }
                all_data.extend_from_slice(t.data());
            }
            _ => unreachable!(),
        }
    }

    // Build new shape: [n, ...first_shape]
    let mut new_shape = vec![n];
    new_shape.extend_from_slice(&first_shape);

    RealTensor::new(all_data, new_shape)
        .map(Value::Tensor)
        .map_err(|e| e.to_string())
}

/// Evaluate a record literal
/// Supports spread syntax: { a: 1, ...other, b: 2 }
pub fn evaluate_record(evaluator: &mut Evaluator, fields: &[achronyme_parser::ast::RecordFieldOrSpread]) -> Result<Value, String> {
    use std::collections::HashMap;
    use achronyme_parser::ast::RecordFieldOrSpread;

    let mut record = HashMap::new();

    for field in fields {
        match field {
            RecordFieldOrSpread::Field { name, value } => {
                let evaluated_value = evaluator.evaluate(value)?;
                record.insert(name.clone(), evaluated_value);
            }
            RecordFieldOrSpread::MutableField { name, value } => {
                // Evaluate the value and wrap it in a MutableRef
                let evaluated_value = evaluator.evaluate(value)?;
                let mutable_value = Value::new_mutable(evaluated_value);
                record.insert(name.clone(), mutable_value);
            }
            RecordFieldOrSpread::Spread(node) => {
                let spread_value = evaluator.evaluate(node)?;
                match spread_value {
                    Value::Record(spread_record) => {
                        for (key, value) in spread_record {
                            record.insert(key, value);
                        }
                    }
                    _ => {
                        return Err(format!("Cannot spread non-Record value in record context. Got: {:?}", spread_value));
                    }
                }
            }
        }
    }

    Ok(Value::Record(record))
}

/// Evaluate an edge literal
/// Note: from and to are pure identifiers (strings), never evaluated as variables
pub fn evaluate_edge(
    evaluator: &mut Evaluator,
    from: &str,
    to: &str,
    directed: bool,
    metadata: &Option<Box<AstNode>>,
) -> Result<Value, String> {
    use std::collections::HashMap;

    // Evaluate metadata if provided
    let properties = if let Some(metadata_node) = metadata {
        let metadata_value = evaluator.evaluate(metadata_node)?;

        // Metadata must be a record
        match metadata_value {
            Value::Record(map) => map,
            _ => return Err("Edge metadata must be a record".to_string()),
        }
    } else {
        HashMap::new()
    };

    Ok(Value::Edge {
        from: from.to_string(),
        to: to.to_string(),
        directed,
        properties,
    })
}
