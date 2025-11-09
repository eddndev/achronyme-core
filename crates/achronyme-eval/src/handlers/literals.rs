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
pub fn evaluate_array(evaluator: &mut Evaluator, elements: &[AstNode]) -> Result<Value, String> {
    if elements.is_empty() {
        return Ok(Value::Vector(vec![]));
    }

    // Evaluate ALL elements first
    let mut values = Vec::new();
    for element in elements {
        let value = evaluator.evaluate(element)?;
        values.push(value);
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

    // Categorize the types in the vector
    let has_number = values.iter().any(|v| matches!(v, Value::Number(_)));
    let has_complex = values.iter().any(|v| matches!(v, Value::Complex(_)));
    let has_edge = values.iter().any(|v| matches!(v, Value::Edge { .. }));
    let has_record = values.iter().any(|v| matches!(v, Value::Record(_)));
    let has_string = values.iter().any(|v| matches!(v, Value::String(_)));
    let has_boolean = values.iter().any(|v| matches!(v, Value::Boolean(_)));
    let has_function = values.iter().any(|v| matches!(v, Value::Function(_)));
    let has_tensor = values.iter().any(|v| matches!(v, Value::Tensor(_)));
    let has_complex_tensor = values.iter().any(|v| matches!(v, Value::ComplexTensor(_)));
    let has_vector = values.iter().any(|v| matches!(v, Value::Vector(_)));

    // Count how many different type categories we have
    let type_categories = vec![
        has_edge,
        has_record,
        has_string,
        has_boolean,
        has_function,
        has_tensor,
        has_complex_tensor,
        has_vector,
    ];
    let non_numeric_types = type_categories.iter().filter(|&&x| x).count();

    // Allow Number and Complex to coexist (they can be promoted)
    let has_numeric = has_number || has_complex;

    if non_numeric_types > 1 || (non_numeric_types > 0 && has_numeric) {
        return Err("Vector elements must be of compatible types".to_string());
    }

    // Apply type promotion for numeric types
    if has_numeric {
        if has_complex {
            // Promote all numbers to complex
            let promoted: Vec<Value> = values.into_iter()
                .map(|v| match v {
                    Value::Number(n) => Value::Complex(Complex::new(n, 0.0)),
                    v => v,
                })
                .collect();
            Ok(Value::Vector(promoted))
        } else {
            // All are numbers, no promotion needed
            Ok(Value::Vector(values))
        }
    } else {
        // Non-numeric homogeneous vector (edges, records, strings, etc.)
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
pub fn evaluate_record(evaluator: &mut Evaluator, fields: &[(String, AstNode)]) -> Result<Value, String> {
    use std::collections::HashMap;

    let mut record = HashMap::new();

    for (key, value_node) in fields {
        let value = evaluator.evaluate(value_node)?;
        record.insert(key.clone(), value);
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
