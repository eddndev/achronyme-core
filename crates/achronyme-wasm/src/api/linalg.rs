use wasm_bindgen::prelude::*;
use crate::state::{Handle, HANDLES};
use achronyme_types::value::Value;
use achronyme_types::tensor::RealTensor;

// ============================================================================
// Linear Algebra Bindings (Compatible with C++ SDK)
// ============================================================================

#[wasm_bindgen(js_name = LUResult)]
pub struct LuResult {
    #[wasm_bindgen(readonly, js_name = L)]
    pub l: Handle,
    #[wasm_bindgen(readonly, js_name = U)]
    pub u: Handle,
    #[wasm_bindgen(readonly, js_name = P)]
    pub p: Handle,
}

#[wasm_bindgen(js_name = lu)]
pub fn lu_decomposition(handle: Handle) -> Result<LuResult, JsValue> {
    HANDLES.with(|h| {
        // First, borrow immutably to read and compute
        let (l_mat, u_mat, p_matrix) = {
            let handles = h.borrow();
            let value = handles.get(handle)
                .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

            match value {
                Value::Tensor(t) => {
                    if !t.is_matrix() {
                        return Err(JsValue::from_str("LU decomposition requires matrix (rank-2 tensor)"));
                    }
                    let (l, u, p) = achronyme_linalg::lu_decomposition(t)
                        .map_err(|e| JsValue::from_str(&e))?;

                    // Convert permutation vector to permutation matrix
                    let n = p.len();
                    let mut p_data = vec![0.0; n * n];
                    for (i, &pi) in p.iter().enumerate() {
                        p_data[i * n + pi] = 1.0;
                    }
                    let p_matrix = RealTensor::matrix(n, n, p_data)
                        .map_err(|e| JsValue::from_str(&e.to_string()))?;

                    Ok((l, u, p_matrix))
                }
                _ => Err(JsValue::from_str("LU decomposition requires tensor"))
            }
        }?; // Immutable borrow is dropped here

        // Now borrow mutably to create handles
        let mut handles_mut = h.borrow_mut();
        let l_handle = handles_mut.create(Value::Tensor(l_mat));
        let u_handle = handles_mut.create(Value::Tensor(u_mat));
        let p_handle = handles_mut.create(Value::Tensor(p_matrix));

        Ok(LuResult { l: l_handle, u: u_handle, p: p_handle })
    })
}

/// Matrix inverse
#[wasm_bindgen(js_name = inverse)]
pub fn matrix_inverse(handle: Handle) -> Result<Handle, JsValue> {
    HANDLES.with(|h| {
        let inv_matrix = {
            let handles = h.borrow();
            let value = handles.get(handle)
                .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

            match value {
                Value::Tensor(t) => {
                    if !t.is_matrix() {
                        return Err(JsValue::from_str("Matrix inverse requires matrix (rank-2 tensor)"));
                    }
                    achronyme_linalg::inverse(t)
                        .map_err(|e| JsValue::from_str(&e))?
                }
                _ => return Err(JsValue::from_str("Matrix inverse requires tensor"))
            }
        };

        Ok(h.borrow_mut().create(Value::Tensor(inv_matrix)))
    })
}

// ============================================================================
// QR Decomposition
// ============================================================================

#[wasm_bindgen(js_name = QRResult)]
pub struct QrResult {
    #[wasm_bindgen(readonly, js_name = Q)]
    pub q: Handle,
    #[wasm_bindgen(readonly, js_name = R)]
    pub r: Handle,
}

#[wasm_bindgen(js_name = qr)]
pub fn qr_decomposition(handle: Handle) -> Result<QrResult, JsValue> {
    HANDLES.with(|h| {
        // First, borrow immutably to read and compute
        let (q_mat, r_mat) = {
            let handles = h.borrow();
            let value = handles.get(handle)
                .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

            match value {
                Value::Tensor(t) => {
                    if !t.is_matrix() {
                        return Err(JsValue::from_str("QR decomposition requires matrix (rank-2 tensor)"));
                    }
                    achronyme_linalg::qr_decomposition(t)
                        .map_err(|e| JsValue::from_str(&e))
                }
                _ => Err(JsValue::from_str("QR decomposition requires tensor"))
            }
        }?; // Immutable borrow is dropped here

        // Now borrow mutably to create handles
        let mut handles_mut = h.borrow_mut();
        let q_handle = handles_mut.create(Value::Tensor(q_mat));
        let r_handle = handles_mut.create(Value::Tensor(r_mat));

        Ok(QrResult { q: q_handle, r: r_handle })
    })
}

// ============================================================================
// Cholesky Decomposition
// ============================================================================

#[wasm_bindgen(js_name = cholesky)]
pub fn cholesky_decomposition(handle: Handle) -> Result<Handle, JsValue> {
    HANDLES.with(|h| {
        // First, borrow immutably to read and compute
        let l = {
            let handles = h.borrow();
            let value = handles.get(handle)
                .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

            match value {
                Value::Tensor(t) => {
                    if !t.is_matrix() {
                        return Err(JsValue::from_str("Cholesky decomposition requires matrix (rank-2 tensor)"));
                    }
                    achronyme_linalg::cholesky_decomposition(t)
                        .map_err(|e| JsValue::from_str(&e))
                }
                _ => Err(JsValue::from_str("Cholesky decomposition requires tensor"))
            }
        }?; // Immutable borrow is dropped here

        // Now borrow mutably to create handle
        Ok(h.borrow_mut().create(Value::Tensor(l)))
    })
}

// ============================================================================
// SVD Decomposition
// ============================================================================

#[wasm_bindgen(js_name = SVDResult)]
pub struct SvdResult {
    #[wasm_bindgen(readonly, js_name = U)]
    pub u: Handle,
    #[wasm_bindgen(readonly, js_name = S)]
    pub s: Handle,
    #[wasm_bindgen(readonly, js_name = V)]
    pub v: Handle,
}

#[wasm_bindgen(js_name = svd)]
pub fn svd_decomposition(handle: Handle) -> Result<SvdResult, JsValue> {
    HANDLES.with(|h| {
        // First, borrow immutably to read and compute
        let (u_mat, s_vec, v_mat) = {
            let handles = h.borrow();
            let value = handles.get(handle)
                .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

            match value {
                Value::Tensor(t) => {
                    if !t.is_matrix() {
                        return Err(JsValue::from_str("SVD requires matrix (rank-2 tensor)"));
                    }
                    achronyme_linalg::svd_decomposition(t)
                        .map_err(|e| JsValue::from_str(&format!("{}", e)))
                }
                _ => Err(JsValue::from_str("SVD requires tensor"))
            }
        }?; // Immutable borrow is dropped here

        // Now borrow mutably to create handles
        let mut handles_mut = h.borrow_mut();
        let u_handle = handles_mut.create(Value::Tensor(u_mat));
        let s_handle = handles_mut.create(Value::Vector(s_vec.into_iter().map(Value::Number).collect()));
        let v_handle = handles_mut.create(Value::Tensor(v_mat));

        Ok(SvdResult { u: u_handle, s: s_handle, v: v_handle })
    })
}

// ============================================================================
// Eigenvalue Solvers
// ============================================================================

#[wasm_bindgen(js_name = PowerIterationResult)]
pub struct PowerIterationResult {
    #[wasm_bindgen(readonly)]
    pub eigenvalue: f64,
    #[wasm_bindgen(readonly)]
    pub eigenvector: Handle,
}

#[wasm_bindgen(js_name = powerIteration)]
pub fn power_iteration(
    handle: Handle,
    max_iterations: usize,
    tolerance: f64
) -> Result<PowerIterationResult, JsValue> {
    HANDLES.with(|h| {
        // First, borrow immutably to read and compute
        let (eigenvalue, eigenvector_matrix) = {
            let handles = h.borrow();
            let value = handles.get(handle)
                .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

            match value {
                Value::Tensor(t) => {
                    if !t.is_matrix() {
                        return Err(JsValue::from_str("Power iteration requires matrix (rank-2 tensor)"));
                    }
                    achronyme_linalg::power_iteration(t, max_iterations, tolerance)
                        .map_err(|e| JsValue::from_str(&format!("{}", e)))
                }
                _ => Err(JsValue::from_str("Power iteration requires tensor"))
            }
        }?; // Immutable borrow is dropped here

        // Now borrow mutably to create handle
        let eigenvector_handle = h.borrow_mut().create(Value::Tensor(eigenvector_matrix));

        Ok(PowerIterationResult {
            eigenvalue,
            eigenvector: eigenvector_handle,
        })
    })
}

#[wasm_bindgen(js_name = qrEigenvalues)]
pub fn qr_eigenvalues(
    handle: Handle,
    max_iterations: usize,
    tolerance: f64
) -> Result<Handle, JsValue> {
    HANDLES.with(|h| {
        // First, borrow immutably to read and compute
        let eigenvalues_vec = {
            let handles = h.borrow();
            let value = handles.get(handle)
                .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

            match value {
                Value::Tensor(t) => {
                    if !t.is_matrix() {
                        return Err(JsValue::from_str("QR eigenvalues requires matrix (rank-2 tensor)"));
                    }
                    achronyme_linalg::qr_eigenvalues(t, max_iterations, tolerance)
                        .map_err(|e| JsValue::from_str(&format!("{}", e)))
                }
                _ => Err(JsValue::from_str("QR eigenvalues requires tensor"))
            }
        }?; // Immutable borrow is dropped here

        // Now borrow mutably to create handle
        Ok(h.borrow_mut().create(Value::Vector(eigenvalues_vec.into_iter().map(Value::Number).collect())))
    })
}

#[wasm_bindgen(js_name = EigenResult)]
pub struct EigenResult {
    #[wasm_bindgen(readonly)]
    pub eigenvalues: Handle,
    #[wasm_bindgen(readonly)]
    pub eigenvectors: Handle,
}

#[wasm_bindgen(js_name = eigenSymmetric)]
pub fn eigen_symmetric(
    handle: Handle,
    max_iterations: usize,
    tolerance: f64
) -> Result<EigenResult, JsValue> {
    HANDLES.with(|h| {
        // First, borrow immutably to read and compute
        let (eigenvalues_vec, eigenvectors_mat) = {
            let handles = h.borrow();
            let value = handles.get(handle)
                .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

            match value {
                Value::Tensor(t) => {
                    if !t.is_matrix() {
                        return Err(JsValue::from_str("Eigen symmetric requires matrix (rank-2 tensor)"));
                    }
                    achronyme_linalg::eigen_symmetric(t, max_iterations, tolerance)
                        .map_err(|e| JsValue::from_str(&format!("{}", e)))
                }
                _ => Err(JsValue::from_str("Eigen symmetric requires tensor"))
            }
        }?; // Immutable borrow is dropped here

        // Now borrow mutably to create handles
        let mut handles_mut = h.borrow_mut();
        let eigenvalues_handle = handles_mut.create(Value::Vector(eigenvalues_vec.into_iter().map(Value::Number).collect()));
        let eigenvectors_handle = handles_mut.create(Value::Tensor(eigenvectors_mat));

        Ok(EigenResult {
            eigenvalues: eigenvalues_handle,
            eigenvectors: eigenvectors_handle,
        })
    })
}

// ============================================================================
// Matrix Utilities
// ============================================================================

#[wasm_bindgen(js_name = isSymmetric)]
pub fn is_symmetric(handle: Handle, tolerance: f64) -> Result<bool, JsValue> {
    HANDLES.with(|h| {
        let handles = h.borrow();
        let value = handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

        match value {
            Value::Tensor(t) => {
                if !t.is_matrix() {
                    return Err(JsValue::from_str("is_symmetric requires matrix (rank-2 tensor)"));
                }
                Ok(achronyme_linalg::is_symmetric(t, tolerance))
            }
            _ => Err(JsValue::from_str("is_symmetric requires tensor"))
        }
    })
}

#[wasm_bindgen(js_name = isPositiveDefinite)]
pub fn is_positive_definite(handle: Handle) -> Result<bool, JsValue> {
    HANDLES.with(|h| {
        let handles = h.borrow();
        let value = handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

        match value {
            Value::Tensor(t) => {
                if !t.is_matrix() {
                    return Err(JsValue::from_str("is_positive_definite requires matrix (rank-2 tensor)"));
                }
                Ok(achronyme_linalg::is_positive_definite(t))
            }
            _ => Err(JsValue::from_str("is_positive_definite requires tensor"))
        }
    })
}

#[wasm_bindgen(js_name = identity)]
pub fn identity(n: usize) -> Result<Handle, JsValue> {
    let tensor = RealTensor::eye(n);

    Ok(HANDLES.with(|h| h.borrow_mut().create(Value::Tensor(tensor))))
}
