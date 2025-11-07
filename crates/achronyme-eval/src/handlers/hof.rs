use achronyme_parser::ast::AstNode;
use achronyme_types::value::Value;
use achronyme_types::vector::Vector;
use achronyme_types::complex::Complex;
use achronyme_types::complex_vector::ComplexVector;

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

    // Evaluate all collection arguments (must be vectors - real or complex)
    enum Collection {
        Real(Vector),
        Complex(ComplexVector),
    }

    let mut collections: Vec<Collection> = Vec::new();
    let mut min_length = usize::MAX;
    let mut has_complex = false;

    for arg in &args[1..] {
        let coll_value = evaluator.evaluate(arg)?;
        match coll_value {
            Value::Vector(v) => {
                min_length = min_length.min(v.len());
                collections.push(Collection::Real(v));
            }
            Value::ComplexVector(cv) => {
                min_length = min_length.min(cv.len());
                collections.push(Collection::Complex(cv));
                has_complex = true;
            }
            _ => return Err("map arguments must be vectors (real or complex)".to_string()),
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

    // Apply function to each element
    let mut real_results = Vec::new();
    let mut complex_results = Vec::new();
    let mut result_is_complex = false;

    for i in 0..min_length {
        // Gather arguments for this iteration
        let mut func_args = Vec::new();
        for coll in &collections {
            match coll {
                Collection::Real(v) => {
                    func_args.push(Value::Number(v.data()[i]));
                }
                Collection::Complex(cv) => {
                    func_args.push(Value::Complex(cv.data()[i]));
                }
            }
        }

        // Apply function
        let result = evaluator.apply_lambda(&func, func_args)?;

        // Store result
        match result {
            Value::Number(n) => {
                real_results.push(n);
                complex_results.push(Complex::new(n, 0.0));
            }
            Value::Complex(c) => {
                complex_results.push(c);
                result_is_complex = true;
            }
            _ => return Err("map function must return numbers or complex numbers".to_string()),
        }
    }

    // Return complex vector if any input was complex or result is complex
    if has_complex || result_is_complex {
        Ok(Value::ComplexVector(ComplexVector::new(complex_results)))
    } else {
        Ok(Value::Vector(Vector::new(real_results)))
    }
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

    // Evaluate second argument (must be a vector - real or complex)
    let collection_value = evaluator.evaluate(&args[1])?;

    match collection_value {
        Value::Vector(collection) => {
            // Filter real vector
            let mut results = Vec::new();
            for i in 0..collection.len() {
                let elem = Value::Number(collection.data()[i]);

                // Apply predicate
                let result = evaluator.apply_lambda(&predicate, vec![elem])?;

                // Check result (boolean or non-zero number = true)
                let should_include = match result {
                    Value::Boolean(b) => b,
                    Value::Number(n) => n != 0.0,
                    _ => return Err("filter predicate must return a boolean or number".to_string()),
                };

                if should_include {
                    results.push(collection.data()[i]);
                }
            }

            Ok(Value::Vector(Vector::new(results)))
        }
        Value::ComplexVector(collection) => {
            // Filter complex vector
            let mut results = Vec::new();
            for i in 0..collection.len() {
                let elem = Value::Complex(collection.data()[i]);

                // Apply predicate
                let result = evaluator.apply_lambda(&predicate, vec![elem])?;

                // Check result (boolean or non-zero number = true)
                let should_include = match result {
                    Value::Boolean(b) => b,
                    Value::Number(n) => n != 0.0,
                    _ => return Err("filter predicate must return a boolean or number".to_string()),
                };

                if should_include {
                    results.push(collection.data()[i]);
                }
            }

            Ok(Value::ComplexVector(ComplexVector::new(results)))
        }
        _ => Err("Second argument to filter must be a vector (real or complex)".to_string()),
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
    let init_value = evaluator.evaluate(&args[1])?;

    // Evaluate third argument (collection)
    let collection_value = evaluator.evaluate(&args[2])?;

    match (init_value, collection_value) {
        // Number accumulator, real vector
        (Value::Number(mut accumulator), Value::Vector(collection)) => {
            for i in 0..collection.len() {
                let acc_val = Value::Number(accumulator);
                let elem_val = Value::Number(collection.data()[i]);

                let result = evaluator.apply_lambda(&func, vec![acc_val, elem_val])?;

                match result {
                    Value::Number(n) => accumulator = n,
                    _ => return Err("reduce function must return a number when accumulator is a number".to_string()),
                }
            }
            Ok(Value::Number(accumulator))
        }
        // Complex accumulator, real vector (promotes to complex)
        (Value::Complex(mut accumulator), Value::Vector(collection)) => {
            for i in 0..collection.len() {
                let acc_val = Value::Complex(accumulator);
                let elem_val = Value::Number(collection.data()[i]);

                let result = evaluator.apply_lambda(&func, vec![acc_val, elem_val])?;

                match result {
                    Value::Complex(c) => accumulator = c,
                    Value::Number(n) => accumulator = Complex::new(n, 0.0),
                    _ => return Err("reduce function must return a number or complex".to_string()),
                }
            }
            Ok(Value::Complex(accumulator))
        }
        // Number accumulator, complex vector (promotes to complex)
        (Value::Number(n), Value::ComplexVector(collection)) => {
            let mut accumulator = Complex::new(n, 0.0);
            for i in 0..collection.len() {
                let acc_val = Value::Complex(accumulator);
                let elem_val = Value::Complex(collection.data()[i]);

                let result = evaluator.apply_lambda(&func, vec![acc_val, elem_val])?;

                match result {
                    Value::Complex(c) => accumulator = c,
                    Value::Number(n) => accumulator = Complex::new(n, 0.0),
                    _ => return Err("reduce function must return a number or complex".to_string()),
                }
            }
            Ok(Value::Complex(accumulator))
        }
        // Complex accumulator, complex vector
        (Value::Complex(mut accumulator), Value::ComplexVector(collection)) => {
            for i in 0..collection.len() {
                let acc_val = Value::Complex(accumulator);
                let elem_val = Value::Complex(collection.data()[i]);

                let result = evaluator.apply_lambda(&func, vec![acc_val, elem_val])?;

                match result {
                    Value::Complex(c) => accumulator = c,
                    Value::Number(n) => accumulator = Complex::new(n, 0.0),
                    _ => return Err("reduce function must return a number or complex".to_string()),
                }
            }
            Ok(Value::Complex(accumulator))
        }
        _ => Err("reduce requires initial value (number or complex) and collection (vector or complex vector)".to_string()),
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
