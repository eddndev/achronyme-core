use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>, // Row-major order
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatrixError {
    DimensionMismatch {
        expected: (usize, usize),
        got: (usize, usize),
    },
    NotSquare,
    Singular,
    IndexOutOfBounds,
    InvalidDimensions,
}

impl std::fmt::Display for MatrixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatrixError::DimensionMismatch { expected, got } => {
                write!(
                    f,
                    "Dimension mismatch: expected {}x{}, got {}x{}",
                    expected.0, expected.1, got.0, got.1
                )
            }
            MatrixError::NotSquare => write!(f, "Matrix must be square"),
            MatrixError::Singular => write!(f, "Matrix is singular (non-invertible)"),
            MatrixError::IndexOutOfBounds => write!(f, "Index out of bounds"),
            MatrixError::InvalidDimensions => write!(f, "Invalid matrix dimensions"),
        }
    }
}

impl std::error::Error for MatrixError {}

impl Matrix {
    /// Create a new matrix from row-major data
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Result<Self, MatrixError> {
        if data.len() != rows * cols {
            return Err(MatrixError::InvalidDimensions);
        }
        Ok(Self { rows, cols, data })
    }

    /// Create a matrix filled with zeros
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    /// Create a matrix filled with ones
    pub fn ones(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![1.0; rows * cols],
        }
    }

    /// Create an identity matrix
    pub fn identity(size: usize) -> Self {
        let mut data = vec![0.0; size * size];
        for i in 0..size {
            data[i * size + i] = 1.0;
        }
        Self {
            rows: size,
            cols: size,
            data,
        }
    }

    /// Get element at (row, col)
    pub fn get(&self, row: usize, col: usize) -> Result<f64, MatrixError> {
        if row >= self.rows || col >= self.cols {
            return Err(MatrixError::IndexOutOfBounds);
        }
        Ok(self.data[row * self.cols + col])
    }

    /// Set element at (row, col)
    pub fn set(&mut self, row: usize, col: usize, value: f64) -> Result<(), MatrixError> {
        if row >= self.rows || col >= self.cols {
            return Err(MatrixError::IndexOutOfBounds);
        }
        self.data[row * self.cols + col] = value;
        Ok(())
    }

    /// Check if matrix is square
    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    /// Matrix transpose
    pub fn transpose(&self) -> Self {
        let mut data = vec![0.0; self.rows * self.cols];
        for i in 0..self.rows {
            for j in 0..self.cols {
                data[j * self.rows + i] = self.data[i * self.cols + j];
            }
        }
        Self {
            rows: self.cols,
            cols: self.rows,
            data,
        }
    }

    /// Matrix trace (sum of diagonal elements)
    pub fn trace(&self) -> Result<f64, MatrixError> {
        if !self.is_square() {
            return Err(MatrixError::NotSquare);
        }
        Ok((0..self.rows)
            .map(|i| self.data[i * self.cols + i])
            .sum())
    }

    /// Matrix addition
    pub fn add(&self, other: &Self) -> Result<Self, MatrixError> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(MatrixError::DimensionMismatch {
                expected: (self.rows, self.cols),
                got: (other.rows, other.cols),
            });
        }
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a + b)
            .collect();
        Ok(Self {
            rows: self.rows,
            cols: self.cols,
            data,
        })
    }

    /// Matrix subtraction
    pub fn sub(&self, other: &Self) -> Result<Self, MatrixError> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(MatrixError::DimensionMismatch {
                expected: (self.rows, self.cols),
                got: (other.rows, other.cols),
            });
        }
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a - b)
            .collect();
        Ok(Self {
            rows: self.rows,
            cols: self.cols,
            data,
        })
    }

    /// Matrix multiplication
    pub fn mul(&self, other: &Self) -> Result<Self, MatrixError> {
        if self.cols != other.rows {
            return Err(MatrixError::DimensionMismatch {
                expected: (self.rows, self.cols),
                got: (other.rows, other.cols),
            });
        }

        let mut data = vec![0.0; self.rows * other.cols];
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i * self.cols + k] * other.data[k * other.cols + j];
                }
                data[i * other.cols + j] = sum;
            }
        }

        Ok(Self {
            rows: self.rows,
            cols: other.cols,
            data,
        })
    }

    /// Scalar multiplication
    pub fn scale(&self, scalar: f64) -> Self {
        Self {
            rows: self.rows,
            cols: self.cols,
            data: self.data.iter().map(|x| x * scalar).collect(),
        }
    }

    /// Element-wise multiplication (Hadamard product)
    pub fn hadamard(&self, other: &Self) -> Result<Self, MatrixError> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(MatrixError::DimensionMismatch {
                expected: (self.rows, self.cols),
                got: (other.rows, other.cols),
            });
        }
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a * b)
            .collect();
        Ok(Self {
            rows: self.rows,
            cols: self.cols,
            data,
        })
    }

    /// Negate all elements
    pub fn negate(&self) -> Self {
        Self {
            rows: self.rows,
            cols: self.cols,
            data: self.data.iter().map(|x| -x).collect(),
        }
    }

    /// Calculate determinant (for small matrices)
    pub fn determinant(&self) -> Result<f64, MatrixError> {
        if !self.is_square() {
            return Err(MatrixError::NotSquare);
        }

        match self.rows {
            1 => Ok(self.data[0]),
            2 => Ok(self.data[0] * self.data[3] - self.data[1] * self.data[2]),
            3 => {
                let a = self.data[0];
                let b = self.data[1];
                let c = self.data[2];
                let d = self.data[3];
                let e = self.data[4];
                let f = self.data[5];
                let g = self.data[6];
                let h = self.data[7];
                let i = self.data[8];
                Ok(a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g))
            }
            _ => {
                // For larger matrices, use Gaussian elimination
                // This is a simplified version
                self.determinant_lu()
            }
        }
    }

    /// Calculate determinant using LU decomposition
    fn determinant_lu(&self) -> Result<f64, MatrixError> {
        // Simplified version - for production use ndarray-linalg
        // This is just a placeholder
        Err(MatrixError::NotSquare)
    }

    /// Get a row as a vector
    pub fn row(&self, index: usize) -> Result<Vec<f64>, MatrixError> {
        if index >= self.rows {
            return Err(MatrixError::IndexOutOfBounds);
        }
        let start = index * self.cols;
        Ok(self.data[start..start + self.cols].to_vec())
    }

    /// Get a column as a vector
    pub fn col(&self, index: usize) -> Result<Vec<f64>, MatrixError> {
        if index >= self.cols {
            return Err(MatrixError::IndexOutOfBounds);
        }
        Ok((0..self.rows)
            .map(|i| self.data[i * self.cols + index])
            .collect())
    }
}

// Display formatting
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[")?;
        for i in 0..self.rows {
            write!(f, "  [")?;
            for j in 0..self.cols {
                if j > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", self.data[i * self.cols + j])?;
            }
            writeln!(f, "]")?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let id = Matrix::identity(3);
        assert_eq!(id.get(0, 0).unwrap(), 1.0);
        assert_eq!(id.get(0, 1).unwrap(), 0.0);
        assert_eq!(id.get(1, 1).unwrap(), 1.0);
    }

    #[test]
    fn test_transpose() {
        let m = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        let mt = m.transpose();
        assert_eq!(mt.rows, 3);
        assert_eq!(mt.cols, 2);
        assert_eq!(mt.get(0, 0).unwrap(), 1.0);
        assert_eq!(mt.get(1, 0).unwrap(), 2.0);
        assert_eq!(mt.get(2, 0).unwrap(), 3.0);
    }

    #[test]
    fn test_multiply() {
        let a = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let b = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]).unwrap();
        let c = a.mul(&b).unwrap();
        assert_eq!(c.get(0, 0).unwrap(), 19.0);
        assert_eq!(c.get(0, 1).unwrap(), 22.0);
        assert_eq!(c.get(1, 0).unwrap(), 43.0);
        assert_eq!(c.get(1, 1).unwrap(), 50.0);
    }

    #[test]
    fn test_determinant_2x2() {
        let m = Matrix::new(2, 2, vec![3.0, 8.0, 4.0, 6.0]).unwrap();
        let det = m.determinant().unwrap();
        assert_eq!(det, -14.0); // 3*6 - 8*4 = 18 - 32 = -14
    }
}
