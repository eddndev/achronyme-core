use crate::complex::Complex;
use crate::tensor::{RealTensor, ComplexTensor};
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
    Vector(Vec<Value>),  // Generic vector - can hold any Value type (strings, records, etc.)
    Tensor(RealTensor),  // Optimized N-dimensional array of real numbers
    ComplexTensor(ComplexTensor),  // Optimized N-dimensional array of complex numbers
    Function(Function),  // Both user-defined lambdas and built-in functions
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

    /// Convert a generic vector to a RealTensor (rank 1)
    pub fn to_real_tensor(vec: &[Value]) -> Result<RealTensor, TypeError> {
        let nums: Result<Vec<f64>, _> = vec.iter().map(|v| match v {
            Value::Number(n) => Ok(*n),
            _ => Err(TypeError::IncompatibleTypes),
        }).collect();

        nums.and_then(|data| {
            let len = data.len();
            RealTensor::new(data, vec![len]).map_err(|_| TypeError::IncompatibleTypes)
        })
    }

    /// Convert a generic vector to a ComplexTensor (rank 1)
    pub fn to_complex_tensor(vec: &[Value]) -> Result<ComplexTensor, TypeError> {
        let complexes: Result<Vec<Complex>, _> = vec.iter().map(|v| match v {
            Value::Number(n) => Ok(Complex::new(*n, 0.0)),
            Value::Complex(c) => Ok(*c),
            _ => Err(TypeError::IncompatibleTypes),
        }).collect();

        complexes.and_then(|data| {
            let len = data.len();
            ComplexTensor::new(data, vec![len]).map_err(|_| TypeError::IncompatibleTypes)
        })
    }

    /// Convert a RealTensor to a generic vector (if rank 1)
    pub fn from_real_tensor(tensor: RealTensor) -> Value {
        if tensor.is_vector() {
            Value::Vector(tensor.data().iter().map(|&n| Value::Number(n)).collect())
        } else {
            Value::Tensor(tensor)
        }
    }

    /// Convert a ComplexTensor to a generic vector (if rank 1) or keep as ComplexTensor
    pub fn from_complex_tensor(tensor: ComplexTensor) -> Value {
        if tensor.is_vector() {
            Value::Vector(tensor.data().iter().map(|&c| Value::Complex(c)).collect())
        } else {
            Value::ComplexTensor(tensor)
        }
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
                        // Complex tensor addition
                        let tensor_a = Value::to_complex_tensor(a)?;
                        let tensor_b = Value::to_complex_tensor(b)?;
                        let result = tensor_a.add(&tensor_b).map_err(|_| TypeError::IncompatibleTypes)?;
                        Ok(Value::ComplexTensor(result))
                    } else {
                        // Real tensor addition
                        let tensor_a = Value::to_real_tensor(a)?;
                        let tensor_b = Value::to_real_tensor(b)?;
                        let result = tensor_a.add(&tensor_b).map_err(|_| TypeError::IncompatibleTypes)?;
                        Ok(Value::Tensor(result))
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