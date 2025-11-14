use crate::complex::Complex;
use crate::tensor::core::{ComplexTensor, TensorError};

impl ComplexTensor {
    /// Transpose a complex matrix
    pub fn transpose(&self) -> Result<ComplexTensor, TensorError> {
        if !self.is_matrix() {
            return Err(TensorError::DimensionMismatch {
                expected: vec![0, 0],
                got: self.shape.clone(),
            });
        }

        let rows = self.shape[0];
        let cols = self.shape[1];
        let mut data = vec![Complex::new(0.0, 0.0); rows * cols];

        for i in 0..rows {
            for j in 0..cols {
                data[j * rows + i] = self.data[i * cols + j];
            }
        }

        ComplexTensor::new(data, vec![cols, rows])
    }

    /// Hermitian transpose (conjugate transpose)
    pub fn hermitian(&self) -> Result<ComplexTensor, TensorError> {
        if !self.is_matrix() {
            return Err(TensorError::DimensionMismatch {
                expected: vec![0, 0],
                got: self.shape.clone(),
            });
        }

        let rows = self.shape[0];
        let cols = self.shape[1];
        let mut data = vec![Complex::new(0.0, 0.0); rows * cols];

        for i in 0..rows {
            for j in 0..cols {
                data[j * rows + i] = self.data[i * cols + j].conjugate();
            }
        }

        ComplexTensor::new(data, vec![cols, rows])
    }

    /// Matrix trace
    pub fn trace(&self) -> Result<Complex, TensorError> {
        if !self.is_matrix() {
            return Err(TensorError::DimensionMismatch {
                expected: vec![0, 0],
                got: self.shape.clone(),
            });
        }

        let rows = self.shape[0];
        let cols = self.shape[1];
        let min_dim = rows.min(cols);

        let mut sum = Complex::new(0.0, 0.0);
        for i in 0..min_dim {
            sum = sum + self.data[i * cols + i];
        }

        Ok(sum)
    }
}
