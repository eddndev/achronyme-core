use achronyme_parser::ast::AstNode;
use achronyme_types::value::Value;

use crate::evaluator::Evaluator;

/// Higher-Order Functions Handler
///
/// This module contains implementations of:
/// - map, filter, reduce, pipe (original HOFs)
/// - any, all, find, findIndex, count (Tier 2 predicates)

/// Helper: Convert a collection (Vector, Tensor, or ComplexTensor) to Vec<Value>
fn collection_to_vec(collection: Value) -> Result<Vec<Value>, String> {
    match collection {
        Value::Vector(v) => Ok(v),
        Value::Tensor(t) => {
            // Convert tensor to vector of numbers
            Ok(t.data().iter().map(|&n| Value::Number(n)).collect())
        }
        Value::ComplexTensor(ct) => {
            // Convert complex tensor to vector of complex values
            Ok(ct.data().iter().map(|&c| Value::Complex(c)).collect())
        }
        _ => Err("Expected a vector or tensor".to_string()),
    }
}

/// map(f, coll1, coll2, ...) - Apply function to elements
///
/// Multi-collection support:
///   map(f, [1,2,3]) → applies f(x) to each element
///   map(f, [1,2], [3,4]) → applies f(x,y) to pairs
///
/// Supports Vector, Tensor, and ComplexTensor.
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

    // Evaluate all collection arguments (vectors, tensors, or complex tensors)
    let mut collections: Vec<Vec<Value>> = Vec::new();
    for arg in &args[1..] {
        let coll_value = evaluator.evaluate(arg)?;
        let vec = collection_to_vec(coll_value)?;
        collections.push(vec);
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

    // Return as Vector - let type promotion handle conversion to Tensor if needed
    Ok(Value::Vector(results))
}

/// filter(predicate, collection) - Filter elements
///
/// Returns elements where predicate returns true (non-zero).
/// Supports Vector, Tensor, and ComplexTensor.
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

    // Evaluate second argument (collection)
    let collection_value = evaluator.evaluate(&args[1])?;
    let collection = collection_to_vec(collection_value)?;

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

/// reduce(f, init, collection) - Reduce collection to single value
///
/// Applies f(accumulator, element) repeatedly.
/// Supports Vector, Tensor, and ComplexTensor.
/// Initial value can be any type.
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
    let collection = collection_to_vec(collection_value)?;

    for elem in collection {
        let result = evaluator.apply_lambda(&func, vec![accumulator, elem])?;
        accumulator = result;
    }

    Ok(accumulator)
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

// ============================================================================
// Tier 2: Predicate Functions
// ============================================================================

/// any(collection, predicate) - Check if any element satisfies predicate
///
/// Returns true if at least one element matches, false otherwise.
/// Short-circuits on first match.
///
/// Examples:
/// - any([1, 2, 3, 4], x => x > 3) => true
/// - any([1, 2, 3], x => x > 10) => false
/// - any([], x => x > 0) => false
pub fn handle_any(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("any requires 2 arguments: collection and predicate".to_string());
    }

    // Evaluate collection
    let coll_value = evaluator.evaluate(&args[0])?;
    let collection = collection_to_vec(coll_value)?;

    // Evaluate predicate (must be a function)
    let predicate_value = evaluator.evaluate(&args[1])?;
    let predicate = match predicate_value {
        Value::Function(f) => f,
        _ => return Err("Second argument to any must be a function".to_string()),
    };

    // Check arity
    if predicate.arity() != 1 {
        return Err("Predicate for any must be unary (take 1 argument)".to_string());
    }

    // Test each element (short-circuit on first true)
    for item in collection {
        let result = evaluator.apply_lambda(&predicate, vec![item])?;
        match result {
            Value::Boolean(true) => return Ok(Value::Boolean(true)),
            Value::Boolean(false) => continue,
            Value::Number(n) => {
                // Truthiness: non-zero is true
                if n != 0.0 {
                    return Ok(Value::Boolean(true));
                }
            }
            _ => return Err("Predicate must return boolean or number".to_string()),
        }
    }

    Ok(Value::Boolean(false))
}

/// all(collection, predicate) - Check if all elements satisfy predicate
///
/// Returns true if all elements match, false otherwise.
/// Short-circuits on first failure.
///
/// Examples:
/// - all([2, 4, 6], x => x % 2 == 0) => true
/// - all([1, 2, 3], x => x > 0) => true
/// - all([1, 2, 3], x => x > 2) => false
/// - all([], x => x > 0) => true (vacuous truth)
pub fn handle_all(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("all requires 2 arguments: collection and predicate".to_string());
    }

    // Evaluate collection
    let coll_value = evaluator.evaluate(&args[0])?;
    let collection = collection_to_vec(coll_value)?;

    // Evaluate predicate (must be a function)
    let predicate_value = evaluator.evaluate(&args[1])?;
    let predicate = match predicate_value {
        Value::Function(f) => f,
        _ => return Err("Second argument to all must be a function".to_string()),
    };

    // Check arity
    if predicate.arity() != 1 {
        return Err("Predicate for all must be unary (take 1 argument)".to_string());
    }

    // Test each element (short-circuit on first false)
    for item in collection {
        let result = evaluator.apply_lambda(&predicate, vec![item])?;
        match result {
            Value::Boolean(false) => return Ok(Value::Boolean(false)),
            Value::Boolean(true) => continue,
            Value::Number(n) => {
                // Truthiness: zero is false
                if n == 0.0 {
                    return Ok(Value::Boolean(false));
                }
            }
            _ => return Err("Predicate must return boolean or number".to_string()),
        }
    }

    Ok(Value::Boolean(true)) // Empty array or all true
}

/// find(collection, predicate) - Find first element that satisfies predicate
///
/// Returns the first matching element, or error if not found.
///
/// Examples:
/// - find([1, 2, 3, 4], x => x > 2) => 3
/// - find([1, 2, 3], x => x > 10) => error
pub fn handle_find(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("find requires 2 arguments: collection and predicate".to_string());
    }

    // Evaluate collection
    let coll_value = evaluator.evaluate(&args[0])?;
    let collection = collection_to_vec(coll_value)?;

    // Evaluate predicate (must be a function)
    let predicate_value = evaluator.evaluate(&args[1])?;
    let predicate = match predicate_value {
        Value::Function(f) => f,
        _ => return Err("Second argument to find must be a function".to_string()),
    };

    // Check arity
    if predicate.arity() != 1 {
        return Err("Predicate for find must be unary (take 1 argument)".to_string());
    }

    // Find first matching element
    for item in collection {
        let result = evaluator.apply_lambda(&predicate, vec![item.clone()])?;
        match result {
            Value::Boolean(true) => return Ok(item),
            Value::Boolean(false) => continue,
            Value::Number(n) => {
                // Truthiness: non-zero is true
                if n != 0.0 {
                    return Ok(item);
                }
            }
            _ => return Err("Predicate must return boolean or number".to_string()),
        }
    }

    Err("Element not found".to_string())
}

/// findIndex(collection, predicate) - Find index of first matching element
///
/// Returns the index (0-based) of the first matching element, or -1 if not found.
///
/// Examples:
/// - findIndex([1, 2, 3, 4], x => x > 2) => 2
/// - findIndex([1, 2, 3], x => x > 10) => -1
pub fn handle_find_index(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("findIndex requires 2 arguments: collection and predicate".to_string());
    }

    // Evaluate collection
    let coll_value = evaluator.evaluate(&args[0])?;
    let collection = collection_to_vec(coll_value)?;

    // Evaluate predicate (must be a function)
    let predicate_value = evaluator.evaluate(&args[1])?;
    let predicate = match predicate_value {
        Value::Function(f) => f,
        _ => return Err("Second argument to findIndex must be a function".to_string()),
    };

    // Check arity
    if predicate.arity() != 1 {
        return Err("Predicate for findIndex must be unary (take 1 argument)".to_string());
    }

    // Find first matching index
    for (index, item) in collection.iter().enumerate() {
        let result = evaluator.apply_lambda(&predicate, vec![item.clone()])?;
        match result {
            Value::Boolean(true) => return Ok(Value::Number(index as f64)),
            Value::Boolean(false) => continue,
            Value::Number(n) => {
                // Truthiness: non-zero is true
                if n != 0.0 {
                    return Ok(Value::Number(index as f64));
                }
            }
            _ => return Err("Predicate must return boolean or number".to_string()),
        }
    }

    Ok(Value::Number(-1.0)) // Not found
}

/// count(collection, predicate) - Count elements that satisfy predicate
///
/// Returns the number of elements that match.
///
/// Examples:
/// - count([1, 2, 3, 4, 5], x => x > 2) => 3
/// - count([1, 2, 3], x => x > 10) => 0
pub fn handle_count(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("count requires 2 arguments: collection and predicate".to_string());
    }

    // Evaluate collection
    let coll_value = evaluator.evaluate(&args[0])?;
    let collection = collection_to_vec(coll_value)?;

    // Evaluate predicate (must be a function)
    let predicate_value = evaluator.evaluate(&args[1])?;
    let predicate = match predicate_value {
        Value::Function(f) => f,
        _ => return Err("Second argument to count must be a function".to_string()),
    };

    // Check arity
    if predicate.arity() != 1 {
        return Err("Predicate for count must be unary (take 1 argument)".to_string());
    }

    // Count matching elements
    let mut count = 0;
    for item in collection {
        let result = evaluator.apply_lambda(&predicate, vec![item])?;
        match result {
            Value::Boolean(true) => count += 1,
            Value::Boolean(false) => continue,
            Value::Number(n) => {
                // Truthiness: non-zero is true
                if n != 0.0 {
                    count += 1;
                }
            }
            _ => return Err("Predicate must return boolean or number".to_string()),
        }
    }

    Ok(Value::Number(count as f64))
}
