use achronyme_parser::ast::UnaryOp;
use achronyme_types::complex::Complex;
use achronyme_types::value::Value;

/// Apply a unary operation to a value
pub fn apply(op: &UnaryOp, operand: Value) -> Result<Value, String> {
    match op {
        UnaryOp::Negate => apply_negate(operand),
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
