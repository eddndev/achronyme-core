use crate::tensor::core::{RealTensor, TensorError};

// ============================================================================
// Matrix Operations (rank 2 tensors)
// ============================================================================

impl RealTensor {
    /// Transpose a matrix
    pub fn transpose(&self) -> Result<RealTensor, TensorError> {
        if !self.is_matrix() {
            return Err(TensorError::DimensionMismatch {
                expected: vec![0, 0],
                got: self.shape.clone(),
            });
        }

        let rows = self.shape[0];
        let cols = self.shape[1];
        let mut data = vec![0.0; rows * cols];

        for i in 0..rows {
            for j in 0..cols {
                data[j * rows + i] = self.data[i * cols + j];
            }
        }

        RealTensor::new(data, vec![cols, rows])
    }

    /// Matrix trace (sum of diagonal elements)
    pub fn trace(&self) -> Result<f64, TensorError> {
        if !self.is_matrix() {
            return Err(TensorError::DimensionMismatch {
                expected: vec![0, 0],
                got: self.shape.clone(),
            });
        }

        let rows = self.shape[0];
        let cols = self.shape[1];
        let min_dim = rows.min(cols);

        let mut sum = 0.0;
        for i in 0..min_dim {
            sum += self.data[i * cols + i];
        }

        Ok(sum)
    }

    /// Matrix multiplication
    pub fn matmul(&self, other: &RealTensor) -> Result<RealTensor, TensorError> {
        if !self.is_matrix() || !other.is_matrix() {
            return Err(TensorError::DimensionMismatch {
                expected: vec![0, 0],
                got: self.shape.clone(),
            });
        }

        let m = self.shape[0];
        let n = self.shape[1];
        let p = other.shape[1];

        if n != other.shape[0] {
            return Err(TensorError::DimensionMismatch {
                expected: vec![n, 0],
                got: other.shape.clone(),
            });
        }

        let mut data = vec![0.0; m * p];

        for i in 0..m {
            for j in 0..p {
                let mut sum = 0.0;
                for k in 0..n {
                    sum += self.data[i * n + k] * other.data[k * p + j];
                }
                data[i * p + j] = sum;
            }
        }

        RealTensor::new(data, vec![m, p])
    }
}
