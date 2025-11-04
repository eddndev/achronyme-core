use crate::complex::Complex;
use crate::vector::Vector;
use crate::matrix::Matrix;
use crate::function::Function;

#[derive(Debug)]
pub enum TypeError {
    IncompatibleTypes,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Complex(Complex),
    Vector(Vector),
    Matrix(Matrix),
    Function(Function),
}

// Conversiones automáticas con From/Into
impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Number(n)
    }
}

// Operadores sobrecargados de forma segura
impl std::ops::Add for Value {
    type Output = Result<Value, TypeError>;

    fn add(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::Vector(a), Value::Vector(b)) => a.add(&b).map(Value::Vector).map_err(|_| TypeError::IncompatibleTypes),
            // Type promotion
            (Value::Number(a), Value::Complex(b)) => {
                Ok(Value::Complex(Complex::new(a, 0.0) + b))
            }
            _ => Err(TypeError::IncompatibleTypes),
        }
    }
}