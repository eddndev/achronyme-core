use achronyme_types::matrix::Matrix;
use achronyme_types::complex::Complex;
use faer::prelude::*;
use faer::complex_native::c64;

/// Convert Achronyme Matrix to faer Mat
fn matrix_to_faer(matrix: &Matrix) -> Mat<f64> {
    let rows = matrix.rows;
    let cols = matrix.cols;
    Mat::from_fn(rows, cols, |i, j| matrix.data[i * cols + j])
}

/// Convert faer Mat to Achronyme Matrix
fn faer_to_matrix(mat: Mat<f64>) -> Result<Matrix, String> {
    let rows = mat.nrows();
    let cols = mat.ncols();
    let mut data = Vec::with_capacity(rows * cols);

    for i in 0..rows {
        for j in 0..cols {
            data.push(mat.read(i, j));
        }
    }

    Matrix::new(rows, cols, data).map_err(|e| e.to_string())
}

/// Compute eigenvalues of a square matrix
///
/// Returns the eigenvalues (potentially complex) of the matrix.
///
/// # Arguments
/// * `matrix` - Square matrix
///
/// # Returns
/// Vector of complex eigenvalues
///
/// # Example
/// ```
/// use achronyme_linalg::eigenvalues;
/// use achronyme_types::matrix::Matrix;
///
/// let a = Matrix::new(2, 2, vec![
///     1.0, 2.0,
///     3.0, 4.0
/// ]).unwrap();
///
/// let eigs = eigenvalues(&a).unwrap();
/// ```
pub fn eigenvalues(matrix: &Matrix) -> Result<Vec<Complex>, String> {
    if matrix.rows != matrix.cols {
        return Err("Eigenvalue computation requires square matrix".to_string());
    }

    let mat = matrix_to_faer(matrix);

    // Compute eigenvalue decomposition
    let evd = mat.eigendecomposition::<c64>();
    let eigenvals = evd.s().column_vector();

    // Convert to Achronyme Complex
    let result: Vec<Complex> = (0..eigenvals.nrows())
        .map(|i| {
            let val = eigenvals.read(i);
            Complex::new(val.re, val.im)
        })
        .collect();

    Ok(result)
}

/// Compute eigenvalues and eigenvectors of a square matrix
///
/// Returns both eigenvalues and corresponding eigenvectors.
///
/// # Arguments
/// * `matrix` - Square matrix
///
/// # Returns
/// Tuple of (eigenvalues, eigenvectors_matrix) where each column
/// of eigenvectors_matrix is an eigenvector.
///
/// # Example
/// ```
/// use achronyme_linalg::eigenvectors;
/// use achronyme_types::matrix::Matrix;
///
/// let a = Matrix::new(2, 2, vec![
///     4.0, 1.0,
///     2.0, 3.0
/// ]).unwrap();
///
/// let (eigs, vecs) = eigenvectors(&a).unwrap();
/// ```
pub fn eigenvectors(matrix: &Matrix) -> Result<(Vec<Complex>, Matrix), String> {
    if matrix.rows != matrix.cols {
        return Err("Eigenvector computation requires square matrix".to_string());
    }

    let mat = matrix_to_faer(matrix);

    // Compute eigenvalue decomposition
    let evd = mat.eigendecomposition::<c64>();
    let eigenvals = evd.s().column_vector();
    let eigenvecs_complex = evd.u();

    // Convert eigenvalues to Achronyme Complex
    let eigs: Vec<Complex> = (0..eigenvals.nrows())
        .map(|i| {
            let val = eigenvals.read(i);
            Complex::new(val.re, val.im)
        })
        .collect();

    // Extract real part of eigenvectors for real matrices
    // Note: For general matrices, eigenvectors can be complex
    let n = eigenvecs_complex.nrows();
    let m = eigenvecs_complex.ncols();
    let mut real_vecs = Vec::with_capacity(n * m);

    for i in 0..n {
        for j in 0..m {
            real_vecs.push(eigenvecs_complex.read(i, j).re);
        }
    }

    let vecs_matrix = Matrix::new(n, m, real_vecs).map_err(|e| e.to_string())?;

    Ok((eigs, vecs_matrix))
}

/// Power iteration method for finding dominant eigenvalue
///
/// Iteratively computes the largest eigenvalue (by magnitude) and its eigenvector.
///
/// # Arguments
/// * `matrix` - Square matrix
/// * `max_iterations` - Maximum number of iterations
/// * `tolerance` - Convergence tolerance
///
/// # Returns
/// Tuple of (eigenvalue, eigenvector as Matrix column vector)
pub fn power_iteration(matrix: &Matrix, max_iterations: usize, tolerance: f64) -> Result<(f64, Matrix), String> {
    if matrix.rows != matrix.cols {
        return Err("Power iteration requires square matrix".to_string());
    }

    let mat = matrix_to_faer(matrix);
    let n = mat.nrows();

    // Initialize with random vector
    let mut v = Col::from_fn(n, |i| if i == 0 { 1.0 } else { 0.0 });

    let mut eigenvalue = 0.0;

    for _ in 0..max_iterations {
        // Multiply: v_new = A * v
        let v_new = &mat * &v;

        // Find norm and eigenvalue
        let norm: f64 = (0..n).map(|i| v_new.read(i).powi(2)).sum::<f64>().sqrt();
        let new_eigenvalue = norm;

        // Normalize
        v = v_new * (1.0 / norm);

        // Check convergence
        if (new_eigenvalue - eigenvalue).abs() < tolerance {
            eigenvalue = new_eigenvalue;
            break;
        }

        eigenvalue = new_eigenvalue;
    }

    // Convert eigenvector to matrix
    let eigenvector_data: Vec<f64> = (0..n).map(|i| v.read(i)).collect();
    let eigenvector = Matrix::new(n, 1, eigenvector_data).map_err(|e| e.to_string())?;

    Ok((eigenvalue, eigenvector))
}

/// QR algorithm for computing all eigenvalues
///
/// Uses QR iteration to find all eigenvalues of a matrix.
///
/// # Arguments
/// * `matrix` - Square matrix
/// * `max_iterations` - Maximum number of iterations
/// * `tolerance` - Convergence tolerance
///
/// # Returns
/// Vector of real eigenvalues (imaginary parts discarded)
pub fn qr_eigenvalues(matrix: &Matrix, max_iterations: usize, tolerance: f64) -> Result<Vec<f64>, String> {
    if matrix.rows != matrix.cols {
        return Err("QR eigenvalue algorithm requires square matrix".to_string());
    }

    let mut a = matrix_to_faer(matrix);

    for _ in 0..max_iterations {
        let qr = a.qr();
        let q = qr.compute_thin_q();
        let r = qr.compute_thin_r();

        // A_new = R * Q
        a = &r * &q;

        // Check for convergence (off-diagonal elements small)
        let mut max_off_diag: f64 = 0.0;
        for i in 0..a.nrows() {
            for j in 0..a.ncols() {
                if i != j {
                    max_off_diag = max_off_diag.max(a.read(i, j).abs());
                }
            }
        }

        if max_off_diag < tolerance {
            break;
        }
    }

    // Extract diagonal (eigenvalues)
    let eigenvalues: Vec<f64> = (0..a.nrows()).map(|i| a.read(i, i)).collect();

    Ok(eigenvalues)
}

/// Eigendecomposition for symmetric matrices
///
/// Computes eigenvalues and eigenvectors for symmetric matrices using QR algorithm.
///
/// # Arguments
/// * `matrix` - Symmetric square matrix
/// * `max_iterations` - Maximum number of iterations
/// * `tolerance` - Convergence tolerance
///
/// # Returns
/// Tuple of (eigenvalues vector, eigenvectors matrix)
pub fn eigen_symmetric(matrix: &Matrix, _max_iterations: usize, _tolerance: f64) -> Result<(Vec<f64>, Matrix), String> {
    if matrix.rows != matrix.cols {
        return Err("Symmetric eigendecomposition requires square matrix".to_string());
    }

    // For now, use the general eigendecomposition
    // TODO: Implement specialized symmetric algorithm for better performance
    let (eigs, vecs) = eigenvectors(matrix)?;

    // Extract real parts only
    let eigenvalues: Vec<f64> = eigs.iter().map(|c| c.re).collect();

    Ok((eigenvalues, vecs))
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_eigenvalues_2x2() {
        // Matrix with known eigenvalues
        let a = Matrix::new(2, 2, vec![
            4.0, 1.0,
            2.0, 3.0
        ]).unwrap();

        let result = eigenvalues(&a);
        assert!(result.is_ok());

        let eigs = result.unwrap();
        assert_eq!(eigs.len(), 2);

        // Eigenvalues should be 5 and 2
        let mut real_parts: Vec<f64> = eigs.iter().map(|c| c.re).collect();
        real_parts.sort_by(|a, b| a.partial_cmp(b).unwrap());

        assert_relative_eq!(real_parts[0], 2.0, epsilon = 1e-10);
        assert_relative_eq!(real_parts[1], 5.0, epsilon = 1e-10);
    }

    #[test]
    fn test_eigenvalues_3x3() {
        // Identity matrix has eigenvalues all equal to 1
        let a = Matrix::identity(3);

        let result = eigenvalues(&a);
        assert!(result.is_ok());

        let eigs = result.unwrap();
        assert_eq!(eigs.len(), 3);

        for eig in eigs {
            assert_relative_eq!(eig.re, 1.0, epsilon = 1e-10);
            assert_relative_eq!(eig.im, 0.0, epsilon = 1e-10);
        }
    }

    #[test]
    fn test_eigenvectors() {
        let a = Matrix::new(2, 2, vec![
            4.0, 1.0,
            2.0, 3.0
        ]).unwrap();

        let result = eigenvectors(&a);
        assert!(result.is_ok());

        let (eigs, vecs) = result.unwrap();
        assert_eq!(eigs.len(), 2);
        assert_eq!(vecs.rows, 2);
        assert_eq!(vecs.cols, 2);
    }

    #[test]
    fn test_eigenvalues_nonsquare_fails() {
        let a = Matrix::new(2, 3, vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0
        ]).unwrap();

        let result = eigenvalues(&a);
        assert!(result.is_err());
    }
}
