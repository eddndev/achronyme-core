use achronyme_parser::ast::AstNode;
use achronyme_types::complex::Complex;
use achronyme_types::complex_vector::ComplexVector;
use achronyme_types::matrix::Matrix;
use achronyme_types::value::Value;
use achronyme_types::vector::Vector;

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
        return Ok(Value::Vector(Vector::new(vec![])));
    }

    // Evaluate ALL elements first to determine if type promotion is needed
    let mut values = Vec::new();
    let mut has_complex = false;

    for element in elements {
        let value = evaluator.evaluate(element)?;
        match &value {
            Value::Complex(_) => has_complex = true,
            Value::Number(_) => {},
            _ => return Err("Vector elements must be numbers or complex numbers".to_string()),
        }
        values.push(value);
    }

    // Decide vector type based on ALL elements
    if has_complex {
        // Promote entire vector to ComplexVector
        let complex_data: Vec<Complex> = values.into_iter()
            .map(|v| match v {
                Value::Complex(c) => c,
                Value::Number(n) => Complex::new(n, 0.0),
                _ => unreachable!(), // Already validated above
            })
            .collect();
        Ok(Value::ComplexVector(ComplexVector::new(complex_data)))
    } else {
        // All elements are real numbers
        let real_data: Vec<f64> = values.into_iter()
            .map(|v| match v {
                Value::Number(n) => n,
                _ => unreachable!(), // Already validated above
            })
            .collect();
        Ok(Value::Vector(Vector::new(real_data)))
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
