/**
 * Achronyme WASM Bindings
 *
 * Expone el evaluador Rust a JavaScript/WebAssembly con una API compatible
 * con la implementación de C++ para mantener compatibilidad con el SDK TypeScript.
 */

use wasm_bindgen::prelude::*;
use achronyme_eval::evaluator::Evaluator;
use achronyme_types::value::Value;
use achronyme_types::vector::Vector;
use achronyme_types::matrix::Matrix;
use achronyme_parser::lexer::Lexer;
use achronyme_parser::parser::Parser;
use std::collections::HashMap;
use std::cell::RefCell;

// ============================================================================
// Global State
// ============================================================================

thread_local! {
    static EVALUATOR: RefCell<Evaluator> = RefCell::new(Evaluator::new());
    static HANDLES: RefCell<HandleManager> = RefCell::new(HandleManager::new());
}

pub type Handle = u32;

struct HandleManager {
    next_handle: Handle,
    values: HashMap<Handle, Value>,
}

impl HandleManager {
    fn new() -> Self {
        Self {
            next_handle: 1,
            values: HashMap::new(),
        }
    }

    fn create(&mut self, value: Value) -> Handle {
        let handle = self.next_handle;
        self.next_handle += 1;
        self.values.insert(handle, value);
        handle
    }

    fn get(&self, handle: Handle) -> Option<&Value> {
        self.values.get(&handle)
    }

    fn release(&mut self, handle: Handle) {
        self.values.remove(&handle);
    }

    fn clear(&mut self) {
        self.values.clear();
        self.next_handle = 1;
    }
}

// ============================================================================
// Core Evaluation API (Compatible with C++ SDK)
// ============================================================================

/// Evalua una expresión y retorna el resultado como string
#[wasm_bindgen(js_name = eval)]
pub fn eval(expression: &str) -> Result<String, JsValue> {
    EVALUATOR.with(|evaluator| {
        let mut eval = evaluator.borrow_mut();

        // Parse
        let mut lexer = Lexer::new(expression);
        let tokens = lexer.tokenize()
            .map_err(|e| JsValue::from_str(&e))?;

        let mut parser = Parser::new(tokens);
        let ast = parser.parse()
            .map_err(|e| JsValue::from_str(&e))?;

        // Evaluate
        let result = eval.evaluate(&ast)
            .map_err(|e| JsValue::from_str(&e))?;

        // Format result (compatible with C++ output format)
        Ok(format_value(&result))
    })
}

/// Resetea el evaluador y libera todos los handles
#[wasm_bindgen]
pub fn reset() {
    EVALUATOR.with(|evaluator| {
        *evaluator.borrow_mut() = Evaluator::new();
    });
    HANDLES.with(|handles| {
        handles.borrow_mut().clear();
    });
}

// ============================================================================
// Memory Management (Emscripten-compatible interface)
// ============================================================================

#[wasm_bindgen]
pub fn _malloc(size: usize) -> *mut u8 {
    unsafe {
        let layout = std::alloc::Layout::from_size_align_unchecked(size, 8);
        std::alloc::alloc(layout)
    }
}

#[wasm_bindgen]
pub fn _free(ptr: *mut u8) {
    unsafe {
        if !ptr.is_null() {
            let layout = std::alloc::Layout::from_size_align_unchecked(1, 8);
            std::alloc::dealloc(ptr, layout);
        }
    }
}

// ============================================================================
// Handle-Based API (Fast Path - Same as C++)
// ============================================================================

#[wasm_bindgen]
pub fn createVectorFromBuffer(data_ptr: *const f64, len: usize) -> Handle {
    unsafe {
        let data = std::slice::from_raw_parts(data_ptr, len).to_vec();
        let vector = Vector::new(data);
        HANDLES.with(|h| h.borrow_mut().create(Value::Vector(vector)))
    }
}

/// Create vector from JavaScript array (easier for testing)
#[wasm_bindgen]
pub fn createVector(data: Vec<f64>) -> Handle {
    let vector = Vector::new(data);
    HANDLES.with(|h| h.borrow_mut().create(Value::Vector(vector)))
}

/// Get vector data from handle (for verification/extraction)
#[wasm_bindgen]
pub fn getVector(handle: Handle) -> Result<Vec<f64>, JsValue> {
    HANDLES.with(|handles| {
        let h = handles.borrow();
        match h.get(handle) {
            Some(Value::Vector(v)) => Ok(v.data().to_vec()),
            Some(_) => Err(JsValue::from_str("Handle does not reference a vector")),
            None => Err(JsValue::from_str("Invalid handle")),
        }
    })
}

#[wasm_bindgen]
pub fn createMatrixFromBuffer(data_ptr: *const f64, rows: usize, cols: usize) -> Result<Handle, JsValue> {
    unsafe {
        let len = rows * cols;
        let data = std::slice::from_raw_parts(data_ptr, len).to_vec();
        let matrix = Matrix::new(rows, cols, data)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(HANDLES.with(|h| h.borrow_mut().create(Value::Matrix(matrix))))
    }
}

#[wasm_bindgen]
pub fn bindVariableToHandle(name: &str, handle: Handle) -> Result<(), JsValue> {
    HANDLES.with(|h| {
        let handles = h.borrow();
        let value = handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?
            .clone();

        EVALUATOR.with(|e| {
            e.borrow_mut().environment_mut().define(name.to_string(), value)
                .map_err(|err| JsValue::from_str(&err))
        })
    })
}

#[wasm_bindgen]
pub fn releaseHandle(handle: Handle) {
    HANDLES.with(|h| h.borrow_mut().release(handle));
}

#[wasm_bindgen]
pub fn getVectorData(handle: Handle, length_ptr: *mut usize) -> *const f64 {
    HANDLES.with(|h| {
        let handles = h.borrow();
        if let Some(Value::Vector(vec)) = handles.get(handle) {
            unsafe {
                *length_ptr = vec.len();
            }
            vec.data().as_ptr()
        } else {
            std::ptr::null()
        }
    })
}

// ============================================================================
// Fast Path Math Operations (Same API as C++)
// ============================================================================

fn apply_unary_op<F>(handle: Handle, f: F) -> Result<Handle, JsValue>
where
    F: Fn(&Vector) -> Vector
{
    // First, get the value and compute the result
    let result = HANDLES.with(|h| {
        let handles = h.borrow();
        let value = handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

        match value {
            Value::Vector(vec) => Ok(f(vec)),
            Value::Number(n) => {
                let vec = Vector::new(vec![*n]);
                Ok(f(&vec))
            }
            _ => Err(JsValue::from_str("Operation requires vector or number"))
        }
    })?;

    // Now create the new handle (borrow is dropped)
    HANDLES.with(|h| Ok(h.borrow_mut().create(Value::Vector(result))))
}

#[wasm_bindgen]
pub fn sin_fast(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, |v: &Vector| {
        let result: Vec<f64> = v.data().iter().map(|x| x.sin()).collect();
        Vector::new(result)
    })
}

#[wasm_bindgen]
pub fn cos_fast(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, |v: &Vector| {
        let result: Vec<f64> = v.data().iter().map(|x| x.cos()).collect();
        Vector::new(result)
    })
}

#[wasm_bindgen]
pub fn tan_fast(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, |v: &Vector| {
        let result: Vec<f64> = v.data().iter().map(|x| x.tan()).collect();
        Vector::new(result)
    })
}

#[wasm_bindgen]
pub fn sqrt_fast(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, |v: &Vector| {
        let result: Vec<f64> = v.data().iter().map(|x| x.sqrt()).collect();
        Vector::new(result)
    })
}

#[wasm_bindgen]
pub fn exp_fast(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, |v: &Vector| {
        let result: Vec<f64> = v.data().iter().map(|x| x.exp()).collect();
        Vector::new(result)
    })
}

#[wasm_bindgen]
pub fn abs_fast(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, |v: &Vector| {
        let result: Vec<f64> = v.data().iter().map(|x| x.abs()).collect();
        Vector::new(result)
    })
}

#[wasm_bindgen]
pub fn ln_fast(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, |v: &Vector| {
        let result: Vec<f64> = v.data().iter().map(|x| x.ln()).collect();
        Vector::new(result)
    })
}

// ============================================================================
// DSP Fast Path
// ============================================================================

#[wasm_bindgen]
pub fn fft_fast(handle: Handle) -> Result<Handle, JsValue> {
    // First, compute the FFT result
    let matrix = HANDLES.with(|h| {
        let handles = h.borrow();
        let value = handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

        match value {
            Value::Vector(vec) => {
                let spectrum = achronyme_dsp::fft_real(vec);

                // Convert to matrix [N x 2] (real, imaginary)
                let n = spectrum.len();
                let mut data = Vec::with_capacity(n * 2);
                for c in spectrum {
                    data.push(c.re);
                    data.push(c.im);
                }

                Matrix::new(n, 2, data)
                    .map_err(|e| JsValue::from_str(&e.to_string()))
            }
            _ => Err(JsValue::from_str("FFT requires vector"))
        }
    })?;

    // Now create the handle (borrow is dropped)
    HANDLES.with(|h| Ok(h.borrow_mut().create(Value::Matrix(matrix))))
}

#[wasm_bindgen]
pub fn fft_mag_fast(handle: Handle) -> Result<Handle, JsValue> {
    // First, compute the FFT magnitudes
    let result = HANDLES.with(|h| {
        let handles = h.borrow();
        let value = handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

        match value {
            Value::Vector(vec) => {
                // If given a vector, compute FFT then magnitudes
                let spectrum = achronyme_dsp::fft_real(vec);
                let magnitudes: Vec<f64> = spectrum.iter()
                    .map(|c| (c.re * c.re + c.im * c.im).sqrt())
                    .collect();

                Ok(Vector::new(magnitudes))
            }
            Value::Matrix(mat) => {
                // If given a matrix (FFT result), compute magnitudes from complex pairs
                if mat.cols != 2 {
                    return Err(JsValue::from_str("FFT matrix must have 2 columns (real, imag)"));
                }

                let data = &mat.data;
                let n = mat.rows;
                let mut magnitudes = Vec::with_capacity(n);

                for i in 0..n {
                    let re = data[i * 2];
                    let im = data[i * 2 + 1];
                    magnitudes.push((re * re + im * im).sqrt());
                }

                Ok(Vector::new(magnitudes))
            }
            _ => Err(JsValue::from_str("FFT_MAG requires vector or FFT matrix"))
        }
    })?;

    // Now create the handle (borrow is dropped)
    HANDLES.with(|h| Ok(h.borrow_mut().create(Value::Vector(result))))
}

#[wasm_bindgen]
pub fn linspace_fast(start: f64, end: f64, n: usize) -> Result<Handle, JsValue> {
    if n < 2 {
        return Err(JsValue::from_str("linspace requires n >= 2"));
    }

    let step = (end - start) / (n as f64 - 1.0);
    let mut result = Vec::with_capacity(n);

    for i in 0..n {
        result.push(start + step * i as f64);
    }

    let vector = Vector::new(result);
    Ok(HANDLES.with(|h| h.borrow_mut().create(Value::Vector(vector))))
}

// ============================================================================
// Linear Algebra Bindings (Compatible with C++ SDK)
// ============================================================================

#[wasm_bindgen]
pub struct LUResult {
    #[wasm_bindgen(readonly)]
    pub L: Handle,
    #[wasm_bindgen(readonly)]
    pub U: Handle,
    #[wasm_bindgen(readonly)]
    pub P: Handle,
}

#[wasm_bindgen]
pub fn lu_decomposition_js(handle: Handle) -> Result<LUResult, JsValue> {
    HANDLES.with(|h| {
        let handles = h.borrow();
        let value = handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

        match value {
            Value::Matrix(m) => {
                let (l, u, p) = achronyme_linalg::lu_decomposition(m)
                    .map_err(|e| JsValue::from_str(&e))?;

                let mut handles_mut = h.borrow_mut();
                let L = handles_mut.create(Value::Matrix(l));
                let U = handles_mut.create(Value::Matrix(u));

                // Convert permutation vector to permutation matrix
                let n = p.len();
                let mut p_data = vec![0.0; n * n];
                for (i, &pi) in p.iter().enumerate() {
                    p_data[i * n + pi] = 1.0;
                }
                let p_matrix = Matrix::new(n, n, p_data)
                    .map_err(|e| JsValue::from_str(&e.to_string()))?;
                let P = handles_mut.create(Value::Matrix(p_matrix));

                Ok(LUResult { L, U, P })
            }
            _ => Err(JsValue::from_str("LU decomposition requires matrix"))
        }
    })
}

// ============================================================================
// Helper Functions
// ============================================================================

fn format_value(value: &Value) -> String {
    match value {
        Value::Number(n) => n.to_string(),
        Value::Complex(c) => format!("{}+{}i", c.re, c.im),
        Value::Vector(v) => {
            let elements: Vec<String> = v.data().iter()
                .map(|x| format!("{:.6}", x))
                .collect();
            format!("[{}]", elements.join(", "))
        }
        Value::Matrix(m) => {
            let mut rows = Vec::new();
            for i in 0..m.rows {
                let mut row_elements = Vec::new();
                for j in 0..m.cols {
                    if let Ok(val) = m.get(i, j) {
                        row_elements.push(format!("{:.6}", val));
                    }
                }
                rows.push(format!("[{}]", row_elements.join(", ")));
            }
            format!("[{}]", rows.join(", "))
        }
        Value::Function(_) => "x => <function>".to_string(),
    }
}

// ============================================================================
// QR Decomposition
// ============================================================================

#[wasm_bindgen]
pub struct QRResult {
    #[wasm_bindgen(readonly)]
    pub Q: Handle,
    #[wasm_bindgen(readonly)]
    pub R: Handle,
}

#[wasm_bindgen]
pub fn qr_decomposition_js(handle: Handle) -> Result<QRResult, JsValue> {
    HANDLES.with(|h| {
        let handles = h.borrow();
        let value = handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

        match value {
            Value::Matrix(m) => {
                let (q, r) = achronyme_linalg::qr_decomposition(m)
                    .map_err(|e| JsValue::from_str(&e))?;

                let mut handles_mut = h.borrow_mut();
                let Q = handles_mut.create(Value::Matrix(q));
                let R = handles_mut.create(Value::Matrix(r));

                Ok(QRResult { Q, R })
            }
            _ => Err(JsValue::from_str("QR decomposition requires matrix"))
        }
    })
}

// ============================================================================
// Cholesky Decomposition
// ============================================================================

#[wasm_bindgen]
pub fn cholesky_decomposition_js(handle: Handle) -> Result<Handle, JsValue> {
    HANDLES.with(|h| {
        let handles = h.borrow();
        let value = handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

        match value {
            Value::Matrix(m) => {
                let l = achronyme_linalg::cholesky_decomposition(m)
                    .map_err(|e| JsValue::from_str(&e))?;

                Ok(h.borrow_mut().create(Value::Matrix(l)))
            }
            _ => Err(JsValue::from_str("Cholesky decomposition requires matrix"))
        }
    })
}

// ============================================================================
// SVD Decomposition
// ============================================================================

#[wasm_bindgen]
pub struct SVDResult {
    #[wasm_bindgen(readonly)]
    pub U: Handle,
    #[wasm_bindgen(readonly)]
    pub S: Handle,
    #[wasm_bindgen(readonly)]
    pub V: Handle,
}

#[wasm_bindgen]
pub fn svd_decomposition_js(handle: Handle) -> Result<SVDResult, JsValue> {
    HANDLES.with(|h| {
        let handles = h.borrow();
        let value = handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

        match value {
            Value::Matrix(m) => {
                let (u, s_vec, v) = achronyme_linalg::svd_decomposition(m)
                    .map_err(|e| JsValue::from_str(&format!("{}", e)))?;

                let mut handles_mut = h.borrow_mut();
                let U = handles_mut.create(Value::Matrix(u));
                let S = handles_mut.create(Value::Vector(Vector::new(s_vec)));
                let V = handles_mut.create(Value::Matrix(v));

                Ok(SVDResult { U, S, V })
            }
            _ => Err(JsValue::from_str("SVD requires matrix"))
        }
    })
}

// ============================================================================
// Eigenvalue Solvers
// ============================================================================

#[wasm_bindgen]
pub struct PowerIterationResult {
    #[wasm_bindgen(readonly)]
    pub eigenvalue: f64,
    #[wasm_bindgen(readonly)]
    pub eigenvector: Handle,
}

#[wasm_bindgen]
pub fn power_iteration_js(
    handle: Handle,
    max_iterations: usize,
    tolerance: f64
) -> Result<PowerIterationResult, JsValue> {
    HANDLES.with(|h| {
        let handles = h.borrow();
        let value = handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

        match value {
            Value::Matrix(m) => {
                let (eigenvalue, eigenvector_matrix) = achronyme_linalg::power_iteration(m, max_iterations, tolerance)
                    .map_err(|e| JsValue::from_str(&format!("{}", e)))?;

                let eigenvector_handle = h.borrow_mut().create(Value::Matrix(eigenvector_matrix));

                Ok(PowerIterationResult {
                    eigenvalue,
                    eigenvector: eigenvector_handle,
                })
            }
            _ => Err(JsValue::from_str("Power iteration requires matrix"))
        }
    })
}

#[wasm_bindgen]
pub fn qr_eigenvalues_js(
    handle: Handle,
    max_iterations: usize,
    tolerance: f64
) -> Result<Handle, JsValue> {
    HANDLES.with(|h| {
        let handles = h.borrow();
        let value = handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

        match value {
            Value::Matrix(m) => {
                let eigenvalues_vec = achronyme_linalg::qr_eigenvalues(m, max_iterations, tolerance)
                    .map_err(|e| JsValue::from_str(&format!("{}", e)))?;

                Ok(h.borrow_mut().create(Value::Vector(Vector::new(eigenvalues_vec))))
            }
            _ => Err(JsValue::from_str("QR eigenvalues requires matrix"))
        }
    })
}

#[wasm_bindgen]
pub struct EigenResult {
    #[wasm_bindgen(readonly)]
    pub eigenvalues: Handle,
    #[wasm_bindgen(readonly)]
    pub eigenvectors: Handle,
}

#[wasm_bindgen]
pub fn eigen_symmetric_js(
    handle: Handle,
    max_iterations: usize,
    tolerance: f64
) -> Result<EigenResult, JsValue> {
    HANDLES.with(|h| {
        let handles = h.borrow();
        let value = handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

        match value {
            Value::Matrix(m) => {
                let (eigenvalues_vec, eigenvectors) = achronyme_linalg::eigen_symmetric(m, max_iterations, tolerance)
                    .map_err(|e| JsValue::from_str(&format!("{}", e)))?;

                let mut handles_mut = h.borrow_mut();
                let eigenvalues_handle = handles_mut.create(Value::Vector(Vector::new(eigenvalues_vec)));
                let eigenvectors_handle = handles_mut.create(Value::Matrix(eigenvectors));

                Ok(EigenResult {
                    eigenvalues: eigenvalues_handle,
                    eigenvectors: eigenvectors_handle,
                })
            }
            _ => Err(JsValue::from_str("Eigen symmetric requires matrix"))
        }
    })
}

// ============================================================================
// Matrix Utilities
// ============================================================================

#[wasm_bindgen]
pub fn is_symmetric_js(handle: Handle, tolerance: f64) -> Result<bool, JsValue> {
    HANDLES.with(|h| {
        let handles = h.borrow();
        let value = handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

        match value {
            Value::Matrix(m) => {
                Ok(achronyme_linalg::is_symmetric(m, tolerance))
            }
            _ => Err(JsValue::from_str("is_symmetric requires matrix"))
        }
    })
}

#[wasm_bindgen]
pub fn is_positive_definite_js(handle: Handle) -> Result<bool, JsValue> {
    HANDLES.with(|h| {
        let handles = h.borrow();
        let value = handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

        match value {
            Value::Matrix(m) => {
                Ok(achronyme_linalg::is_positive_definite(m))
            }
            _ => Err(JsValue::from_str("is_positive_definite requires matrix"))
        }
    })
}

#[wasm_bindgen]
pub fn identity_js(n: usize) -> Result<Handle, JsValue> {
    let mut data = vec![0.0; n * n];
    for i in 0..n {
        data[i * n + i] = 1.0;
    }

    let matrix = Matrix::new(n, n, data)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(HANDLES.with(|h| h.borrow_mut().create(Value::Matrix(matrix))))
}
