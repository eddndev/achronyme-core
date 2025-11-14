use crate::complex::Complex;
use crate::tensor::core::ComplexTensor;

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
