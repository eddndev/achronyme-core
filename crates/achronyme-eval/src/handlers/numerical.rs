use achronyme_parser::ast::AstNode;
use achronyme_types::value::Value;

use crate::evaluator::Evaluator;

/// Numerical Calculus Handler
///
/// This module contains implementations of numerical differentiation,
/// integration, and root-finding methods.

/// Numerical first derivative: diff(f, x, h)
pub fn handle_diff(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("diff() requires 3 arguments: function, x, h".to_string());
    }

    let func_value = evaluator.evaluate(&args[0])?;
    let func = match func_value {
        Value::Function(f) => f,
        _ => return Err("diff() requires a function as first argument".to_string()),
    };

    let x = match evaluator.evaluate(&args[1])? {
        Value::Number(n) => n,
        _ => return Err("diff() requires a number for x".to_string()),
    };

    let h = match evaluator.evaluate(&args[2])? {
        Value::Number(n) => n,
        _ => return Err("diff() requires a number for h".to_string()),
    };

    use achronyme_numerical::diff_central;
    let result = diff_central(evaluator, &func, x, h)?;
    Ok(Value::Number(result))
}

/// Numerical second derivative: diff2(f, x, h)
pub fn handle_diff2(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("diff2() requires 3 arguments: function, x, h".to_string());
    }

    let func_value = evaluator.evaluate(&args[0])?;
    let func = match func_value {
        Value::Function(f) => f,
        _ => return Err("diff2() requires a function as first argument".to_string()),
    };

    let x = match evaluator.evaluate(&args[1])? {
        Value::Number(n) => n,
        _ => return Err("diff2() requires a number for x".to_string()),
    };

    let h = match evaluator.evaluate(&args[2])? {
        Value::Number(n) => n,
        _ => return Err("diff2() requires a number for h".to_string()),
    };

    use achronyme_numerical::diff2_central;
    let result = diff2_central(evaluator, &func, x, h)?;
    Ok(Value::Number(result))
}

/// Numerical third derivative: diff3(f, x, h)
pub fn handle_diff3(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("diff3() requires 3 arguments: function, x, h".to_string());
    }

    let func_value = evaluator.evaluate(&args[0])?;
    let func = match func_value {
        Value::Function(f) => f,
        _ => return Err("diff3() requires a function as first argument".to_string()),
    };

    let x = match evaluator.evaluate(&args[1])? {
        Value::Number(n) => n,
        _ => return Err("diff3() requires a number for x".to_string()),
    };

    let h = match evaluator.evaluate(&args[2])? {
        Value::Number(n) => n,
        _ => return Err("diff3() requires a number for h".to_string()),
    };

    use achronyme_numerical::diff3_central;
    let result = diff3_central(evaluator, &func, x, h)?;
    Ok(Value::Number(result))
}

/// Gradient: gradient(f, point, h)
pub fn handle_gradient(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("gradient() requires 3 arguments: function, point, h".to_string());
    }

    let func = match evaluator.evaluate(&args[0])? {
        Value::Function(f) => f,
        _ => return Err("gradient() requires a function as first argument".to_string()),
    };

    let point_value = evaluator.evaluate(&args[1])?;
    let point_vec = match &point_value {
        Value::Vector(v) => {
            let mut points = Vec::new();
            for val in v {
                if let Value::Number(n) = val {
                    points.push(*n);
                } else {
                    return Err("gradient() requires a numeric vector for point".to_string());
                }
            }
            points
        }
        _ => return Err("gradient() requires a vector for point".to_string()),
    };

    let h = match evaluator.evaluate(&args[2])? {
        Value::Number(n) => n,
        _ => return Err("gradient() requires a number for h".to_string()),
    };

    use achronyme_numerical::gradient as gradient_calc;
    let result = gradient_calc(evaluator, &func, &point_vec, h)?;
    Ok(Value::Vector(result.into_iter().map(Value::Number).collect()))
}

/// Numerical integration (trapezoidal): integral(f, a, b, n)
pub fn handle_integral(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 4 {
        return Err("integral() requires 4 arguments: function, a, b, n".to_string());
    }

    let func_value = evaluator.evaluate(&args[0])?;
    let func = match func_value {
        Value::Function(f) => f,
        _ => return Err("integral() requires a function as first argument".to_string()),
    };

    let a = match evaluator.evaluate(&args[1])? {
        Value::Number(n) => n,
        _ => return Err("integral() requires a number for a".to_string()),
    };

    let b = match evaluator.evaluate(&args[2])? {
        Value::Number(n) => n,
        _ => return Err("integral() requires a number for b".to_string()),
    };

    let n = match evaluator.evaluate(&args[3])? {
        Value::Number(n) => n as usize,
        _ => return Err("integral() requires a number for n".to_string()),
    };

    use achronyme_numerical::trapz;
    let result = trapz(evaluator, &func, a, b, n)?;
    Ok(Value::Number(result))
}

/// Simpson's rule integration: simpson(f, a, b, n)
pub fn handle_simpson(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 4 {
        return Err("simpson() requires 4 arguments: function, a, b, n".to_string());
    }

    let func_value = evaluator.evaluate(&args[0])?;
    let func = match func_value {
        Value::Function(f) => f,
        _ => return Err("simpson() requires a function as first argument".to_string()),
    };

    let a = match evaluator.evaluate(&args[1])? {
        Value::Number(n) => n,
        _ => return Err("simpson() requires a number for a".to_string()),
    };

    let b = match evaluator.evaluate(&args[2])? {
        Value::Number(n) => n,
        _ => return Err("simpson() requires a number for b".to_string()),
    };

    let n = match evaluator.evaluate(&args[3])? {
        Value::Number(n) => n as usize,
        _ => return Err("simpson() requires a number for n".to_string()),
    };

    use achronyme_numerical::simpson;
    let result = simpson(evaluator, &func, a, b, n)?;
    Ok(Value::Number(result))
}

/// Romberg integration: romberg(f, a, b, tol)
pub fn handle_romberg(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 4 {
        return Err("romberg() requires 4 arguments: function, a, b, tol".to_string());
    }

    let func_value = evaluator.evaluate(&args[0])?;
    let func = match func_value {
        Value::Function(f) => f,
        _ => return Err("romberg() requires a function as first argument".to_string()),
    };

    let a = match evaluator.evaluate(&args[1])? {
        Value::Number(n) => n,
        _ => return Err("romberg() requires a number for a".to_string()),
    };

    let b = match evaluator.evaluate(&args[2])? {
        Value::Number(n) => n,
        _ => return Err("romberg() requires a number for b".to_string()),
    };

    let tol = match evaluator.evaluate(&args[3])? {
        Value::Number(n) => n,
        _ => return Err("romberg() requires a number for tol".to_string()),
    };

    use achronyme_numerical::romberg;
    let result = romberg(evaluator, &func, a, b, tol, 20)?;
    Ok(Value::Number(result))
}

/// Adaptive quadrature: quad(f, a, b)
pub fn handle_quad(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("quad() requires 3 arguments: function, a, b".to_string());
    }

    let func_value = evaluator.evaluate(&args[0])?;
    let func = match func_value {
        Value::Function(f) => f,
        _ => return Err("quad() requires a function as first argument".to_string()),
    };

    let a = match evaluator.evaluate(&args[1])? {
        Value::Number(n) => n,
        _ => return Err("quad() requires a number for a".to_string()),
    };

    let b = match evaluator.evaluate(&args[2])? {
        Value::Number(n) => n,
        _ => return Err("quad() requires a number for b".to_string()),
    };

    use achronyme_numerical::quad;
    let result = quad(evaluator, &func, a, b, 1e-10)?;
    Ok(Value::Number(result))
}

/// Root finding (bisection): solve(f, a, b, tol)
pub fn handle_solve(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 4 {
        return Err("solve() requires 4 arguments: function, a, b, tol".to_string());
    }

    let func_value = evaluator.evaluate(&args[0])?;
    let func = match func_value {
        Value::Function(f) => f,
        _ => return Err("solve() requires a function as first argument".to_string()),
    };

    let a = match evaluator.evaluate(&args[1])? {
        Value::Number(n) => n,
        _ => return Err("solve() requires a number for a".to_string()),
    };

    let b = match evaluator.evaluate(&args[2])? {
        Value::Number(n) => n,
        _ => return Err("solve() requires a number for b".to_string()),
    };

    let tol = match evaluator.evaluate(&args[3])? {
        Value::Number(n) => n,
        _ => return Err("solve() requires a number for tol".to_string()),
    };

    use achronyme_numerical::bisect;
    let result = bisect(evaluator, &func, a, b, tol)?;
    Ok(Value::Number(result))
}

/// Newton's method: newton(f, df, x0, tol, max_iter)
pub fn handle_newton(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 5 {
        return Err("newton() requires 5 arguments: function, derivative, x0, tol, max_iter".to_string());
    }

    let func_value = evaluator.evaluate(&args[0])?;
    let func = match func_value {
        Value::Function(f) => f,
        _ => return Err("newton() requires a function as first argument".to_string()),
    };

    let dfunc_value = evaluator.evaluate(&args[1])?;
    let dfunc = match dfunc_value {
        Value::Function(f) => f,
        _ => return Err("newton() requires a function as second argument (derivative)".to_string()),
    };

    let x0 = match evaluator.evaluate(&args[2])? {
        Value::Number(n) => n,
        _ => return Err("newton() requires a number for x0".to_string()),
    };

    let tol = match evaluator.evaluate(&args[3])? {
        Value::Number(n) => n,
        _ => return Err("newton() requires a number for tol".to_string()),
    };

    let max_iter = match evaluator.evaluate(&args[4])? {
        Value::Number(n) => n as usize,
        _ => return Err("newton() requires a number for max_iter".to_string()),
    };

    use achronyme_numerical::newton;
    let result = newton(evaluator, &func, &dfunc, x0, tol, max_iter)?;
    Ok(Value::Number(result))
}

/// Secant method: secant(f, x0, x1, tol, max_iter)
pub fn handle_secant(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 5 {
        return Err("secant() requires 5 arguments: function, x0, x1, tol, max_iter".to_string());
    }

    let func_value = evaluator.evaluate(&args[0])?;
    let func = match func_value {
        Value::Function(f) => f,
        _ => return Err("secant() requires a function as first argument".to_string()),
    };

    let x0 = match evaluator.evaluate(&args[1])? {
        Value::Number(n) => n,
        _ => return Err("secant() requires a number for x0".to_string()),
    };

    let x1 = match evaluator.evaluate(&args[2])? {
        Value::Number(n) => n,
        _ => return Err("secant() requires a number for x1".to_string()),
    };

    let tol = match evaluator.evaluate(&args[3])? {
        Value::Number(n) => n,
        _ => return Err("secant() requires a number for tol".to_string()),
    };

    let max_iter = match evaluator.evaluate(&args[4])? {
        Value::Number(n) => n as usize,
        _ => return Err("secant() requires a number for max_iter".to_string()),
    };

    use achronyme_numerical::secant;
    let result = secant(evaluator, &func, x0, x1, tol, max_iter)?;
    Ok(Value::Number(result))
}
