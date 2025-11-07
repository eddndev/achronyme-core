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

/// Evaluate a complex literal
pub fn evaluate_complex(re: f64, im: f64) -> Result<Value, String> {
    Ok(Value::Complex(Complex::new(re, im)))
}

/// Evaluate a vector literal
pub fn evaluate_vector(evaluator: &mut Evaluator, elements: &[AstNode]) -> Result<Value, String> {
    if elements.is_empty() {
        return Ok(Value::Vector(Vector::new(vec![])));
    }

    // Evaluate first element to determine vector type
    let first_value = evaluator.evaluate(&elements[0])?;

    match first_value {
        Value::Number(n) => {
            // Real vector
            let mut data = vec![n];
            for element in &elements[1..] {
                let value = evaluator.evaluate(element)?;
                match value {
                    Value::Number(num) => data.push(num),
                    _ => return Err("All vector elements must be of the same type (numbers)".to_string()),
                }
            }
            Ok(Value::Vector(Vector::new(data)))
        }
        Value::Complex(c) => {
            // Complex vector
            let mut data = vec![c];
            for element in &elements[1..] {
                let value = evaluator.evaluate(element)?;
                match value {
                    Value::Complex(complex) => data.push(complex),
                    Value::Number(n) => {
                        // Auto-promote real numbers to complex
                        data.push(Complex::new(n, 0.0));
                    }
                    _ => return Err("All vector elements must be numbers or complex numbers".to_string()),
                }
            }
            Ok(Value::ComplexVector(ComplexVector::new(data)))
        }
        _ => Err("Vector elements must be numbers or complex numbers".to_string()),
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
