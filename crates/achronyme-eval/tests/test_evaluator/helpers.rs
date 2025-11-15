use achronyme_eval::Evaluator;
use achronyme_parser::parse;
use achronyme_types::value::Value;

pub fn eval(source: &str) -> Result<Value, String> {
    let statements = parse(source)?;
    let mut evaluator = Evaluator::new();

    // Evaluate all statements, return the last result
    let mut result = Value::Number(0.0);
    for stmt in &statements {
        result = evaluator.evaluate(stmt)?;
    }

    Ok(result)
}

/// Helper function for tests that need to maintain state across multiple eval calls
pub fn eval_with_evaluator(evaluator: &mut Evaluator, source: &str) -> Result<Value, String> {
    let statements = parse(source)?;

    // Evaluate all statements, return the last result
    let mut result = Value::Number(0.0);
    for stmt in &statements {
        result = evaluator.evaluate(stmt)?;
    }

    Ok(result)
}
