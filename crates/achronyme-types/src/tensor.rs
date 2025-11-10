use crate::complex::Complex;
use std::fmt;

/// Generic N-dimensional tensor with efficient storage and operations
#[derive(Debug, Clone, PartialEq)]
pub struct Tensor<T> {
    pub data: Vec<T>,           // Flat storage in row-major order
    pub shape: Vec<usize>,      // Dimensions [d0, d1, d2, ...]
    pub strides: Vec<usize>,    // Strides for efficient indexing
}

// Type aliases for common cases
pub type RealTensor = Tensor<f64>;
pub type ComplexTensor = Tensor<Complex>;

#[derive(Debug, Clone, PartialEq)]
pub enum TensorError {
    DimensionMismatch {
        expected: Vec<usize>,
        got: Vec<usize>,
    },
    InvalidShape {
        shape: Vec<usize>,
        data_len: usize,
    },
    IndexOutOfBounds {
        index: Vec<usize>,
        shape: Vec<usize>,
    },
    EmptyTensor,
    InvalidReshape {
        old_shape: Vec<usize>,
        new_shape: Vec<usize>,
    },
    BroadcastError {
        shape1: Vec<usize>,
        shape2: Vec<usize>,
    },
}

impl std::fmt::Display for TensorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TensorError::DimensionMismatch { expected, got } => {
                write!(f, "Dimension mismatch: expected {:?}, got {:?}", expected, got)
            }
            TensorError::InvalidShape { shape, data_len } => {
                write!(
                    f,
                    "Invalid shape {:?} for data length {}",
                    shape, data_len
                )
            }
            TensorError::IndexOutOfBounds { index, shape } => {
                write!(f, "Index {:?} out of bounds for shape {:?}", index, shape)
            }
            TensorError::EmptyTensor => write!(f, "Operation on empty tensor"),
            TensorError::InvalidReshape { old_shape, new_shape } => {
                write!(
                    f,
                    "Cannot reshape tensor from {:?} to {:?}",
                    old_shape, new_shape
                )
            }
            TensorError::BroadcastError { shape1, shape2 } => {
                write!(f, "Cannot broadcast shapes {:?} and {:?}", shape1, shape2)
            }
        }
    }
}

impl std::error::Error for TensorError {}

impl<T: Clone> Tensor<T> {
    /// Create a new tensor from data and shape
    /// Data is stored in row-major order
    pub fn new(data: Vec<T>, shape: Vec<usize>) -> Result<Self, TensorError> {
        let expected_len: usize = shape.iter().product();

        if data.len() != expected_len {
            return Err(TensorError::InvalidShape {
                shape: shape.clone(),
                data_len: data.len(),
            });
        }

        let strides = Self::compute_strides(&shape);

        Ok(Self {
            data,
            shape,
            strides,
        })
    }

    /// Compute strides from shape (row-major order)
    fn compute_strides(shape: &[usize]) -> Vec<usize> {
        let mut strides = vec![1; shape.len()];
        for i in (0..shape.len().saturating_sub(1)).rev() {
            strides[i] = strides[i + 1] * shape[i + 1];
        }
        strides
    }

    /// Get the rank (number of dimensions) of the tensor
    /// 0 = scalar, 1 = vector, 2 = matrix, 3+ = higher-order tensor
    pub fn rank(&self) -> usize {
        self.shape.len()
    }

    /// Get total number of elements
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Check if tensor is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Check if tensor is a scalar (rank 0)
    pub fn is_scalar(&self) -> bool {
        self.rank() == 0
    }

    /// Check if tensor is a vector (rank 1)
    pub fn is_vector(&self) -> bool {
        self.rank() == 1
    }

    /// Check if tensor is a matrix (rank 2)
    pub fn is_matrix(&self) -> bool {
        self.rank() == 2
    }

    /// Convert multi-dimensional index to flat index
    fn ravel_index(&self, index: &[usize]) -> Result<usize, TensorError> {
        if index.len() != self.shape.len() {
            return Err(TensorError::IndexOutOfBounds {
                index: index.to_vec(),
                shape: self.shape.clone(),
            });
        }

        for (i, &idx) in index.iter().enumerate() {
            if idx >= self.shape[i] {
                return Err(TensorError::IndexOutOfBounds {
                    index: index.to_vec(),
                    shape: self.shape.clone(),
                });
            }
        }

        let flat_idx = index
            .iter()
            .zip(self.strides.iter())
            .map(|(i, s)| i * s)
            .sum();

        Ok(flat_idx)
    }

    /// Get element at given index
    pub fn get(&self, index: &[usize]) -> Result<&T, TensorError> {
        let flat_idx = self.ravel_index(index)?;
        Ok(&self.data[flat_idx])
    }

    /// Set element at given index
    pub fn set(&mut self, index: &[usize], value: T) -> Result<(), TensorError> {
        let flat_idx = self.ravel_index(index)?;
        self.data[flat_idx] = value;
        Ok(())
    }

    /// Reshape tensor to new shape (must preserve total size)
    pub fn reshape(&self, new_shape: Vec<usize>) -> Result<Self, TensorError> {
        let old_size: usize = self.shape.iter().product();
        let new_size: usize = new_shape.iter().product();

        if old_size != new_size {
            return Err(TensorError::InvalidReshape {
                old_shape: self.shape.clone(),
                new_shape,
            });
        }

        Self::new(self.data.clone(), new_shape)
    }

    /// Get a reference to the underlying data
    pub fn data(&self) -> &[T] {
        &self.data
    }

    /// Get shape
    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    /// Get strides
    pub fn strides(&self) -> &[usize] {
        &self.strides
    }
}

// Specialized constructors for RealTensor
impl RealTensor {
    /// Create a tensor filled with zeros
    pub fn zeros(shape: Vec<usize>) -> Self {
        let size: usize = shape.iter().product();
        let data = vec![0.0; size];
        Self::new(data, shape).expect("zeros: invalid shape")
    }

    /// Create a tensor filled with ones
    pub fn ones(shape: Vec<usize>) -> Self {
        let size: usize = shape.iter().product();
        let data = vec![1.0; size];
        Self::new(data, shape).expect("ones: invalid shape")
    }

    /// Create a tensor filled with a specific value
    pub fn filled(shape: Vec<usize>, value: f64) -> Self {
        let size: usize = shape.iter().product();
        let data = vec![value; size];
        Self::new(data, shape).expect("filled: invalid shape")
    }

    /// Create an identity matrix (rank 2 tensor)
    pub fn eye(n: usize) -> Self {
        let mut data = vec![0.0; n * n];
        for i in 0..n {
            data[i * n + i] = 1.0;
        }
        Self::new(data, vec![n, n]).expect("eye: invalid shape")
    }

    /// Create a scalar tensor
    pub fn scalar(value: f64) -> Self {
        Self::new(vec![value], vec![]).expect("scalar: invalid shape")
    }

    /// Create a vector tensor from a Vec<f64>
    pub fn vector(data: Vec<f64>) -> Self {
        let len = data.len();
        Self::new(data, vec![len]).expect("vector: invalid shape")
    }

    /// Create a matrix tensor from data and dimensions
    pub fn matrix(rows: usize, cols: usize, data: Vec<f64>) -> Result<Self, TensorError> {
        Self::new(data, vec![rows, cols])
    }

    /// Get the number of rows (for rank-2 tensors)
    pub fn rows(&self) -> usize {
        if self.is_matrix() {
            self.shape[0]
        } else {
            0
        }
    }

    /// Get the number of columns (for rank-2 tensors)
    pub fn cols(&self) -> usize {
        if self.is_matrix() {
            self.shape[1]
        } else {
            0
        }
    }

    /// Check if matrix is square (for rank-2 tensors)
    pub fn is_square(&self) -> bool {
        self.is_matrix() && self.shape[0] == self.shape[1]
    }

    /// Get matrix element at (row, col) - convenience method for rank-2 tensors
    pub fn get_matrix(&self, row: usize, col: usize) -> Result<f64, TensorError> {
        if !self.is_matrix() {
            return Err(TensorError::DimensionMismatch {
                expected: vec![0, 0],
                got: self.shape.clone(),
            });
        }
        self.get(&[row, col]).map(|&v| v)
    }

    /// Set matrix element at (row, col) - convenience method for rank-2 tensors
    pub fn set_matrix(&mut self, row: usize, col: usize, value: f64) -> Result<(), TensorError> {
        if !self.is_matrix() {
            return Err(TensorError::DimensionMismatch {
                expected: vec![0, 0],
                got: self.shape.clone(),
            });
        }
        self.set(&[row, col], value)
    }

    /// Get a row from a matrix as a vector
    pub fn row(&self, index: usize) -> Result<Vec<f64>, TensorError> {
        if !self.is_matrix() {
            return Err(TensorError::DimensionMismatch {
                expected: vec![0, 0],
                got: self.shape.clone(),
            });
        }
        if index >= self.shape[0] {
            return Err(TensorError::IndexOutOfBounds {
                index: vec![index],
                shape: self.shape.clone(),
            });
        }
        let cols = self.shape[1];
        let start = index * cols;
        Ok(self.data[start..start + cols].to_vec())
    }

    /// Get a column from a matrix as a vector
    pub fn col(&self, index: usize) -> Result<Vec<f64>, TensorError> {
        if !self.is_matrix() {
            return Err(TensorError::DimensionMismatch {
                expected: vec![0, 0],
                got: self.shape.clone(),
            });
        }
        if index >= self.shape[1] {
            return Err(TensorError::IndexOutOfBounds {
                index: vec![index],
                shape: self.shape.clone(),
            });
        }
        let rows = self.shape[0];
        let cols = self.shape[1];
        let mut column = Vec::with_capacity(rows);
        for row in 0..rows {
            column.push(self.data[row * cols + index]);
        }
        Ok(column)
    }

    /// Scalar multiplication (convenience method, same as mul_scalar)
    pub fn scale(&self, scalar: f64) -> RealTensor {
        self.mul_scalar(scalar)
    }

    /// Hadamard product (element-wise multiplication, same as mul)
    pub fn hadamard(&self, other: &RealTensor) -> Result<RealTensor, TensorError> {
        self.mul(other)
    }

    /// Matrix determinant (only for square rank-2 tensors)
    pub fn determinant(&self) -> Result<f64, TensorError> {
        if !self.is_square() {
            return Err(TensorError::DimensionMismatch {
                expected: vec![0, 0],
                got: self.shape.clone(),
            });
        }

        let n = self.shape[0];

        // Base cases
        if n == 1 {
            return Ok(self.data[0]);
        }
        if n == 2 {
            return Ok(self.data[0] * self.data[3] - self.data[1] * self.data[2]);
        }

        // For larger matrices, use LU decomposition or cofactor expansion
        // Simple cofactor expansion for now (can be optimized later)
        let mut det = 0.0;
        for j in 0..n {
            let minor = self.minor(0, j)?;
            let sign = if j % 2 == 0 { 1.0 } else { -1.0 };
            det += sign * self.data[j] * minor.determinant()?;
        }
        Ok(det)
    }

    /// Get minor matrix by removing row and column
    fn minor(&self, row: usize, col: usize) -> Result<RealTensor, TensorError> {
        if !self.is_matrix() {
            return Err(TensorError::DimensionMismatch {
                expected: vec![0, 0],
                got: self.shape.clone(),
            });
        }

        let n = self.shape[0];
        let mut data = Vec::with_capacity((n - 1) * (n - 1));

        for i in 0..n {
            if i == row {
                continue;
            }
            for j in 0..n {
                if j == col {
                    continue;
                }
                data.push(self.data[i * n + j]);
            }
        }

        RealTensor::matrix(n - 1, n - 1, data)
    }
}

// Specialized constructors for ComplexTensor
impl ComplexTensor {
    /// Create a complex tensor filled with zeros
    pub fn zeros(shape: Vec<usize>) -> Self {
        let size: usize = shape.iter().product();
        let data = vec![Complex::new(0.0, 0.0); size];
        Self::new(data, shape).expect("zeros: invalid shape")
    }

    /// Create a complex tensor filled with ones
    pub fn ones(shape: Vec<usize>) -> Self {
        let size: usize = shape.iter().product();
        let data = vec![Complex::new(1.0, 0.0); size];
        Self::new(data, shape).expect("ones: invalid shape")
    }

    /// Create a complex scalar tensor
    pub fn scalar(value: Complex) -> Self {
        Self::new(vec![value], vec![]).expect("scalar: invalid shape")
    }

    /// Create a complex vector tensor
    pub fn vector(data: Vec<Complex>) -> Self {
        let len = data.len();
        Self::new(data, vec![len]).expect("vector: invalid shape")
    }

    /// Create a complex identity matrix
    pub fn eye(n: usize) -> Self {
        let mut data = vec![Complex::new(0.0, 0.0); n * n];
        for i in 0..n {
            data[i * n + i] = Complex::new(1.0, 0.0);
        }
        Self::new(data, vec![n, n]).expect("eye: invalid shape")
    }
}

// Display formatting for RealTensor
impl fmt::Display for RealTensor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.rank() {
            0 => {
                // Scalar
                write!(f, "{}", self.data[0])
            }
            1 => {
                // Vector
                write!(f, "[")?;
                for (i, val) in self.data.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", val)?;
                }
                write!(f, "]")
            }
            2 => {
                // Matrix
                let rows = self.shape[0];
                let cols = self.shape[1];
                write!(f, "[")?;
                for i in 0..rows {
                    if i > 0 {
                        write!(f, "\n ")?;
                    }
                    write!(f, "[")?;
                    for j in 0..cols {
                        if j > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", self.data[i * cols + j])?;
                    }
                    write!(f, "]")?;
                }
                write!(f, "]")
            }
            _ => {
                // Higher-order tensor (3D+)
                format_nd_tensor(f, &self.data, &self.shape, 0, 0)
            }
        }
    }
}

/// Recursively format N-dimensional tensors
fn format_nd_tensor(
    f: &mut fmt::Formatter<'_>,
    data: &[f64],
    shape: &[usize],
    depth: usize,
    offset: usize,
) -> fmt::Result {
    if shape.is_empty() {
        return write!(f, "{}", data[offset]);
    }

    if shape.len() == 1 {
        // Last dimension - print as vector
        write!(f, "[")?;
        for i in 0..shape[0] {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", data[offset + i])?;
        }
        write!(f, "]")
    } else {
        // Multiple dimensions remaining
        let current_dim = shape[0];
        let stride: usize = shape[1..].iter().product();

        write!(f, "[")?;
        for i in 0..current_dim {
            if i > 0 {
                write!(f, ",")?;
                // Add newline and indentation for readability
                write!(f, "\n")?;
                for _ in 0..=depth {
                    write!(f, " ")?;
                }
            }
            format_nd_tensor(f, data, &shape[1..], depth + 1, offset + i * stride)?;
        }
        write!(f, "]")
    }
}

// Display formatting for ComplexTensor
impl fmt::Display for ComplexTensor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.rank() {
            0 => {
                // Scalar
                write!(f, "{}", self.data[0])
            }
            1 => {
                // Vector
                write!(f, "[")?;
                for (i, val) in self.data.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", val)?;
                }
                write!(f, "]")
            }
            2 => {
                // Matrix
                let rows = self.shape[0];
                let cols = self.shape[1];
                write!(f, "[")?;
                for i in 0..rows {
                    if i > 0 {
                        write!(f, "\n ")?;
                    }
                    write!(f, "[")?;
                    for j in 0..cols {
                        if j > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", self.data[i * cols + j])?;
                    }
                    write!(f, "]")?;
                }
                write!(f, "]")
            }
            _ => {
                // Higher-order tensor (3D+)
                format_nd_complex_tensor(f, &self.data, &self.shape, 0, 0)
            }
        }
    }
}

/// Recursively format N-dimensional complex tensors
fn format_nd_complex_tensor(
    f: &mut fmt::Formatter<'_>,
    data: &[Complex],
    shape: &[usize],
    depth: usize,
    offset: usize,
) -> fmt::Result {
    if shape.is_empty() {
        return write!(f, "{}", data[offset]);
    }

    if shape.len() == 1 {
        // Last dimension - print as vector
        write!(f, "[")?;
        for i in 0..shape[0] {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", data[offset + i])?;
        }
        write!(f, "]")
    } else {
        // Multiple dimensions remaining
        let current_dim = shape[0];
        let stride: usize = shape[1..].iter().product();

        write!(f, "[")?;
        for i in 0..current_dim {
            if i > 0 {
                write!(f, ",")?;
                // Add newline and indentation for readability
                write!(f, "\n")?;
                for _ in 0..=depth {
                    write!(f, " ")?;
                }
            }
            format_nd_complex_tensor(f, data, &shape[1..], depth + 1, offset + i * stride)?;
        }
        write!(f, "]")
    }
}

// ============================================================================
// Broadcasting Support
// ============================================================================

impl<T: Clone> Tensor<T> {
    /// Check if two shapes are broadcast-compatible
    /// Following NumPy broadcasting rules: dimensions are compatible when:
    /// 1. They are equal, or
    /// 2. One of them is 1
    pub fn can_broadcast(shape1: &[usize], shape2: &[usize]) -> bool {
        let max_len = shape1.len().max(shape2.len());

        for i in 0..max_len {
            let dim1 = if i < shape1.len() {
                shape1[shape1.len() - 1 - i]
            } else {
                1
            };

            let dim2 = if i < shape2.len() {
                shape2[shape2.len() - 1 - i]
            } else {
                1
            };

            if dim1 != dim2 && dim1 != 1 && dim2 != 1 {
                return false;
            }
        }

        true
    }

    /// Compute the broadcasted shape of two tensors
    pub fn broadcast_shape(shape1: &[usize], shape2: &[usize]) -> Result<Vec<usize>, TensorError> {
        if !Self::can_broadcast(shape1, shape2) {
            return Err(TensorError::BroadcastError {
                shape1: shape1.to_vec(),
                shape2: shape2.to_vec(),
            });
        }

        let max_len = shape1.len().max(shape2.len());
        let mut result = Vec::with_capacity(max_len);

        for i in 0..max_len {
            let dim1 = if i < shape1.len() {
                shape1[shape1.len() - 1 - i]
            } else {
                1
            };

            let dim2 = if i < shape2.len() {
                shape2[shape2.len() - 1 - i]
            } else {
                1
            };

            result.push(dim1.max(dim2));
        }

        result.reverse();
        Ok(result)
    }

    /// Convert a flat index to multi-dimensional indices given a shape
    fn unravel_index(flat_idx: usize, shape: &[usize], strides: &[usize]) -> Vec<usize> {
        let mut indices = Vec::with_capacity(shape.len());
        let mut remaining = flat_idx;

        for &stride in strides.iter() {
            indices.push(remaining / stride);
            remaining %= stride;
        }

        indices
    }

    /// Adjust multi-dimensional indices for broadcasting
    /// Maps indices from result shape to indices in original shape
    fn broadcast_index(result_indices: &[usize], original_shape: &[usize], result_shape: &[usize]) -> Vec<usize> {
        let rank_diff = result_shape.len() - original_shape.len();
        let mut adjusted = Vec::with_capacity(original_shape.len());

        for i in 0..original_shape.len() {
            let result_idx = result_indices[i + rank_diff];
            // If the original dimension is 1, always use index 0 (broadcasting)
            adjusted.push(if original_shape[i] == 1 { 0 } else { result_idx });
        }

        adjusted
    }
}

// ============================================================================
// Arithmetic Operations for RealTensor
// ============================================================================

impl RealTensor {
    /// Element-wise addition with broadcasting
    pub fn add(&self, other: &RealTensor) -> Result<RealTensor, TensorError> {
        // Fast path: if shapes are identical, use direct element-wise operation
        if self.shape == other.shape {
            let data: Vec<f64> = self.data.iter()
                .zip(other.data.iter())
                .map(|(a, b)| a + b)
                .collect();
            return RealTensor::new(data, self.shape.clone());
        }

        // Broadcasting path
        let result_shape = Tensor::<f64>::broadcast_shape(&self.shape, &other.shape)?;
        let result_strides = Tensor::<f64>::compute_strides(&result_shape);
        let result_size: usize = result_shape.iter().product();

        let mut data = Vec::with_capacity(result_size);

        for flat_idx in 0..result_size {
            let result_indices = Tensor::<f64>::unravel_index(flat_idx, &result_shape, &result_strides);
            let self_indices = Tensor::<f64>::broadcast_index(&result_indices, &self.shape, &result_shape);
            let other_indices = Tensor::<f64>::broadcast_index(&result_indices, &other.shape, &result_shape);

            let a = self.get(&self_indices)?;
            let b = other.get(&other_indices)?;
            data.push(a + b);
        }

        RealTensor::new(data, result_shape)
    }

    /// Element-wise subtraction with broadcasting
    pub fn sub(&self, other: &RealTensor) -> Result<RealTensor, TensorError> {
        // Fast path: if shapes are identical, use direct element-wise operation
        if self.shape == other.shape {
            let data: Vec<f64> = self.data.iter()
                .zip(other.data.iter())
                .map(|(a, b)| a - b)
                .collect();
            return RealTensor::new(data, self.shape.clone());
        }

        // Broadcasting path
        let result_shape = Tensor::<f64>::broadcast_shape(&self.shape, &other.shape)?;
        let result_strides = Tensor::<f64>::compute_strides(&result_shape);
        let result_size: usize = result_shape.iter().product();

        let mut data = Vec::with_capacity(result_size);

        for flat_idx in 0..result_size {
            let result_indices = Tensor::<f64>::unravel_index(flat_idx, &result_shape, &result_strides);
            let self_indices = Tensor::<f64>::broadcast_index(&result_indices, &self.shape, &result_shape);
            let other_indices = Tensor::<f64>::broadcast_index(&result_indices, &other.shape, &result_shape);

            let a = self.get(&self_indices)?;
            let b = other.get(&other_indices)?;
            data.push(a - b);
        }

        RealTensor::new(data, result_shape)
    }

    /// Element-wise multiplication (Hadamard product) with broadcasting
    pub fn mul(&self, other: &RealTensor) -> Result<RealTensor, TensorError> {
        // Fast path: if shapes are identical, use direct element-wise operation
        if self.shape == other.shape {
            let data: Vec<f64> = self.data.iter()
                .zip(other.data.iter())
                .map(|(a, b)| a * b)
                .collect();
            return RealTensor::new(data, self.shape.clone());
        }

        // Broadcasting path
        let result_shape = Tensor::<f64>::broadcast_shape(&self.shape, &other.shape)?;
        let result_strides = Tensor::<f64>::compute_strides(&result_shape);
        let result_size: usize = result_shape.iter().product();

        let mut data = Vec::with_capacity(result_size);

        for flat_idx in 0..result_size {
            let result_indices = Tensor::<f64>::unravel_index(flat_idx, &result_shape, &result_strides);
            let self_indices = Tensor::<f64>::broadcast_index(&result_indices, &self.shape, &result_shape);
            let other_indices = Tensor::<f64>::broadcast_index(&result_indices, &other.shape, &result_shape);

            let a = self.get(&self_indices)?;
            let b = other.get(&other_indices)?;
            data.push(a * b);
        }

        RealTensor::new(data, result_shape)
    }

    /// Element-wise division with broadcasting
    pub fn div(&self, other: &RealTensor) -> Result<RealTensor, TensorError> {
        // Fast path: if shapes are identical, use direct element-wise operation
        if self.shape == other.shape {
            let data: Vec<f64> = self.data.iter()
                .zip(other.data.iter())
                .map(|(a, b)| a / b)
                .collect();
            return RealTensor::new(data, self.shape.clone());
        }

        // Broadcasting path
        let result_shape = Tensor::<f64>::broadcast_shape(&self.shape, &other.shape)?;
        let result_strides = Tensor::<f64>::compute_strides(&result_shape);
        let result_size: usize = result_shape.iter().product();

        let mut data = Vec::with_capacity(result_size);

        for flat_idx in 0..result_size {
            let result_indices = Tensor::<f64>::unravel_index(flat_idx, &result_shape, &result_strides);
            let self_indices = Tensor::<f64>::broadcast_index(&result_indices, &self.shape, &result_shape);
            let other_indices = Tensor::<f64>::broadcast_index(&result_indices, &other.shape, &result_shape);

            let a = self.get(&self_indices)?;
            let b = other.get(&other_indices)?;
            data.push(a / b);
        }

        RealTensor::new(data, result_shape)
    }

    /// Scalar addition
    pub fn add_scalar(&self, scalar: f64) -> RealTensor {
        let data: Vec<f64> = self.data.iter().map(|x| x + scalar).collect();
        RealTensor::new(data, self.shape.clone()).unwrap()
    }

    /// Scalar subtraction (tensor - scalar)
    pub fn sub_scalar(&self, scalar: f64) -> RealTensor {
        let data: Vec<f64> = self.data.iter().map(|x| x - scalar).collect();
        RealTensor::new(data, self.shape.clone()).unwrap()
    }

    /// Scalar multiplication
    pub fn mul_scalar(&self, scalar: f64) -> RealTensor {
        let data: Vec<f64> = self.data.iter().map(|x| x * scalar).collect();
        RealTensor::new(data, self.shape.clone()).unwrap()
    }

    /// Scalar division (tensor / scalar)
    pub fn div_scalar(&self, scalar: f64) -> Result<RealTensor, String> {
        if scalar == 0.0 {
            return Err("Division by zero".to_string());
        }
        let data: Vec<f64> = self.data.iter().map(|x| x / scalar).collect();
        Ok(RealTensor::new(data, self.shape.clone()).unwrap())
    }

    /// Negate all elements
    pub fn negate(&self) -> RealTensor {
        let data: Vec<f64> = self.data.iter().map(|x| -x).collect();
        RealTensor::new(data, self.shape.clone()).unwrap()
    }
}

// ============================================================================
// Arithmetic Operations for ComplexTensor
// ============================================================================

impl ComplexTensor {
    /// Element-wise addition with broadcasting
    pub fn add(&self, other: &ComplexTensor) -> Result<ComplexTensor, TensorError> {
        // Fast path: if shapes are identical, use direct element-wise operation
        if self.shape == other.shape {
            let data: Vec<Complex> = self.data.iter()
                .zip(other.data.iter())
                .map(|(a, b)| *a + *b)
                .collect();
            return ComplexTensor::new(data, self.shape.clone());
        }

        // Broadcasting path
        let result_shape = Tensor::<Complex>::broadcast_shape(&self.shape, &other.shape)?;
        let result_strides = Tensor::<Complex>::compute_strides(&result_shape);
        let result_size: usize = result_shape.iter().product();

        let mut data = Vec::with_capacity(result_size);

        for flat_idx in 0..result_size {
            let result_indices = Tensor::<Complex>::unravel_index(flat_idx, &result_shape, &result_strides);
            let self_indices = Tensor::<Complex>::broadcast_index(&result_indices, &self.shape, &result_shape);
            let other_indices = Tensor::<Complex>::broadcast_index(&result_indices, &other.shape, &result_shape);

            let a = self.get(&self_indices)?;
            let b = other.get(&other_indices)?;
            data.push(*a + *b);
        }

        ComplexTensor::new(data, result_shape)
    }

    /// Element-wise subtraction with broadcasting
    pub fn sub(&self, other: &ComplexTensor) -> Result<ComplexTensor, TensorError> {
        // Fast path: if shapes are identical, use direct element-wise operation
        if self.shape == other.shape {
            let data: Vec<Complex> = self.data.iter()
                .zip(other.data.iter())
                .map(|(a, b)| *a - *b)
                .collect();
            return ComplexTensor::new(data, self.shape.clone());
        }

        // Broadcasting path
        let result_shape = Tensor::<Complex>::broadcast_shape(&self.shape, &other.shape)?;
        let result_strides = Tensor::<Complex>::compute_strides(&result_shape);
        let result_size: usize = result_shape.iter().product();

        let mut data = Vec::with_capacity(result_size);

        for flat_idx in 0..result_size {
            let result_indices = Tensor::<Complex>::unravel_index(flat_idx, &result_shape, &result_strides);
            let self_indices = Tensor::<Complex>::broadcast_index(&result_indices, &self.shape, &result_shape);
            let other_indices = Tensor::<Complex>::broadcast_index(&result_indices, &other.shape, &result_shape);

            let a = self.get(&self_indices)?;
            let b = other.get(&other_indices)?;
            data.push(*a - *b);
        }

        ComplexTensor::new(data, result_shape)
    }

    /// Element-wise multiplication with broadcasting
    pub fn mul(&self, other: &ComplexTensor) -> Result<ComplexTensor, TensorError> {
        // Fast path: if shapes are identical, use direct element-wise operation
        if self.shape == other.shape {
            let data: Vec<Complex> = self.data.iter()
                .zip(other.data.iter())
                .map(|(a, b)| *a * *b)
                .collect();
            return ComplexTensor::new(data, self.shape.clone());
        }

        // Broadcasting path
        let result_shape = Tensor::<Complex>::broadcast_shape(&self.shape, &other.shape)?;
        let result_strides = Tensor::<Complex>::compute_strides(&result_shape);
        let result_size: usize = result_shape.iter().product();

        let mut data = Vec::with_capacity(result_size);

        for flat_idx in 0..result_size {
            let result_indices = Tensor::<Complex>::unravel_index(flat_idx, &result_shape, &result_strides);
            let self_indices = Tensor::<Complex>::broadcast_index(&result_indices, &self.shape, &result_shape);
            let other_indices = Tensor::<Complex>::broadcast_index(&result_indices, &other.shape, &result_shape);

            let a = self.get(&self_indices)?;
            let b = other.get(&other_indices)?;
            data.push(*a * *b);
        }

        ComplexTensor::new(data, result_shape)
    }

    /// Element-wise division with broadcasting
    pub fn div(&self, other: &ComplexTensor) -> Result<ComplexTensor, TensorError> {
        // Fast path: if shapes are identical, use direct element-wise operation
        if self.shape == other.shape {
            let data: Vec<Complex> = self.data.iter()
                .zip(other.data.iter())
                .map(|(a, b)| *a / *b)
                .collect();
            return ComplexTensor::new(data, self.shape.clone());
        }

        // Broadcasting path
        let result_shape = Tensor::<Complex>::broadcast_shape(&self.shape, &other.shape)?;
        let result_strides = Tensor::<Complex>::compute_strides(&result_shape);
        let result_size: usize = result_shape.iter().product();

        let mut data = Vec::with_capacity(result_size);

        for flat_idx in 0..result_size {
            let result_indices = Tensor::<Complex>::unravel_index(flat_idx, &result_shape, &result_strides);
            let self_indices = Tensor::<Complex>::broadcast_index(&result_indices, &self.shape, &result_shape);
            let other_indices = Tensor::<Complex>::broadcast_index(&result_indices, &other.shape, &result_shape);

            let a = self.get(&self_indices)?;
            let b = other.get(&other_indices)?;
            data.push(*a / *b);
        }

        ComplexTensor::new(data, result_shape)
    }

    /// Scalar addition
    pub fn add_scalar(&self, scalar: Complex) -> ComplexTensor {
        let data: Vec<Complex> = self.data.iter().map(|x| *x + scalar).collect();
        ComplexTensor::new(data, self.shape.clone()).unwrap()
    }

    /// Scalar subtraction (tensor - scalar)
    pub fn sub_scalar(&self, scalar: Complex) -> ComplexTensor {
        let data: Vec<Complex> = self.data.iter().map(|x| *x - scalar).collect();
        ComplexTensor::new(data, self.shape.clone()).unwrap()
    }

    /// Scalar multiplication
    pub fn mul_scalar(&self, scalar: Complex) -> ComplexTensor {
        let data: Vec<Complex> = self.data.iter().map(|x| *x * scalar).collect();
        ComplexTensor::new(data, self.shape.clone()).unwrap()
    }

    /// Scalar division (tensor / scalar)
    pub fn div_scalar(&self, scalar: Complex) -> Result<ComplexTensor, String> {
        if scalar.re == 0.0 && scalar.im == 0.0 {
            return Err("Division by zero".to_string());
        }
        let data: Vec<Complex> = self.data.iter().map(|x| *x / scalar).collect();
        Ok(ComplexTensor::new(data, self.shape.clone()).unwrap())
    }

    /// Negate all elements
    pub fn negate(&self) -> ComplexTensor {
        let data: Vec<Complex> = self.data.iter().map(|x| -*x).collect();
        ComplexTensor::new(data, self.shape.clone()).unwrap()
    }

    /// Convert to RealTensor (magnitude)
    pub fn abs(&self) -> RealTensor {
        let data: Vec<f64> = self.data.iter().map(|c| c.magnitude()).collect();
        RealTensor::new(data, self.shape.clone()).unwrap()
    }
}

// ============================================================================
// Vector Operations (rank 1 tensors)
// ============================================================================

impl RealTensor {
    /// Dot product (inner product) of two vectors
    pub fn dot(&self, other: &RealTensor) -> Result<f64, TensorError> {
        if !self.is_vector() || !other.is_vector() {
            return Err(TensorError::DimensionMismatch {
                expected: vec![self.size()],
                got: other.shape.clone(),
            });
        }

        if self.size() != other.size() {
            return Err(TensorError::DimensionMismatch {
                expected: self.shape.clone(),
                got: other.shape.clone(),
            });
        }

        Ok(self.data.iter().zip(other.data.iter()).map(|(a, b)| a * b).sum())
    }

    /// Cross product of two 3D vectors
    pub fn cross(&self, other: &RealTensor) -> Result<RealTensor, TensorError> {
        if !self.is_vector() || !other.is_vector() {
            return Err(TensorError::DimensionMismatch {
                expected: vec![3],
                got: vec![],
            });
        }

        if self.size() != 3 || other.size() != 3 {
            return Err(TensorError::DimensionMismatch {
                expected: vec![3],
                got: self.shape.clone(),
            });
        }

        let a = &self.data;
        let b = &other.data;

        let data = vec![
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ];

        Ok(RealTensor::vector(data))
    }

    /// Euclidean norm (L2 norm)
    pub fn norm(&self) -> f64 {
        self.data.iter().map(|x| x * x).sum::<f64>().sqrt()
    }

    /// L1 norm (Manhattan distance)
    pub fn norm_l1(&self) -> f64 {
        self.data.iter().map(|x| x.abs()).sum()
    }

    /// Normalize vector to unit length
    pub fn normalize(&self) -> Result<RealTensor, TensorError> {
        let n = self.norm();
        if n < 1e-10 {
            return Err(TensorError::EmptyTensor);
        }
        Ok(self.mul_scalar(1.0 / n))
    }

    /// Sum of all elements
    pub fn sum(&self) -> f64 {
        self.data.iter().sum()
    }

    /// Mean of all elements
    pub fn mean(&self) -> Result<f64, TensorError> {
        if self.is_empty() {
            return Err(TensorError::EmptyTensor);
        }
        Ok(self.sum() / self.size() as f64)
    }

    /// Maximum element
    pub fn max(&self) -> Result<f64, TensorError> {
        self.data.iter()
            .copied()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .ok_or(TensorError::EmptyTensor)
    }

    /// Minimum element
    pub fn min(&self) -> Result<f64, TensorError> {
        self.data.iter()
            .copied()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .ok_or(TensorError::EmptyTensor)
    }

    /// Variance of all elements (sample variance with Bessel's correction)
    pub fn variance(&self) -> Result<f64, TensorError> {
        if self.size() < 2 {
            return Err(TensorError::EmptyTensor);
        }
        let mean = self.mean()?;
        let sum_squared_diff: f64 = self.data.iter()
            .map(|x| (x - mean).powi(2))
            .sum();
        Ok(sum_squared_diff / (self.size() - 1) as f64)
    }

    /// Standard deviation of all elements (sample std dev with Bessel's correction)
    pub fn std_dev(&self) -> Result<f64, TensorError> {
        Ok(self.variance()?.sqrt())
    }
}

impl ComplexTensor {
    /// Dot product of two complex vectors
    pub fn dot(&self, other: &ComplexTensor) -> Result<Complex, TensorError> {
        if !self.is_vector() || !other.is_vector() {
            return Err(TensorError::DimensionMismatch {
                expected: vec![self.size()],
                got: other.shape.clone(),
            });
        }

        if self.size() != other.size() {
            return Err(TensorError::DimensionMismatch {
                expected: self.shape.clone(),
                got: other.shape.clone(),
            });
        }

        Ok(self.data.iter()
            .zip(other.data.iter())
            .map(|(a, b)| a.conjugate() * *b)  // Hermitian inner product
            .fold(Complex::new(0.0, 0.0), |acc, x| acc + x))
    }

    /// Norm of complex vector
    pub fn norm(&self) -> f64 {
        self.data.iter().map(|c| c.magnitude() * c.magnitude()).sum::<f64>().sqrt()
    }

    /// Normalize complex vector
    pub fn normalize(&self) -> Result<ComplexTensor, TensorError> {
        let n = self.norm();
        if n < 1e-10 {
            return Err(TensorError::EmptyTensor);
        }
        Ok(self.mul_scalar(Complex::new(1.0 / n, 0.0)))
    }

    /// Sum of all complex elements
    pub fn sum(&self) -> Complex {
        self.data.iter().fold(Complex::new(0.0, 0.0), |acc, &x| acc + x)
    }

    /// Mean of all complex elements
    pub fn mean(&self) -> Result<Complex, TensorError> {
        if self.is_empty() {
            return Err(TensorError::EmptyTensor);
        }
        let sum = self.sum();
        let n = self.size() as f64;
        Ok(Complex::new(sum.re / n, sum.im / n))
    }

    /// Standard deviation of complex tensor (magnitude-based)
    pub fn std_dev(&self) -> Result<f64, TensorError> {
        if self.size() < 2 {
            return Err(TensorError::EmptyTensor);
        }
        let mean = self.mean()?;
        let sum_squared_diff: f64 = self.data.iter()
            .map(|x| (*x - mean).magnitude().powi(2))
            .sum();
        Ok((sum_squared_diff / (self.size() - 1) as f64).sqrt())
    }
}

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

// ============================================================================
// Type Conversion
// ============================================================================

impl RealTensor {
    /// Convert real tensor to complex tensor
    pub fn to_complex(&self) -> ComplexTensor {
        let data: Vec<Complex> = self.data.iter()
            .map(|&x| Complex::new(x, 0.0))
            .collect();
        ComplexTensor::new(data, self.shape.clone()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_creation() {
        let t = RealTensor::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]).unwrap();
        assert_eq!(t.rank(), 2);
        assert_eq!(t.size(), 4);
        assert_eq!(t.shape(), &[2, 2]);
    }

    #[test]
    fn test_scalar() {
        let s = RealTensor::scalar(5.0);
        assert_eq!(s.rank(), 0);
        assert_eq!(s.size(), 1);
        assert!(s.is_scalar());
    }

    #[test]
    fn test_vector() {
        let v = RealTensor::vector(vec![1.0, 2.0, 3.0]);
        assert_eq!(v.rank(), 1);
        assert_eq!(v.size(), 3);
        assert!(v.is_vector());
    }

    #[test]
    fn test_matrix() {
        let m = RealTensor::matrix(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        assert_eq!(m.rank(), 2);
        assert_eq!(m.shape(), &[2, 3]);
        assert!(m.is_matrix());
    }

    #[test]
    fn test_get_set() {
        let mut t = RealTensor::matrix(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        assert_eq!(*t.get(&[0, 0]).unwrap(), 1.0);
        assert_eq!(*t.get(&[1, 1]).unwrap(), 4.0);

        t.set(&[0, 1], 10.0).unwrap();
        assert_eq!(*t.get(&[0, 1]).unwrap(), 10.0);
    }

    #[test]
    fn test_reshape() {
        let t = RealTensor::vector(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let reshaped = t.reshape(vec![2, 3]).unwrap();
        assert_eq!(reshaped.shape(), &[2, 3]);
        assert_eq!(reshaped.rank(), 2);
    }

    #[test]
    fn test_zeros_ones() {
        let z = RealTensor::zeros(vec![2, 3]);
        assert_eq!(z.size(), 6);
        assert!(z.data.iter().all(|&x| x == 0.0));

        let o = RealTensor::ones(vec![2, 3]);
        assert!(o.data.iter().all(|&x| x == 1.0));
    }

    #[test]
    fn test_eye() {
        let eye = RealTensor::eye(3);
        assert_eq!(eye.shape(), &[3, 3]);
        assert_eq!(*eye.get(&[0, 0]).unwrap(), 1.0);
        assert_eq!(*eye.get(&[1, 1]).unwrap(), 1.0);
        assert_eq!(*eye.get(&[0, 1]).unwrap(), 0.0);
    }

    #[test]
    fn test_complex_tensor() {
        let data = vec![
            Complex::new(1.0, 2.0),
            Complex::new(3.0, 4.0),
        ];
        let ct = ComplexTensor::vector(data);
        assert_eq!(ct.rank(), 1);
        assert_eq!(ct.size(), 2);
    }

    // ========================================================================
    // Broadcasting Tests
    // ========================================================================

    #[test]
    fn test_broadcast_compatibility() {
        assert!(RealTensor::can_broadcast(&[3, 4], &[3, 4]));  // Same shape
        assert!(RealTensor::can_broadcast(&[3, 1], &[3, 4]));  // One dimension is 1
        assert!(RealTensor::can_broadcast(&[3, 4], &[1, 4]));  // One dimension is 1
        assert!(RealTensor::can_broadcast(&[5, 3, 4], &[3, 4]));  // Different ranks
        assert!(!RealTensor::can_broadcast(&[3, 4], &[2, 4]));  // Incompatible
    }

    #[test]
    fn test_broadcast_shape() {
        assert_eq!(
            RealTensor::broadcast_shape(&[3, 4], &[3, 4]).unwrap(),
            vec![3, 4]
        );
        assert_eq!(
            RealTensor::broadcast_shape(&[3, 1], &[3, 4]).unwrap(),
            vec![3, 4]
        );
        assert_eq!(
            RealTensor::broadcast_shape(&[5, 3, 4], &[3, 4]).unwrap(),
            vec![5, 3, 4]
        );
    }

    // ========================================================================
    // Arithmetic Tests
    // ========================================================================

    #[test]
    fn test_tensor_addition() {
        let a = RealTensor::vector(vec![1.0, 2.0, 3.0]);
        let b = RealTensor::vector(vec![4.0, 5.0, 6.0]);
        let c = a.add(&b).unwrap();

        assert_eq!(c.data(), &[5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_tensor_subtraction() {
        let a = RealTensor::vector(vec![5.0, 7.0, 9.0]);
        let b = RealTensor::vector(vec![1.0, 2.0, 3.0]);
        let c = a.sub(&b).unwrap();

        assert_eq!(c.data(), &[4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_tensor_multiplication() {
        let a = RealTensor::vector(vec![2.0, 3.0, 4.0]);
        let b = RealTensor::vector(vec![5.0, 6.0, 7.0]);
        let c = a.mul(&b).unwrap();

        assert_eq!(c.data(), &[10.0, 18.0, 28.0]);
    }

    #[test]
    fn test_tensor_division() {
        let a = RealTensor::vector(vec![10.0, 20.0, 30.0]);
        let b = RealTensor::vector(vec![2.0, 4.0, 5.0]);
        let c = a.div(&b).unwrap();

        assert_eq!(c.data(), &[5.0, 5.0, 6.0]);
    }

    #[test]
    fn test_scalar_operations() {
        let t = RealTensor::vector(vec![1.0, 2.0, 3.0]);

        let add_result = t.add_scalar(10.0);
        assert_eq!(add_result.data(), &[11.0, 12.0, 13.0]);

        let mul_result = t.mul_scalar(2.0);
        assert_eq!(mul_result.data(), &[2.0, 4.0, 6.0]);

        let neg_result = t.negate();
        assert_eq!(neg_result.data(), &[-1.0, -2.0, -3.0]);
    }

    #[test]
    fn test_complex_arithmetic() {
        let a = ComplexTensor::vector(vec![
            Complex::new(1.0, 2.0),
            Complex::new(3.0, 4.0),
        ]);
        let b = ComplexTensor::vector(vec![
            Complex::new(5.0, 6.0),
            Complex::new(7.0, 8.0),
        ]);

        let c = a.add(&b).unwrap();
        assert_eq!(c.data[0], Complex::new(6.0, 8.0));
        assert_eq!(c.data[1], Complex::new(10.0, 12.0));
    }

    // ========================================================================
    // Vector Operation Tests
    // ========================================================================

    #[test]
    fn test_dot_product() {
        let a = RealTensor::vector(vec![1.0, 2.0, 3.0]);
        let b = RealTensor::vector(vec![4.0, 5.0, 6.0]);

        let result = a.dot(&b).unwrap();
        assert_eq!(result, 32.0);  // 1*4 + 2*5 + 3*6 = 32
    }

    #[test]
    fn test_cross_product() {
        let a = RealTensor::vector(vec![1.0, 0.0, 0.0]);
        let b = RealTensor::vector(vec![0.0, 1.0, 0.0]);

        let c = a.cross(&b).unwrap();
        assert_eq!(c.data(), &[0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_norm() {
        let v = RealTensor::vector(vec![3.0, 4.0]);
        assert_eq!(v.norm(), 5.0);  // 3-4-5 triangle

        let v2 = RealTensor::vector(vec![1.0, 2.0, 2.0]);
        assert_eq!(v2.norm(), 3.0);
    }

    #[test]
    fn test_normalize() {
        let v = RealTensor::vector(vec![3.0, 4.0]);
        let normalized = v.normalize().unwrap();

        assert!((normalized.data()[0] - 0.6).abs() < 1e-10);
        assert!((normalized.data()[1] - 0.8).abs() < 1e-10);
        assert!((normalized.norm() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_statistics() {
        let t = RealTensor::vector(vec![1.0, 2.0, 3.0, 4.0, 5.0]);

        assert_eq!(t.sum(), 15.0);
        assert_eq!(t.mean().unwrap(), 3.0);
        assert_eq!(t.max().unwrap(), 5.0);
        assert_eq!(t.min().unwrap(), 1.0);
    }

    #[test]
    fn test_complex_dot() {
        let a = ComplexTensor::vector(vec![
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 1.0),
        ]);
        let b = ComplexTensor::vector(vec![
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 1.0),
        ]);

        let result = a.dot(&b).unwrap();
        // Hermitian: conj(a[0])*b[0] + conj(a[1])*b[1]
        // = (1+0i)*conj * (1+0i) + (0+1i)*conj * (0+1i)
        // = 1 + (0-1i)*(0+1i) = 1 + 1 = 2
        assert_eq!(result, Complex::new(2.0, 0.0));
    }

    #[test]
    fn test_complex_norm() {
        let v = ComplexTensor::vector(vec![
            Complex::new(3.0, 0.0),
            Complex::new(0.0, 4.0),
        ]);
        assert_eq!(v.norm(), 5.0);
    }

    // ========================================================================
    // Matrix Operation Tests
    // ========================================================================

    #[test]
    fn test_transpose() {
        let m = RealTensor::matrix(2, 3, vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
        ]).unwrap();

        let mt = m.transpose().unwrap();
        assert_eq!(mt.shape(), &[3, 2]);
        assert_eq!(*mt.get(&[0, 0]).unwrap(), 1.0);
        assert_eq!(*mt.get(&[0, 1]).unwrap(), 4.0);
        assert_eq!(*mt.get(&[1, 0]).unwrap(), 2.0);
        assert_eq!(*mt.get(&[1, 1]).unwrap(), 5.0);
    }

    #[test]
    fn test_trace() {
        let m = RealTensor::matrix(3, 3, vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0,
        ]).unwrap();

        let tr = m.trace().unwrap();
        assert_eq!(tr, 15.0);  // 1 + 5 + 9
    }

    #[test]
    fn test_matrix_multiplication() {
        let a = RealTensor::matrix(2, 3, vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
        ]).unwrap();

        let b = RealTensor::matrix(3, 2, vec![
            7.0, 8.0,
            9.0, 10.0,
            11.0, 12.0,
        ]).unwrap();

        let c = a.matmul(&b).unwrap();
        assert_eq!(c.shape(), &[2, 2]);

        // Result should be:
        // [1*7 + 2*9 + 3*11,  1*8 + 2*10 + 3*12]   = [58, 64]
        // [4*7 + 5*9 + 6*11,  4*8 + 5*10 + 6*12]   = [139, 154]
        assert_eq!(*c.get(&[0, 0]).unwrap(), 58.0);
        assert_eq!(*c.get(&[0, 1]).unwrap(), 64.0);
        assert_eq!(*c.get(&[1, 0]).unwrap(), 139.0);
        assert_eq!(*c.get(&[1, 1]).unwrap(), 154.0);
    }

    #[test]
    fn test_complex_hermitian() {
        let m = ComplexTensor::zeros(vec![2, 2]);
        let h = m.hermitian().unwrap();
        assert_eq!(h.shape(), &[2, 2]);
    }

    #[test]
    fn test_complex_trace() {
        let m = ComplexTensor::eye(3);
        let tr = m.trace().unwrap();
        assert_eq!(tr, Complex::new(3.0, 0.0));
    }

    // ========================================================================
    // Type Conversion Tests
    // ========================================================================

    #[test]
    fn test_real_to_complex() {
        let r = RealTensor::vector(vec![1.0, 2.0, 3.0]);
        let c = r.to_complex();

        assert_eq!(c.size(), 3);
        assert_eq!(c.data[0], Complex::new(1.0, 0.0));
        assert_eq!(c.data[1], Complex::new(2.0, 0.0));
    }

    #[test]
    fn test_complex_abs() {
        let c = ComplexTensor::vector(vec![
            Complex::new(3.0, 4.0),
            Complex::new(5.0, 12.0),
        ]);

        let abs_tensor = c.abs();
        assert_eq!(abs_tensor.data()[0], 5.0);   // sqrt(3^2 + 4^2)
        assert_eq!(abs_tensor.data()[1], 13.0);  // sqrt(5^2 + 12^2)
    }

    // ========================================================================
    // N-Dimensional Broadcasting Tests
    // ========================================================================

    #[test]
    fn test_broadcast_vector_to_matrix() {
        // Matrix [2, 3] + Vector [3] → broadcast to [2, 3]
        let m = RealTensor::matrix(2, 3, vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
        ]).unwrap();

        let v = RealTensor::vector(vec![10.0, 20.0, 30.0]);

        let result = m.add(&v).unwrap();

        assert_eq!(result.shape(), &[2, 3]);
        // First row: [1+10, 2+20, 3+30] = [11, 22, 33]
        assert_eq!(*result.get(&[0, 0]).unwrap(), 11.0);
        assert_eq!(*result.get(&[0, 1]).unwrap(), 22.0);
        assert_eq!(*result.get(&[0, 2]).unwrap(), 33.0);
        // Second row: [4+10, 5+20, 6+30] = [14, 25, 36]
        assert_eq!(*result.get(&[1, 0]).unwrap(), 14.0);
        assert_eq!(*result.get(&[1, 1]).unwrap(), 25.0);
        assert_eq!(*result.get(&[1, 2]).unwrap(), 36.0);
    }

    #[test]
    fn test_broadcast_column_to_matrix() {
        // Matrix [2, 3] + Column [2, 1] → broadcast to [2, 3]
        let m = RealTensor::matrix(2, 3, vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
        ]).unwrap();

        let col = RealTensor::matrix(2, 1, vec![100.0, 200.0]).unwrap();

        let result = m.add(&col).unwrap();

        assert_eq!(result.shape(), &[2, 3]);
        // First row: [1+100, 2+100, 3+100] = [101, 102, 103]
        assert_eq!(*result.get(&[0, 0]).unwrap(), 101.0);
        assert_eq!(*result.get(&[0, 1]).unwrap(), 102.0);
        assert_eq!(*result.get(&[0, 2]).unwrap(), 103.0);
        // Second row: [4+200, 5+200, 6+200] = [204, 205, 206]
        assert_eq!(*result.get(&[1, 0]).unwrap(), 204.0);
        assert_eq!(*result.get(&[1, 1]).unwrap(), 205.0);
        assert_eq!(*result.get(&[1, 2]).unwrap(), 206.0);
    }

    #[test]
    fn test_broadcast_3d_tensor() {
        // Tensor [2, 2, 1] + Tensor [2, 1, 3] → broadcast to [2, 2, 3]
        let t1 = RealTensor::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2, 1]).unwrap();
        let t2 = RealTensor::new(vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0], vec![2, 1, 3]).unwrap();

        let result = t1.add(&t2).unwrap();

        assert_eq!(result.shape(), &[2, 2, 3]);
        assert_eq!(result.size(), 12);

        // Check a few key elements
        assert_eq!(*result.get(&[0, 0, 0]).unwrap(), 11.0); // 1 + 10
        assert_eq!(*result.get(&[0, 0, 1]).unwrap(), 21.0); // 1 + 20
        assert_eq!(*result.get(&[0, 0, 2]).unwrap(), 31.0); // 1 + 30
    }

    #[test]
    fn test_broadcast_subtraction() {
        // Test broadcasting with subtraction
        let m = RealTensor::matrix(3, 2, vec![
            10.0, 20.0,
            30.0, 40.0,
            50.0, 60.0,
        ]).unwrap();

        let v = RealTensor::vector(vec![1.0, 2.0]);

        let result = m.sub(&v).unwrap();

        assert_eq!(result.shape(), &[3, 2]);
        assert_eq!(*result.get(&[0, 0]).unwrap(), 9.0);   // 10 - 1
        assert_eq!(*result.get(&[0, 1]).unwrap(), 18.0);  // 20 - 2
        assert_eq!(*result.get(&[1, 0]).unwrap(), 29.0);  // 30 - 1
        assert_eq!(*result.get(&[1, 1]).unwrap(), 38.0);  // 40 - 2
    }

    #[test]
    fn test_broadcast_multiplication() {
        // Test broadcasting with multiplication
        let m = RealTensor::matrix(2, 3, vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
        ]).unwrap();

        let v = RealTensor::vector(vec![10.0, 100.0, 1000.0]);

        let result = m.mul(&v).unwrap();

        assert_eq!(result.shape(), &[2, 3]);
        assert_eq!(*result.get(&[0, 0]).unwrap(), 10.0);    // 1 * 10
        assert_eq!(*result.get(&[0, 1]).unwrap(), 200.0);   // 2 * 100
        assert_eq!(*result.get(&[0, 2]).unwrap(), 3000.0);  // 3 * 1000
        assert_eq!(*result.get(&[1, 0]).unwrap(), 40.0);    // 4 * 10
        assert_eq!(*result.get(&[1, 1]).unwrap(), 500.0);   // 5 * 100
        assert_eq!(*result.get(&[1, 2]).unwrap(), 6000.0);  // 6 * 1000
    }

    #[test]
    fn test_broadcast_division() {
        // Test broadcasting with division
        let m = RealTensor::matrix(2, 2, vec![
            100.0, 200.0,
            300.0, 400.0,
        ]).unwrap();

        let v = RealTensor::vector(vec![10.0, 20.0]);

        let result = m.div(&v).unwrap();

        assert_eq!(result.shape(), &[2, 2]);
        assert_eq!(*result.get(&[0, 0]).unwrap(), 10.0);  // 100 / 10
        assert_eq!(*result.get(&[0, 1]).unwrap(), 10.0);  // 200 / 20
        assert_eq!(*result.get(&[1, 0]).unwrap(), 30.0);  // 300 / 10
        assert_eq!(*result.get(&[1, 1]).unwrap(), 20.0);  // 400 / 20
    }

    #[test]
    fn test_broadcast_complex_tensors() {
        // Test broadcasting with complex tensors
        let m = ComplexTensor::new(vec![
            Complex::new(1.0, 1.0),
            Complex::new(2.0, 2.0),
            Complex::new(3.0, 3.0),
            Complex::new(4.0, 4.0),
        ], vec![2, 2]).unwrap();

        let v = ComplexTensor::vector(vec![
            Complex::new(10.0, 0.0),
            Complex::new(20.0, 0.0),
        ]);

        let result = m.add(&v).unwrap();

        assert_eq!(result.shape(), &[2, 2]);
        assert_eq!(*result.get(&[0, 0]).unwrap(), Complex::new(11.0, 1.0));  // (1+1i) + 10
        assert_eq!(*result.get(&[0, 1]).unwrap(), Complex::new(22.0, 2.0));  // (2+2i) + 20
    }

    #[test]
    fn test_broadcast_incompatible_shapes() {
        // Test that incompatible shapes fail properly
        let m1 = RealTensor::matrix(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        let m2 = RealTensor::matrix(2, 4, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]).unwrap();

        let result = m1.add(&m2);
        assert!(result.is_err());
    }

    #[test]
    fn test_broadcast_higher_rank() {
        // Test broadcasting from lower to higher rank
        // [3, 4, 5] + [5] → should broadcast to [3, 4, 5]
        let t1 = RealTensor::ones(vec![3, 4, 5]);
        let t2 = RealTensor::vector(vec![1.0, 2.0, 3.0, 4.0, 5.0]);

        let result = t1.add(&t2).unwrap();

        assert_eq!(result.shape(), &[3, 4, 5]);

        // Every element in the last dimension should be offset by the vector
        assert_eq!(*result.get(&[0, 0, 0]).unwrap(), 2.0);  // 1 + 1
        assert_eq!(*result.get(&[0, 0, 1]).unwrap(), 3.0);  // 1 + 2
        assert_eq!(*result.get(&[0, 0, 2]).unwrap(), 4.0);  // 1 + 3
        assert_eq!(*result.get(&[0, 0, 3]).unwrap(), 5.0);  // 1 + 4
        assert_eq!(*result.get(&[0, 0, 4]).unwrap(), 6.0);  // 1 + 5
    }
}
