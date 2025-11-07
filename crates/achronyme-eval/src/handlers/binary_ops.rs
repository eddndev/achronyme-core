use achronyme_parser::ast::BinaryOp;
use achronyme_types::complex::Complex;
use achronyme_types::value::Value;

/// Apply a binary operation to two values
pub fn apply(op: &BinaryOp, left: Value, right: Value) -> Result<Value, String> {
    match op {
        BinaryOp::Add => apply_add(left, right),
        BinaryOp::Subtract => apply_subtract(left, right),
        BinaryOp::Multiply => apply_multiply(left, right),
        BinaryOp::Divide => apply_divide(left, right),
        BinaryOp::Power => apply_power(left, right),
        BinaryOp::Modulo => apply_modulo(left, right),
        BinaryOp::Gt => apply_gt(left, right),
        BinaryOp::Lt => apply_lt(left, right),
        BinaryOp::Gte => apply_gte(left, right),
        BinaryOp::Lte => apply_lte(left, right),
        BinaryOp::Eq => apply_eq(left, right),
        BinaryOp::Neq => apply_neq(left, right),
        BinaryOp::And => apply_and(left, right),
        BinaryOp::Or => apply_or(left, right),
    }
}

fn apply_add(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
        (Value::Vector(a), Value::Vector(b)) => a
            .add(&b)
            .map(Value::Vector)
            .map_err(|e| e.to_string()),
        (Value::ComplexVector(a), Value::ComplexVector(b)) => a
            .add(&b)
            .map(Value::ComplexVector)
            .map_err(|e| e.to_string()),
        (Value::Matrix(a), Value::Matrix(b)) => a
            .add(&b)
            .map(Value::Matrix)
            .map_err(|e| e.to_string()),
        (Value::Complex(a), Value::Complex(b)) => Ok(Value::Complex(a + b)),
        // Type promotion: Number → Complex
        (Value::Number(a), Value::Complex(b)) => {
            Ok(Value::Complex(Complex::from_real(a) + b))
        }
        (Value::Complex(a), Value::Number(b)) => {
            Ok(Value::Complex(a + Complex::from_real(b)))
        }
        _ => Err("Incompatible types for addition".to_string()),
    }
}

fn apply_subtract(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
        (Value::Vector(a), Value::Vector(b)) => a
            .sub(&b)
            .map(Value::Vector)
            .map_err(|e| e.to_string()),
        (Value::ComplexVector(a), Value::ComplexVector(b)) => a
            .sub(&b)
            .map(Value::ComplexVector)
            .map_err(|e| e.to_string()),
        (Value::Matrix(a), Value::Matrix(b)) => a
            .sub(&b)
            .map(Value::Matrix)
            .map_err(|e| e.to_string()),
        (Value::Complex(a), Value::Complex(b)) => Ok(Value::Complex(a - b)),
        (Value::Number(a), Value::Complex(b)) => {
            Ok(Value::Complex(Complex::from_real(a) - b))
        }
        (Value::Complex(a), Value::Number(b)) => {
            Ok(Value::Complex(a - Complex::from_real(b)))
        }
        _ => Err("Incompatible types for subtraction".to_string()),
    }
}

fn apply_multiply(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
        (Value::Vector(a), Value::Vector(b)) => a
            .mul(&b)
            .map(Value::Vector)
            .map_err(|e| e.to_string()),
        (Value::ComplexVector(a), Value::ComplexVector(b)) => a
            .mul(&b)
            .map(Value::ComplexVector)
            .map_err(|e| e.to_string()),
        (Value::Matrix(a), Value::Matrix(b)) => a
            .mul(&b)
            .map(Value::Matrix)
            .map_err(|e| e.to_string()),
        (Value::Complex(a), Value::Complex(b)) => Ok(Value::Complex(a * b)),
        (Value::Number(a), Value::Complex(b)) => {
            Ok(Value::Complex(Complex::from_real(a) * b))
        }
        (Value::Complex(a), Value::Number(b)) => {
            Ok(Value::Complex(a * Complex::from_real(b)))
        }
        _ => Err("Incompatible types for multiplication".to_string()),
    }
}

fn apply_divide(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => {
            if b == 0.0 {
                Err("Division by zero".to_string())
            } else {
                Ok(Value::Number(a / b))
            }
        }
        (Value::Vector(a), Value::Vector(b)) => a
            .div(&b)
            .map(Value::Vector)
            .map_err(|e| e.to_string()),
        (Value::ComplexVector(a), Value::ComplexVector(b)) => a
            .div(&b)
            .map(Value::ComplexVector)
            .map_err(|e| e.to_string()),
        (Value::Complex(a), Value::Complex(b)) => Ok(Value::Complex(a / b)),
        (Value::Number(a), Value::Complex(b)) => {
            Ok(Value::Complex(Complex::from_real(a) / b))
        }
        (Value::Complex(a), Value::Number(b)) => {
            Ok(Value::Complex(a / Complex::from_real(b)))
        }
        _ => Err("Incompatible types for division".to_string()),
    }
}

fn apply_power(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.powf(b))),
        (Value::Complex(a), Value::Number(b)) => Ok(Value::Complex(a.pow(b))),
        (Value::Complex(a), Value::Complex(b)) => Ok(Value::Complex(a.pow_complex(&b))),
        (Value::Number(a), Value::Complex(b)) => {
            Ok(Value::Complex(Complex::from_real(a).pow_complex(&b)))
        }
        _ => Err("Incompatible types for power".to_string()),
    }
}

fn apply_modulo(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => {
            if b == 0.0 {
                Err("Modulo by zero".to_string())
            } else {
                Ok(Value::Number(a % b))
            }
        }
        _ => Err("Modulo operator currently only supports numbers".to_string()),
    }
}

// Comparison operators (return boolean values)
fn apply_gt(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a > b)),
        _ => Err("Comparison operators currently only support numbers".to_string()),
    }
}

fn apply_lt(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a < b)),
        _ => Err("Comparison operators currently only support numbers".to_string()),
    }
}

fn apply_gte(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a >= b)),
        _ => Err("Comparison operators currently only support numbers".to_string()),
    }
}

fn apply_lte(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a <= b)),
        _ => Err("Comparison operators currently only support numbers".to_string()),
    }
}

fn apply_eq(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a == b)),
        (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a == b)),
        _ => Err("Comparison operators currently only support numbers and booleans".to_string()),
    }
}

fn apply_neq(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a != b)),
        (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a != b)),
        _ => Err("Comparison operators currently only support numbers and booleans".to_string()),
    }
}

fn apply_and(left: Value, right: Value) -> Result<Value, String> {
    let left_bool = match left {
        Value::Boolean(b) => b,
        Value::Number(n) => n != 0.0,
        _ => return Err("Logical AND operator requires boolean or number values".to_string()),
    };
    let right_bool = match right {
        Value::Boolean(b) => b,
        Value::Number(n) => n != 0.0,
        _ => return Err("Logical AND operator requires boolean or number values".to_string()),
    };
    Ok(Value::Boolean(left_bool && right_bool))
}

fn apply_or(left: Value, right: Value) -> Result<Value, String> {
    let left_bool = match left {
        Value::Boolean(b) => b,
        Value::Number(n) => n != 0.0,
        _ => return Err("Logical OR operator requires boolean or number values".to_string()),
    };
    let right_bool = match right {
        Value::Boolean(b) => b,
        Value::Number(n) => n != 0.0,
        _ => return Err("Logical OR operator requires boolean or number values".to_string()),
    };
    Ok(Value::Boolean(left_bool || right_bool))
}