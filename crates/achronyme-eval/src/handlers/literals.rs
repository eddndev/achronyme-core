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

    // Check if all elements are numeric Vectors of the same length - create 2D tensor (matrix)
    // This handles [[1, 2], [3, 4]] → Tensor with shape [2, 2]
    if values.iter().all(|v| matches!(v, Value::Vector(_))) {
        let all_numeric_same_len = values.iter().all(|v| {
            if let Value::Vector(vec) = v {
                !vec.is_empty() && Value::is_numeric_vector(vec)
            } else {
                false
            }
        });

        if all_numeric_same_len {
            // Get the length of the first vector
            let first_len = if let Value::Vector(vec) = &values[0] {
                vec.len()
            } else {
                unreachable!()
            };

            // Check if all vectors have the same length
            let same_length = values.iter().all(|v| {
                if let Value::Vector(vec) = v {
                    vec.len() == first_len
                } else {
                    false
                }
            });

            if same_length {
                // Convert to 2D tensor
                return convert_vector_of_vectors_to_tensor(values);
            }
        }
    }

    // For simple arrays (even if all numbers), keep them as Vector for better UX
    // Only create Tensor when explicitly needed (matrices, multi-dimensional arrays)
    // This ensures that [1, 2, 3] remains a Vector, not auto-promoted to Tensor

    // Validate type homogeneity and apply type promotion for generic vectors
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

/// Convert a vector of numeric vectors into a 2D tensor (matrix)
/// Example: [[1, 2], [3, 4], [5, 6]] → Tensor with shape [3, 2]
fn convert_vector_of_vectors_to_tensor(vectors: Vec<Value>) -> Result<Value, String> {
    if vectors.is_empty() {
        return Err("Cannot convert empty vector list to tensor".to_string());
    }

    // Check if any vector contains complex numbers
    let has_complex = vectors.iter().any(|v| {
        if let Value::Vector(vec) = v {
            vec.iter().any(|elem| matches!(elem, Value::Complex(_)))
        } else {
            false
        }
    });

    let num_rows = vectors.len();
    let num_cols = if let Value::Vector(vec) = &vectors[0] {
        vec.len()
    } else {
        unreachable!()
    };

    if has_complex {
        // Create complex tensor
        use achronyme_types::complex::Complex;
        use achronyme_types::tensor::ComplexTensor;

        let mut data = Vec::with_capacity(num_rows * num_cols);
        for vector_value in vectors {
            if let Value::Vector(vec) = vector_value {
                for elem in vec {
                    let c = match elem {
                        Value::Number(n) => Complex::from_real(n),
                        Value::Complex(c) => c,
                        _ => return Err("Matrix elements must be numeric".to_string()),
                    };
                    data.push(c);
                }
            }
        }

        ComplexTensor::new(data, vec![num_rows, num_cols])
            .map(Value::ComplexTensor)
            .map_err(|e| e.to_string())
    } else {
        // Create real tensor
        let mut data = Vec::with_capacity(num_rows * num_cols);
        for vector_value in vectors {
            if let Value::Vector(vec) = vector_value {
                for elem in vec {
                    if let Value::Number(n) = elem {
                        data.push(n);
                    } else {
                        return Err("Matrix elements must be numeric".to_string());
                    }
                }
            }
        }

        RealTensor::new(data, vec![num_rows, num_cols])
            .map(Value::Tensor)
            .map_err(|e| e.to_string())
    }
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
