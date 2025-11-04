use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Vector(pub Vec<f64>);

#[derive(Debug, Clone, PartialEq)]
pub enum VectorError {
    DimensionMismatch { expected: usize, got: usize },
    EmptyVector,
    IndexOutOfBounds,
}

impl std::fmt::Display for VectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VectorError::DimensionMismatch { expected, got } => {
                write!(f, "Dimension mismatch: expected {}, got {}", expected, got)
            }
            VectorError::EmptyVector => write!(f, "Operation on empty vector"),
            VectorError::IndexOutOfBounds => write!(f, "Index out of bounds"),
        }
    }
}

impl std::error::Error for VectorError {}

impl Vector {
    /// Create a new vector from data
    pub fn new(data: Vec<f64>) -> Self {
        Self(data)
    }

    /// Create a vector filled with zeros
    pub fn zeros(size: usize) -> Self {
        Self(vec![0.0; size])
    }

    /// Create a vector filled with ones
    pub fn ones(size: usize) -> Self {
        Self(vec![1.0; size])
    }

    /// Create a vector filled with a specific value
    pub fn filled(size: usize, value: f64) -> Self {
        Self(vec![value; size])
    }

    /// Get the length of the vector
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if the vector is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get element at index
    pub fn get(&self, index: usize) -> Option<f64> {
        self.0.get(index).copied()
    }

    /// Get the underlying data
    pub fn data(&self) -> &[f64] {
        &self.0
    }

    /// Vector addition
    pub fn add(&self, other: &Self) -> Result<Self, VectorError> {
        if self.len() != other.len() {
            return Err(VectorError::DimensionMismatch {
                expected: self.len(),
                got: other.len(),
            });
        }
        let data = self
            .0
            .iter()
            .zip(other.0.iter())
            .map(|(a, b)| a + b)
            .collect();
        Ok(Self(data))
    }

    /// Vector subtraction
    pub fn sub(&self, other: &Self) -> Result<Self, VectorError> {
        if self.len() != other.len() {
            return Err(VectorError::DimensionMismatch {
                expected: self.len(),
                got: other.len(),
            });
        }
        let data = self
            .0
            .iter()
            .zip(other.0.iter())
            .map(|(a, b)| a - b)
            .collect();
        Ok(Self(data))
    }

    /// Element-wise multiplication (Hadamard product)
    pub fn mul(&self, other: &Self) -> Result<Self, VectorError> {
        if self.len() != other.len() {
            return Err(VectorError::DimensionMismatch {
                expected: self.len(),
                got: other.len(),
            });
        }
        let data = self
            .0
            .iter()
            .zip(other.0.iter())
            .map(|(a, b)| a * b)
            .collect();
        Ok(Self(data))
    }

    /// Element-wise division
    pub fn div(&self, other: &Self) -> Result<Self, VectorError> {
        if self.len() != other.len() {
            return Err(VectorError::DimensionMismatch {
                expected: self.len(),
                got: other.len(),
            });
        }
        let data = self
            .0
            .iter()
            .zip(other.0.iter())
            .map(|(a, b)| a / b)
            .collect();
        Ok(Self(data))
    }

    /// Scalar multiplication
    pub fn scale(&self, scalar: f64) -> Self {
        Self(self.0.iter().map(|x| x * scalar).collect())
    }

    /// Dot product
    pub fn dot(&self, other: &Self) -> Result<f64, VectorError> {
        if self.len() != other.len() {
            return Err(VectorError::DimensionMismatch {
                expected: self.len(),
                got: other.len(),
            });
        }
        Ok(self.0.iter().zip(other.0.iter()).map(|(a, b)| a * b).sum())
    }

    /// Euclidean norm (L2 norm)
    pub fn norm(&self) -> f64 {
        self.0.iter().map(|x| x * x).sum::<f64>().sqrt()
    }

    /// L1 norm (Manhattan distance)
    pub fn norm_l1(&self) -> f64 {
        self.0.iter().map(|x| x.abs()).sum()
    }

    /// Sum of all elements
    pub fn sum(&self) -> f64 {
        self.0.iter().sum()
    }

    /// Mean of all elements
    pub fn mean(&self) -> Result<f64, VectorError> {
        if self.is_empty() {
            return Err(VectorError::EmptyVector);
        }
        Ok(self.sum() / self.len() as f64)
    }

    /// Maximum element
    pub fn max(&self) -> Result<f64, VectorError> {
        self.0
            .iter()
            .copied()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .ok_or(VectorError::EmptyVector)
    }

    /// Minimum element
    pub fn min(&self) -> Result<f64, VectorError> {
        self.0
            .iter()
            .copied()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .ok_or(VectorError::EmptyVector)
    }

    /// Apply a function to each element
    pub fn map<F>(&self, f: F) -> Self
    where
        F: Fn(f64) -> f64,
    {
        Self(self.0.iter().map(|&x| f(x)).collect())
    }

    /// Filter elements based on a predicate
    pub fn filter<F>(&self, f: F) -> Self
    where
        F: Fn(f64) -> bool,
    {
        Self(self.0.iter().copied().filter(|&x| f(x)).collect())
    }

    /// Negate all elements
    pub fn negate(&self) -> Self {
        Self(self.0.iter().map(|x| -x).collect())
    }
}

// Display formatting
impl fmt::Display for Vector {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = Vector::new(vec![1.0, 2.0, 3.0]);
        let b = Vector::new(vec![4.0, 5.0, 6.0]);
        let c = a.add(&b).unwrap();
        assert_eq!(c, Vector::new(vec![5.0, 7.0, 9.0]));
    }

    #[test]
    fn test_dot() {
        let a = Vector::new(vec![1.0, 2.0, 3.0]);
        let b = Vector::new(vec![4.0, 5.0, 6.0]);
        let result = a.dot(&b).unwrap();
        assert_eq!(result, 32.0); // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
    }

    #[test]
    fn test_norm() {
        let a = Vector::new(vec![3.0, 4.0]);
        assert_eq!(a.norm(), 5.0);
    }

    #[test]
    fn test_scale() {
        let a = Vector::new(vec![1.0, 2.0, 3.0]);
        let b = a.scale(2.0);
        assert_eq!(b, Vector::new(vec![2.0, 4.0, 6.0]));
    }

    #[test]
    fn test_map() {
        let a = Vector::new(vec![1.0, 2.0, 3.0]);
        let b = a.map(|x| x * 2.0);
        assert_eq!(b, Vector::new(vec![2.0, 4.0, 6.0]));
    }
}
