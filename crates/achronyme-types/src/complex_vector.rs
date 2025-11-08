use crate::complex::Complex;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct ComplexVector(pub Vec<Complex>);

#[derive(Debug, Clone, PartialEq)]
pub enum ComplexVectorError {
    DimensionMismatch { expected: usize, got: usize },
    EmptyVector,
    IndexOutOfBounds,
}

impl std::fmt::Display for ComplexVectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComplexVectorError::DimensionMismatch { expected, got } => {
                write!(f, "Dimension mismatch: expected {}, got {}", expected, got)
            }
            ComplexVectorError::EmptyVector => write!(f, "Operation on empty vector"),
            ComplexVectorError::IndexOutOfBounds => write!(f, "Index out of bounds"),
        }
    }
}

impl std::error::Error for ComplexVectorError {}

impl ComplexVector {
    /// Create a new complex vector from data
    pub fn new(data: Vec<Complex>) -> Self {
        Self(data)
    }

    /// Create a complex vector filled with zeros
    pub fn zeros(size: usize) -> Self {
        Self(vec![Complex::new(0.0, 0.0); size])
    }

    /// Get the length of the vector
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if vector is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get a reference to the underlying data
    pub fn data(&self) -> &[Complex] {
        &self.0
    }

    /// Element-wise addition
    pub fn add(&self, other: &ComplexVector) -> Result<ComplexVector, ComplexVectorError> {
        if self.len() != other.len() {
            return Err(ComplexVectorError::DimensionMismatch {
                expected: self.len(),
                got: other.len(),
            });
        }

        let result: Vec<Complex> = self.0.iter()
            .zip(other.0.iter())
            .map(|(a, b)| *a + *b)
            .collect();

        Ok(ComplexVector(result))
    }

    /// Element-wise subtraction
    pub fn sub(&self, other: &ComplexVector) -> Result<ComplexVector, ComplexVectorError> {
        if self.len() != other.len() {
            return Err(ComplexVectorError::DimensionMismatch {
                expected: self.len(),
                got: other.len(),
            });
        }

        let result: Vec<Complex> = self.0.iter()
            .zip(other.0.iter())
            .map(|(a, b)| *a - *b)
            .collect();

        Ok(ComplexVector(result))
    }

    /// Element-wise multiplication
    pub fn mul(&self, other: &ComplexVector) -> Result<ComplexVector, ComplexVectorError> {
        if self.len() != other.len() {
            return Err(ComplexVectorError::DimensionMismatch {
                expected: self.len(),
                got: other.len(),
            });
        }

        let result: Vec<Complex> = self.0.iter()
            .zip(other.0.iter())
            .map(|(a, b)| *a * *b)
            .collect();

        Ok(ComplexVector(result))
    }

    /// Element-wise division
    pub fn div(&self, other: &ComplexVector) -> Result<ComplexVector, ComplexVectorError> {
        if self.len() != other.len() {
            return Err(ComplexVectorError::DimensionMismatch {
                expected: self.len(),
                got: other.len(),
            });
        }

        let result: Vec<Complex> = self.0.iter()
            .zip(other.0.iter())
            .map(|(a, b)| *a / *b)
            .collect();

        Ok(ComplexVector(result))
    }

    /// Dot product (complex conjugate of first vector)
    pub fn dot(&self, other: &ComplexVector) -> Result<Complex, ComplexVectorError> {
        if self.len() != other.len() {
            return Err(ComplexVectorError::DimensionMismatch {
                expected: self.len(),
                got: other.len(),
            });
        }

        let sum = self.0.iter()
            .zip(other.0.iter())
            .map(|(a, b)| a.conjugate() * *b)
            .fold(Complex::new(0.0, 0.0), |acc, x| acc + x);

        Ok(sum)
    }

    /// Euclidean norm (L2 norm)
    pub fn norm(&self) -> f64 {
        self.0.iter().map(|c| c.re * c.re + c.im * c.im).sum::<f64>().sqrt()
    }

    /// Normalize the vector to unit length
    pub fn normalize(&self) -> Result<Self, ComplexVectorError> {
        let norm = self.norm();
        if norm < 1e-10 {
            return Err(ComplexVectorError::EmptyVector); // Or a more specific error
        }
        let data = self.0.iter().map(|c| *c / Complex::new(norm, 0.0)).collect();
        Ok(Self(data))
    }
}

impl fmt::Display for ComplexVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, val) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", val)?;
        }
        write!(f, "]")
    }
}
