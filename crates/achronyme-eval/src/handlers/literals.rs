use achronyme_parser::ast::AstNode;
use achronyme_types::complex::Complex;
use achronyme_types::matrix::Matrix;
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

/// Evaluate a vector literal
pub fn evaluate_vector(evaluator: &mut Evaluator, elements: &[AstNode]) -> Result<Value, String> {
    if elements.is_empty() {
        return Ok(Value::Vector(vec![]));
    }

    // Evaluate ALL elements first
    let mut values = Vec::new();
    for element in elements {
        let value = evaluator.evaluate(element)?;
        values.push(value);
    }

    // Validate type homogeneity and apply type promotion
    validate_and_promote_vector(values)
}

/// Validate that vector elements are type-compatible and apply type promotion
fn validate_and_promote_vector(values: Vec<Value>) -> Result<Value, String> {
    if values.is_empty() {
        return Ok(Value::Vector(vec![]));
    }

    // Categorize the types in the vector
    let has_number = values.iter().any(|v| matches!(v, Value::Number(_)));
    let has_complex = values.iter().any(|v| matches!(v, Value::Complex(_)));
    let has_edge = values.iter().any(|v| matches!(v, Value::Edge { .. }));
    let has_record = values.iter().any(|v| matches!(v, Value::Record(_)));
    let has_string = values.iter().any(|v| matches!(v, Value::String(_)));
    let has_boolean = values.iter().any(|v| matches!(v, Value::Boolean(_)));
    let has_function = values.iter().any(|v| matches!(v, Value::Function(_)));
    let has_matrix = values.iter().any(|v| matches!(v, Value::Matrix(_)));
    let has_vector = values.iter().any(|v| matches!(v, Value::Vector(_)));

    // Count how many different type categories we have
    let type_categories = vec![
        has_edge,
        has_record,
        has_string,
        has_boolean,
        has_function,
        has_matrix,
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

/// Evaluate a matrix literal
pub fn evaluate_matrix(evaluator: &mut Evaluator, rows: &[Vec<AstNode>]) -> Result<Value, String> {
    if rows.is_empty() {
        return Err("Matrix cannot be empty".to_string());
    }

    let num_rows = rows.len();
    let num_cols = rows[0].len();

    // Flatten matrix data (row-major)
    let mut data = Vec::new();

    for row in rows {
        for element in row {
            let value = evaluator.evaluate(element)?;

            // For now, only support numbers in matrices
            match value {
                Value::Number(n) => data.push(n),
                _ => return Err("Matrix elements must be numbers".to_string()),
            }
        }
    }

    Matrix::new(num_rows, num_cols, data)
        .map(Value::Matrix)
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
