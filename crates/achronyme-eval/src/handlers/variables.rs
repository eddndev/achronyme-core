use achronyme_parser::ast::AstNode;
use achronyme_types::complex::Complex;
use achronyme_types::value::Value;

use crate::evaluator::Evaluator;

/// Evaluate a variable declaration (let statement)
pub fn evaluate_declaration(
    evaluator: &mut Evaluator,
    name: &str,
    initializer: &AstNode,
) -> Result<Value, String> {
    // Evaluate the initializer
    let value = evaluator.evaluate(initializer)?;

    // Define the variable in the environment
    evaluator.environment_mut().define(name.to_string(), value.clone())?;

    Ok(value)
}

/// Evaluate a variable reference
pub fn evaluate_reference(evaluator: &Evaluator, name: &str) -> Result<Value, String> {
    // Check if it's a variable first
    if evaluator.environment().has(name) {
        return evaluator.environment().get(name);
    }

    // Special case for imaginary unit 'i'
    if name.to_lowercase() == "i" {
        return Ok(Value::Complex(Complex::new(0.0, 1.0)));
    }

    // Otherwise, check if it's a constant
    if evaluator.constants().has(name) {
        return Ok(Value::Number(evaluator.constants().get(name)?));
    }

    // Not found
    Err(format!("Undefined variable or constant: {}", name))
}
