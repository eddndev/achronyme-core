use achronyme_parser::ast::UnaryOp;
use achronyme_types::complex::Complex;
use achronyme_types::value::Value;

/// Apply a unary operation to a value
pub fn apply(op: &UnaryOp, operand: Value) -> Result<Value, String> {
    match op {
        UnaryOp::Negate => apply_negate(operand),
        UnaryOp::Not => apply_not(operand),
    }
}

fn apply_negate(operand: Value) -> Result<Value, String> {
    match operand {
        Value::Number(n) => Ok(Value::Number(-n)),
        Value::Complex(c) => Ok(Value::Complex(Complex::new(-c.re, -c.im))),
        Value::Vector(v) => Ok(Value::Vector(v.negate())),
        Value::Matrix(m) => Ok(Value::Matrix(m.negate())),
        _ => Err("Cannot negate this type".to_string()),
    }
}

fn apply_not(operand: Value) -> Result<Value, String> {
    match operand {
        Value::Boolean(b) => Ok(Value::Boolean(!b)),
        Value::Number(n) => Ok(Value::Boolean(n == 0.0)),
        _ => Err("Logical NOT operator requires a boolean or a number".to_string()),
    }
}