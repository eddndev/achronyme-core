use achronyme_parser::ast::AstNode;
use achronyme_types::value::Value;

use crate::evaluator::Evaluator;

/// Higher-Order Functions Handler
///
/// This module contains implementations of map, filter, reduce, and pipe.

/// map(f, coll1, coll2, ...) - Apply function to elements
///
/// Multi-collection support:
///   map(f, [1,2,3]) → applies f(x) to each element
///   map(f, [1,2], [3,4]) → applies f(x,y) to pairs
///
/// Supports both real and complex vectors.
/// Truncates to shortest collection.
pub fn handle_map(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("map requires at least 2 arguments: function and collection(s)".to_string());
    }

    // Evaluate first argument (must be a function)
    let func_value = evaluator.evaluate(&args[0])?;
    let func = match func_value {
        Value::Function(f) => f,
        _ => return Err("First argument to map must be a function".to_string()),
    };

    // Evaluate all collection arguments (must be vectors)
    let mut collections: Vec<Vec<Value>> = Vec::new();
    for arg in &args[1..] {
        let coll_value = evaluator.evaluate(arg)?;
        match coll_value {
            Value::Vector(v) => {
                collections.push(v);
            }
            _ => return Err("map arguments must be vectors".to_string()),
        }
    }

    // Check arity matches number of collections
    if func.arity() != collections.len() {
        return Err(format!(
            "Function arity ({}) must match number of collections ({})",
            func.arity(),
            collections.len()
        ));
    }

    // Determine minimum length
    let min_length = collections.iter().map(|v| v.len()).min().unwrap_or(0);

    // Apply function to each element
    let mut results = Vec::new();
    for i in 0..min_length {
        // Gather arguments for this iteration
        let mut func_args = Vec::new();
        for coll in &collections {
            func_args.push(coll[i].clone());
        }

        // Apply function
        let result = evaluator.apply_lambda(&func, func_args)?;
        results.push(result);
    }

    Ok(Value::Vector(results))
}

/// filter(predicate, collection) - Filter elements
///
/// Returns elements where predicate returns true (non-zero).
/// Supports both Vector and ComplexVector.
pub fn handle_filter(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("filter requires 2 arguments: predicate and collection".to_string());
    }

    // Evaluate first argument (must be a function)
    let predicate_value = evaluator.evaluate(&args[0])?;
    let predicate = match predicate_value {
        Value::Function(f) => f,
        _ => return Err("First argument to filter must be a function".to_string()),
    };

    // Predicate must be unary
    if predicate.arity() != 1 {
        return Err("filter predicate must take exactly 1 argument".to_string());
    }

    // Evaluate second argument (must be a vector)
    let collection_value = evaluator.evaluate(&args[1])?;

    match collection_value {
        Value::Vector(collection) => {
            let mut results = Vec::new();
            for elem in collection {
                // Apply predicate
                let result = evaluator.apply_lambda(&predicate, vec![elem.clone()])?;

                // Check result (boolean or non-zero number = true)
                let should_include = match result {
                    Value::Boolean(b) => b,
                    Value::Number(n) => n != 0.0,
                    _ => return Err("filter predicate must return a boolean or number".to_string()),
                };

                if should_include {
                    results.push(elem);
                }
            }
            Ok(Value::Vector(results))
        }
        _ => Err("Second argument to filter must be a vector".to_string()),
    }
}

/// reduce(f, init, collection) - Reduce collection to single value
///
/// Applies f(accumulator, element) repeatedly.
/// Supports both Vector and ComplexVector.
/// Initial value can be Number or Complex.
pub fn handle_reduce(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("reduce requires 3 arguments: function, initial value, and collection".to_string());
    }

    // Evaluate first argument (must be a function)
    let func_value = evaluator.evaluate(&args[0])?;
    let func = match func_value {
        Value::Function(f) => f,
        _ => return Err("First argument to reduce must be a function".to_string()),
    };

    // Function must be binary
    if func.arity() != 2 {
        return Err("reduce function must take exactly 2 arguments".to_string());
    }

    // Evaluate second argument (initial value)
    let mut accumulator = evaluator.evaluate(&args[1])?;

    // Evaluate third argument (collection)
    let collection_value = evaluator.evaluate(&args[2])?;

    match collection_value {
        Value::Vector(collection) => {
            for elem in collection {
                let result = evaluator.apply_lambda(&func, vec![accumulator, elem])?;
                accumulator = result;
            }
            Ok(accumulator)
        }
        _ => Err("Third argument to reduce must be a vector".to_string()),
    }
}

/// pipe(value, f1, f2, ...) - Apply functions left-to-right
///
/// pipe(x, f, g, h) = h(g(f(x)))
///
/// First argument is the initial value, rest are unary functions.
pub fn handle_pipe(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("pipe requires at least 2 arguments: value and function(s)".to_string());
    }

    // Evaluate first argument (initial value)
    let mut result = evaluator.evaluate(&args[0])?;

    // Apply each function left-to-right
    for arg in &args[1..] {
        let func_value = evaluator.evaluate(arg)?;
        let func = match func_value {
            Value::Function(f) => f,
            _ => return Err("pipe arguments after the first must be functions".to_string()),
        };

        // Check arity
        if func.arity() != 1 {
            return Err("pipe only supports unary functions".to_string());
        }

        // Apply function to current result
        result = evaluator.apply_lambda(&func, vec![result])?;
    }

    Ok(result)
}
