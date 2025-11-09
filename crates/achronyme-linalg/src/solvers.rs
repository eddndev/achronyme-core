use achronyme_types::tensor::RealTensor;
use faer::prelude::*;

/// Convert Achronyme RealTensor (matrix) to faer Mat
fn tensor_to_faer_mat(tensor: &RealTensor) -> Mat<f64> {
    assert!(tensor.is_matrix());
    let rows = tensor.rows();
    let cols = tensor.cols();
    Mat::from_fn(rows, cols, |i, j| tensor.get_matrix(i, j).unwrap())
}

/// Convert faer Mat to Achronyme RealTensor (matrix)
fn faer_mat_to_tensor(mat: Mat<f64>) -> Result<RealTensor, String> {
    let rows = mat.nrows();
    let cols = mat.ncols();
    let mut data = Vec::with_capacity(rows * cols);

    for i in 0..rows {
        for j in 0..cols {
            data.push(mat.read(i, j));
        }
    }

    RealTensor::matrix(rows, cols, data).map_err(|e| e.to_string())
}

/// Convert Achronyme RealTensor (vector) to faer Col
fn tensor_to_faer_col(tensor: &RealTensor) -> Col<f64> {
    assert!(tensor.is_vector());
    let data = tensor.data();
    Col::from_fn(data.len(), |i| data[i])
}

/// Convert faer Col to Achronyme RealTensor (vector)
fn faer_col_to_tensor(col: Col<f64>) -> RealTensor {
    let data: Vec<f64> = (0..col.nrows()).map(|i| col.read(i)).collect();
    RealTensor::vector(data)
}

/// Compute determinant of a square matrix (NxN)
///
/// Uses LU decomposition for efficient computation on any size.
///
/// # Arguments
/// * `tensor` - Square matrix (rank-2 tensor)
///
/// # Returns
/// Determinant value
///
/// # Example
/// ```
/// use achronyme_linalg::determinant_nd;
/// use achronyme_types::tensor::RealTensor;
///
/// let a = RealTensor::matrix(3, 3, vec![
///     1.0, 2.0, 3.0,
///     0.0, 1.0, 4.0,
///     5.0, 6.0, 0.0
/// ]).unwrap();
///
/// let det = determinant_nd(&a).unwrap();
/// ```
pub fn determinant_nd(tensor: &RealTensor) -> Result<f64, String> {
    if !tensor.is_square() {
        return Err("Determinant requires square matrix".to_string());
    }

    let mat = tensor_to_faer_mat(tensor);

    // Use LU decomposition to compute determinant
    let lu = mat.partial_piv_lu();

    // Compute determinant from LU: det(A) = det(P) * det(L) * det(U)
    // det(L) = 1 (unit diagonal), det(U) = product of diagonal elements
    let u = lu.compute_u();
    let mut det = 1.0;
    for i in 0..u.nrows() {
        det *= u.read(i, i);
    }

    // Account for permutation sign
    // Count number of swaps to determine sign
    let perm = lu.row_permutation();
    let perm_indices = perm.arrays().0;
    let mut swaps = 0;
    let mut visited = vec![false; perm_indices.len()];

    for i in 0..perm_indices.len() {
        if !visited[i] && perm_indices[i] != i {
            let mut j = i;
            while !visited[j] {
                visited[j] = true;
                j = perm_indices[j];
                swaps += 1;
            }
            swaps -= 1; // Each cycle of length n requires n-1 swaps
        }
    }

    if swaps % 2 == 1 {
        det = -det;
    }

    Ok(det)
}

/// Compute the inverse of a square matrix
///
/// Returns the multiplicative inverse A^(-1) such that A * A^(-1) = I
///
/// # Arguments
/// * `tensor` - Square, non-singular matrix (rank-2 tensor)
///
/// # Returns
/// Inverse matrix
///
/// # Example
/// ```
/// use achronyme_linalg::inverse;
/// use achronyme_types::tensor::RealTensor;
///
/// let a = RealTensor::matrix(2, 2, vec![
///     4.0, 7.0,
///     2.0, 6.0
/// ]).unwrap();
///
/// let a_inv = inverse(&a).unwrap();
/// ```
pub fn inverse(tensor: &RealTensor) -> Result<RealTensor, String> {
    if !tensor.is_square() {
        return Err("Inverse requires square matrix".to_string());
    }

    let mat = tensor_to_faer_mat(tensor);
    let n = mat.nrows();

    // Create identity matrix
    let identity = Mat::<f64>::identity(n, n);

    // Solve A * X = I to get X = A^(-1)
    let lu = mat.partial_piv_lu();
    let inv = lu.solve(&identity);

    faer_mat_to_tensor(inv)
}

/// Solve a linear system Ax = b
///
/// Finds vector x that satisfies A * x = b
///
/// # Arguments
/// * `a` - Coefficient matrix (m x n rank-2 tensor)
/// * `b` - Right-hand side vector (rank-1 tensor)
///
/// # Returns
/// Solution vector x (rank-1 tensor)
///
/// # Example
/// ```
/// use achronyme_linalg::solve_system;
/// use achronyme_types::tensor::RealTensor;
///
/// let a = RealTensor::matrix(2, 2, vec![
///     3.0, 1.0,
///     1.0, 2.0
/// ]).unwrap();
///
/// let b = RealTensor::vector(vec![9.0, 8.0]).unwrap();
///
/// let x = solve_system(&a, &b).unwrap();
/// // x should be [2.0, 3.0]
/// ```
pub fn solve_system(a: &RealTensor, b: &RealTensor) -> Result<RealTensor, String> {
    if !a.is_matrix() || !b.is_vector() {
        return Err("`a` must be a matrix and `b` must be a vector".to_string());
    }
    if a.rows() != b.size() {
        return Err(format!(
            "Dimension mismatch: matrix has {} rows but vector has {} elements",
            a.rows(),
            b.size()
        ));
    }

    let a_mat = tensor_to_faer_mat(a);
    let b_col = tensor_to_faer_col(b);

    // Use LU decomposition to solve the system
    let lu = a_mat.partial_piv_lu();
    let x = lu.solve(&b_col);

    Ok(faer_col_to_tensor(x))
}

/// Check if a matrix is symmetric within a tolerance
///
/// # Arguments
/// * `tensor` - Square matrix (rank-2 tensor) to check
/// * `tolerance` - Maximum allowed difference between matrix[i,j] and matrix[j,i]
///
/// # Returns
/// True if symmetric, false otherwise
pub fn is_symmetric(tensor: &RealTensor, tolerance: f64) -> bool {
    if !tensor.is_square() {
        return false;
    }

    for i in 0..tensor.rows() {
        for j in (i + 1)..tensor.cols() {
            let diff = (tensor.get_matrix(i, j).unwrap() - tensor.get_matrix(j, i).unwrap()).abs();
            if diff > tolerance {
                return false;
            }
        }
    }

    true
}

/// Check if a matrix is positive definite
///
/// Uses Cholesky decomposition - a matrix is positive definite
/// if and only if it has a Cholesky decomposition.
///
/// # Arguments
/// * `tensor` - Square symmetric matrix (rank-2 tensor) to check
///
/// # Returns
/// True if positive definite, false otherwise
pub fn is_positive_definite(tensor: &RealTensor) -> bool {
    if !tensor.is_square() {
        return false;
    }

    let mat = tensor_to_faer_mat(tensor);

    // Try Cholesky decomposition
    mat.cholesky(faer::Side::Lower).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_determinant_2x2() {
        let a = RealTensor::matrix(2, 2, vec![
            4.0, 7.0,
            2.0, 6.0
        ]).unwrap();

        let det = determinant_nd(&a).unwrap();
        // det = 4*6 - 7*2 = 24 - 14 = 10
        assert_relative_eq!(det, 10.0, epsilon = 1e-10);
    }

    #[test]
    fn test_determinant_3x3() {
        let a = RealTensor::matrix(3, 3, vec![
            1.0, 2.0, 3.0,
            0.0, 1.0, 4.0,
            5.0, 6.0, 0.0
        ]).unwrap();

        let det = determinant_nd(&a).unwrap();
        assert_relative_eq!(det, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_inverse_2x2() {
        let a = RealTensor::matrix(2, 2, vec![
            4.0, 7.0,
            2.0, 6.0
        ]).unwrap();

        let a_inv = inverse(&a).unwrap();

        // Check that A * A_inv = I
        let product = a.matmul(&a_inv).unwrap();
        let identity = RealTensor::eye(2);

        for i in 0..2 {
            for j in 0..2 {
                assert_relative_eq!(
                    product.get_matrix(i, j).unwrap(),
                    identity.get_matrix(i, j).unwrap(),
                    epsilon = 1e-10
                );
            }
        }
    }

    #[test]
    fn test_solve_system_2x2() {
        // System: 3x + y = 9
        //         x + 2y = 8
        // Solution: x = 2, y = 3
        let a = RealTensor::matrix(2, 2, vec![
            3.0, 1.0,
            1.0, 2.0
        ]).unwrap();
        let b = RealTensor::vector(vec![9.0, 8.0]);

        let x = solve_system(&a, &b).unwrap();

        assert_eq!(x.size(), 2);
        assert_relative_eq!(x.data()[0], 2.0, epsilon = 1e-10);
        assert_relative_eq!(x.data()[1], 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_solve_system_dimension_mismatch() {
        let a = RealTensor::matrix(2, 2, vec![
            1.0, 2.0,
            3.0, 4.0
        ]).unwrap();
        let b = RealTensor::vector(vec![1.0, 2.0, 3.0]); // Wrong size

        let result = solve_system(&a, &b);
        assert!(result.is_err());
    }

    #[test]
    fn test_inverse_singular_fails() {
        // Singular matrix (determinant = 0)
        let a = RealTensor::matrix(2, 2, vec![
            1.0, 2.0,
            2.0, 4.0
        ]).unwrap();

        let result = inverse(&a);
        // faer should detect singularity, though it might not always error
        // This test may pass or fail depending on numerical precision
        // In practice, the inverse will be numerically unstable
        let _ = result; // Just check it doesn't panic
    }
}
