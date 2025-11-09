use achronyme_types::tensor::RealTensor;
use faer::prelude::*;

/// Convert Achronyme RealTensor to faer Mat
fn tensor_to_faer_mat(tensor: &RealTensor) -> Mat<f64> {
    assert!(tensor.is_matrix());
    let rows = tensor.rows();
    let cols = tensor.cols();
    Mat::from_fn(rows, cols, |i, j| tensor.get_matrix(i, j).unwrap())
}

/// Convert faer Mat to Achronyme RealTensor
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

/// Convert faer MatRef to Achronyme RealTensor
fn faer_mat_ref_to_tensor(mat: MatRef<f64>) -> Result<RealTensor, String> {
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

/// LU Decomposition with Partial Pivoting
///
/// Decomposes matrix A into L (lower triangular) and U (upper triangular)
/// such that P * A = L * U, where P is a permutation matrix.
///
/// Returns: (L, U, P) or error
///
/// # Example
/// ```
/// use achronyme_linalg::lu_decomposition;
/// use achronyme_types::tensor::RealTensor;
///
/// let a = RealTensor::matrix(3, 3, vec![
///     2.0, 1.0, 1.0,
///     4.0, 3.0, 3.0,
///     8.0, 7.0, 9.0
/// ]).unwrap();
///
/// let (l, u, _p) = lu_decomposition(&a).unwrap();
/// ```
pub fn lu_decomposition(tensor: &RealTensor) -> Result<(RealTensor, RealTensor, Vec<usize>), String> {
    if !tensor.is_square() {
        return Err("LU decomposition requires square matrix".to_string());
    }

    let mat = tensor_to_faer_mat(tensor);
    let lu = mat.partial_piv_lu();

    let l = lu.compute_l();
    let u = lu.compute_u();
    let p = lu.row_permutation();

    // Convert permutation to vector
    let perm_vec: Vec<usize> = (0..tensor.rows())
        .map(|i| p.arrays().0[i])
        .collect();

    let l_tensor = faer_mat_to_tensor(l)?;
    let u_tensor = faer_mat_to_tensor(u)?;

    Ok((l_tensor, u_tensor, perm_vec))
}

/// Cholesky Decomposition
///
/// Decomposes a symmetric positive-definite matrix A into L * L^T
/// where L is a lower triangular matrix.
///
/// Returns: L or error
///
/// # Example
/// ```
/// use achronyme_linalg::cholesky_decomposition;
/// use achronyme_types::tensor::RealTensor;
///
/// let a = RealTensor::matrix(3, 3, vec![
///     4.0, 2.0, 1.0,
///     2.0, 3.0, 1.0,
///     1.0, 1.0, 2.0
/// ]).unwrap();
///
/// let l = cholesky_decomposition(&a).unwrap();
/// ```
pub fn cholesky_decomposition(tensor: &RealTensor) -> Result<RealTensor, String> {
    if !tensor.is_square() {
        return Err("Cholesky decomposition requires square matrix".to_string());
    }

    let mat = tensor_to_faer_mat(tensor);

    // Perform Cholesky decomposition
    let chol = mat
        .cholesky(faer::Side::Lower)
        .map_err(|_| "Cholesky decomposition failed (matrix not positive definite?)".to_string())?;

    let l = chol.compute_l();
    faer_mat_to_tensor(l)
}

/// QR Decomposition
///
/// Decomposes matrix A into Q (orthogonal) and R (upper triangular)
/// such that A = Q * R.
///
/// Returns: (Q, R) or error
///
/// # Example
/// ```
/// use achronyme_linalg::qr_decomposition;
/// use achronyme_types::tensor::RealTensor;
///
/// let a = RealTensor::matrix(3, 2, vec![
///     1.0, 1.0,
///     1.0, 2.0,
///     1.0, 3.0
/// ]).unwrap();
///
/// let (q, r) = qr_decomposition(&a).unwrap();
/// ```
pub fn qr_decomposition(tensor: &RealTensor) -> Result<(RealTensor, RealTensor), String> {
    let mat = tensor_to_faer_mat(tensor);

    // Perform QR decomposition
    let qr = mat.qr();

    let q = qr.compute_thin_q();
    let r = qr.compute_thin_r();

    let q_tensor = faer_mat_to_tensor(q)?;
    let r_tensor = faer_mat_to_tensor(r)?;

    Ok((q_tensor, r_tensor))
}

/// SVD (Singular Value Decomposition)
///
/// Decomposes matrix A into U, S, V^T such that A = U * S * V^T
/// where:
/// - U: Left singular vectors (m x m)
/// - S: Singular values (diagonal, min(m,n) elements)
/// - V^T: Right singular vectors transposed (n x n)
///
/// Returns: (U, singular_values, V^T) or error
///
/// # Example
/// ```
/// use achronyme_linalg::svd_decomposition;
/// use achronyme_types::tensor::RealTensor;
///
/// let a = RealTensor::matrix(3, 2, vec![
///     1.0, 2.0,
///     3.0, 4.0,
///     5.0, 6.0
/// ]).unwrap();
///
/// let (u, s, vt) = svd_decomposition(&a).unwrap();
/// ```
pub fn svd_decomposition(tensor: &RealTensor) -> Result<(RealTensor, Vec<f64>, RealTensor), String> {
    let mat = tensor_to_faer_mat(tensor);

    // Perform SVD
    let svd = mat.thin_svd();

    let u = svd.u();
    let s = svd.s_diagonal();
    let v = svd.v();

    // Convert V to V^T by transposing during extraction
    let u_tensor = faer_mat_ref_to_tensor(u)?;
    let vt_tensor = faer_mat_ref_to_tensor(v.transpose().as_ref())?;

    // Extract singular values
    let singular_values: Vec<f64> = (0..s.nrows())
        .map(|i| s.read(i))
        .collect();

    Ok((u_tensor, singular_values, vt_tensor))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lu_decomposition() {
        let a = RealTensor::matrix(3, 3, vec![
            2.0, 1.0, 1.0,
            4.0, 3.0, 3.0,
            8.0, 7.0, 9.0
        ]).unwrap();

        let result = lu_decomposition(&a);
        assert!(result.is_ok());

        let (l, u, _p) = result.unwrap();
        assert_eq!(l.rows(), 3);
        assert_eq!(l.cols(), 3);
        assert_eq!(u.rows(), 3);
        assert_eq!(u.cols(), 3);
    }

    #[test]
    fn test_cholesky_decomposition() {
        // Symmetric positive-definite matrix
        let a = RealTensor::matrix(3, 3, vec![
            4.0, 2.0, 1.0,
            2.0, 3.0, 1.0,
            1.0, 1.0, 2.0
        ]).unwrap();

        let result = cholesky_decomposition(&a);
        assert!(result.is_ok());

        let l = result.unwrap();
        assert_eq!(l.rows(), 3);
        assert_eq!(l.cols(), 3);
    }

    #[test]
    fn test_qr_decomposition() {
        let a = RealTensor::matrix(3, 2, vec![
            1.0, 1.0,
            1.0, 2.0,
            1.0, 3.0
        ]).unwrap();

        let result = qr_decomposition(&a);
        assert!(result.is_ok());

        let (q, r) = result.unwrap();
        assert_eq!(q.rows(), 3);
        assert_eq!(r.rows(), 2);
    }

    #[test]
    fn test_svd_decomposition() {
        let a = RealTensor::matrix(3, 2, vec![
            1.0, 2.0,
            3.0, 4.0,
            5.0, 6.0
        ]).unwrap();

        let result = svd_decomposition(&a);
        assert!(result.is_ok());

        let (u, s, vt) = result.unwrap();
        assert_eq!(u.rows(), 3);
        assert_eq!(s.len(), 2); // min(3, 2)
        assert_eq!(vt.rows(), 2);
    }
}
