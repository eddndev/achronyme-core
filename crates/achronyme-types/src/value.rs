use crate::complex::Complex;
use crate::vector::Vector as RealVector;
use crate::complex_vector::ComplexVector;
use crate::matrix::Matrix;
use crate::function::Function;
use std::collections::HashMap;

#[derive(Debug)]
pub enum TypeError {
    IncompatibleTypes,
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeError::IncompatibleTypes => write!(f, "Incompatible types"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    Complex(Complex),
    Vector(Vec<Value>),  // Generic vector - can hold any Value type
    Matrix(Matrix),
    Function(Function),
    String(String),
    Record(HashMap<String, Value>),
    Edge {
        from: String,
        to: String,
        directed: bool,
        properties: HashMap<String, Value>,
    },
}

// Conversiones automáticas con From/Into
impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Number(n)
    }
}

// Helper functions for vector operations
impl Value {
    /// Check if a vector is numeric (contains only Number or Complex values)
    pub fn is_numeric_vector(vec: &[Value]) -> bool {
        vec.iter().all(|v| matches!(v, Value::Number(_) | Value::Complex(_)))
    }

    /// Convert a generic vector to a RealVector if all elements are numbers
    pub fn to_real_vector(vec: &[Value]) -> Result<RealVector, TypeError> {
        let nums: Result<Vec<f64>, _> = vec.iter().map(|v| match v {
            Value::Number(n) => Ok(*n),
            _ => Err(TypeError::IncompatibleTypes),
        }).collect();
        nums.map(RealVector::new)
    }

    /// Convert a generic vector to a ComplexVector if all elements are numeric
    pub fn to_complex_vector(vec: &[Value]) -> Result<ComplexVector, TypeError> {
        let complexes: Result<Vec<Complex>, _> = vec.iter().map(|v| match v {
            Value::Number(n) => Ok(Complex::new(*n, 0.0)),
            Value::Complex(c) => Ok(*c),
            _ => Err(TypeError::IncompatibleTypes),
        }).collect();
        complexes.map(ComplexVector::new)
    }

    /// Convert a RealVector to a generic vector
    pub fn from_real_vector(vec: RealVector) -> Value {
        Value::Vector(vec.data().iter().map(|&n| Value::Number(n)).collect())
    }

    /// Convert a ComplexVector to a generic vector
    pub fn from_complex_vector(vec: ComplexVector) -> Value {
        Value::Vector(vec.data().iter().map(|&c| Value::Complex(c)).collect())
    }

    pub fn as_complex(&self) -> Option<&Complex> {
        if let Value::Complex(c) = self {
            Some(c)
        } else {
            None
        }
    }
}

// Operadores sobrecargados de forma segura
impl std::ops::Add for Value {
    type Output = Result<Value, TypeError>;

    fn add(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::Vector(ref a), Value::Vector(ref b)) => {
                // Check if both vectors are numeric
                if Value::is_numeric_vector(a) && Value::is_numeric_vector(b) {
                    // Check if any element is complex
                    let has_complex_a = a.iter().any(|v| matches!(v, Value::Complex(_)));
                    let has_complex_b = b.iter().any(|v| matches!(v, Value::Complex(_)));

                    if has_complex_a || has_complex_b {
                        // Complex vector addition
                        let vec_a = Value::to_complex_vector(a)?;
                        let vec_b = Value::to_complex_vector(b)?;
                        let result = vec_a.add(&vec_b).map_err(|_| TypeError::IncompatibleTypes)?;
                        Ok(Value::from_complex_vector(result))
                    } else {
                        // Real vector addition
                        let vec_a = Value::to_real_vector(a)?;
                        let vec_b = Value::to_real_vector(b)?;
                        let result = vec_a.add(&vec_b).map_err(|_| TypeError::IncompatibleTypes)?;
                        Ok(Value::from_real_vector(result))
                    }
                } else {
                    Err(TypeError::IncompatibleTypes)
                }
            }
            // Type promotion
            (Value::Number(a), Value::Complex(b)) => {
                Ok(Value::Complex(Complex::new(a, 0.0) + b))
            }
            _ => Err(TypeError::IncompatibleTypes),
        }
    }
}