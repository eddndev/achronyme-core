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

    // Define the variable in the environment (immutable)
    evaluator.environment_mut().define(name.to_string(), value.clone())?;

    Ok(value)
}

/// Evaluate a mutable variable declaration (mut statement)
pub fn evaluate_mutable_declaration(
    evaluator: &mut Evaluator,
    name: &str,
    initializer: &AstNode,
) -> Result<Value, String> {
    // Evaluate the initializer
    let value = evaluator.evaluate(initializer)?;

    // Define as mutable variable in the environment
    evaluator.environment_mut().define_mutable(name.to_string(), value.clone())?;

    // Return the value (not the MutableRef wrapper)
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

    // Check if it's a constant
    if evaluator.constants().has(name) {
        return Ok(Value::Number(evaluator.constants().get(name)?));
    }

    // Check if it's a built-in function (dynamic check from registry)
    if evaluator.functions().has(name) {
        use achronyme_types::function::Function;
        return Ok(Value::Function(Function::builtin(name.to_string())));
    }

    // Check if it's a special form function (functions that require special evaluation)
    if is_special_form(name) {
        use achronyme_types::function::Function;
        return Ok(Value::Function(Function::builtin(name.to_string())));
    }

    // Not found
    Err(format!("Undefined variable or constant: {}", name))
}

/// Check if a name corresponds to a special form function
/// These are functions that require special evaluation (lazy evaluation, evaluator access, etc.)
/// and are not registered in the standard FunctionRegistry
fn is_special_form(name: &str) -> bool {
    matches!(
        name,
        // Higher-order functions (require lazy evaluation)
        "map" | "filter" | "reduce" | "pipe" |
        // Tier 2 array predicates (require lambda evaluation)
        "any" | "all" | "find" | "findIndex" | "count" |
        // Numerical calculus functions (require evaluator for lambda evaluation)
        "diff" | "diff2" | "diff3" | "gradient" | "integral" | "trapz" |
        "simpson" | "romberg" | "quad" | "solve" | "bisect" | "newton" | "secant" | "derivative" |
        // Debug functions
        "describe" |
        // Optimization functions
        "simplex" | "linprog" | "dual_simplex" | "two_phase_simplex" | "revised_simplex" |
        "objective_value" | "shadow_price" | "sensitivity_c" | "sensitivity_b" |
        "reduced_costs" | "basic_variables" | "nonbasic_variables"
    )
}
