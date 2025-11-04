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
    // First, get value from HANDLES
    let value = HANDLES.with(|h| {
        let handles = h.borrow();
        handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))
            .map(|v| v.clone())
    })?;

    // Now bind to EVALUATOR (HANDLES borrow is dropped)
    EVALUATOR.with(|e| {
        e.borrow_mut().environment_mut().define(name.to_string(), value)
            .map_err(|err| JsValue::from_str(&err))
    })
}

#[wasm_bindgen]
pub fn releaseHandle(handle: Handle) {
    // Direct release - no auto-cleanup, only manual via dispose() or session.use()
    HANDLES.with(|h| {
        h.borrow_mut().release(handle);
    });
}

// REMOVED: getVectorData was unsafe - returned raw pointer while RefCell was borrowed
// This could cause "already borrowed" panics if JavaScript tried to use the pointer
// while other code was accessing the HandleManager.
// Use getVector() instead which safely copies the data.

// ============================================================================
// Fast Path Math Operations (Same API as C++)
// ============================================================================

fn apply_unary_op<F>(handle: Handle, f: F) -> Result<Handle, JsValue>
where
    F: Fn(&Vector) -> Vector,
{
    HANDLES.with(|h| {
        let mut handles = h.borrow_mut();
        // Clone the value to release the borrow on the HashMap's internal data
        let value = handles
            .get(handle)
            .cloned()
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

        let result_vec = match value {
            Value::Vector(vec) => f(&vec),
            Value::Number(n) => {
                let vec = Vector::new(vec![n]);
                f(&vec)
            }
            _ => return Err(JsValue::from_str("Operation requires vector or number")),
        };

        Ok(handles.create(Value::Vector(result_vec)))
    })
}

// Renamed from *_fast - all handle-based operations are fast path by definition
#[wasm_bindgen]
pub fn sin(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, |v: &Vector| {
        let result: Vec<f64> = v.data().iter().map(|x| x.sin()).collect();
        Vector::new(result)
    })
}

#[wasm_bindgen]
pub fn cos(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, |v: &Vector| {
        let result: Vec<f64> = v.data().iter().map(|x| x.cos()).collect();
        Vector::new(result)
    })
}

#[wasm_bindgen]
pub fn tan(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, |v: &Vector| {
        let result: Vec<f64> = v.data().iter().map(|x| x.tan()).collect();
        Vector::new(result)
    })
}

#[wasm_bindgen]
pub fn sqrt(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, |v: &Vector| {
        let result: Vec<f64> = v.data().iter().map(|x| x.sqrt()).collect();
        Vector::new(result)
    })
}

#[wasm_bindgen]
pub fn exp(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, |v: &Vector| {
        let result: Vec<f64> = v.data().iter().map(|x| x.exp()).collect();
        Vector::new(result)
    })
}

#[wasm_bindgen]
pub fn abs(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, |v: &Vector| {
        let result: Vec<f64> = v.data().iter().map(|x| x.abs()).collect();
        Vector::new(result)
    })
}

#[wasm_bindgen]
pub fn ln(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, |v: &Vector| {
        let result: Vec<f64> = v.data().iter().map(|x| x.ln()).collect();
        Vector::new(result)
    })
}

// ============================================================================
// DSP Operations
// ============================================================================

#[wasm_bindgen]
pub fn fft(handle: Handle) -> Result<Handle, JsValue> {
    HANDLES.with(|h| {
        // NO ? inside borrow scope
        let matrix = {
            let handles = h.borrow();
            match handles.get(handle) {
                Some(value) => {
                    match value {
                        Value::Vector(vec) => {
                            let spectrum = achronyme_dsp::fft_real(vec);
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
                }
                None => Err(JsValue::from_str("Invalid handle"))
            }
        }?;

        Ok(h.borrow_mut().create(Value::Matrix(matrix)))
    })
}

#[wasm_bindgen]
pub fn fft_mag(handle: Handle) -> Result<Handle, JsValue> {
    HANDLES.with(|h| {
        // NO ? inside borrow scope
        let result = {
            let handles = h.borrow();
            match handles.get(handle) {
                Some(value) => {
                    match value {
                        Value::Vector(vec) => {
                            let spectrum = achronyme_dsp::fft_real(vec);
                            let magnitudes: Vec<f64> = spectrum.iter()
                                .map(|c| (c.re * c.re + c.im * c.im).sqrt())
                                .collect();
                            Ok(Vector::new(magnitudes))
                        }
                        Value::Matrix(mat) => {
                            if mat.cols != 2 {
                                Err(JsValue::from_str("FFT matrix must have 2 columns (real, imag)"))
                            } else {
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
                        }
                        _ => Err(JsValue::from_str("FFT_MAG requires vector or FFT matrix"))
                    }
                }
                None => Err(JsValue::from_str("Invalid handle"))
            }
        }?;

        Ok(h.borrow_mut().create(Value::Vector(result)))
    })
}

#[wasm_bindgen]
pub fn linspace(start: f64, end: f64, n: usize) -> Result<Handle, JsValue> {
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

/// Inverse Fast Fourier Transform
#[wasm_bindgen]
pub fn ifft(handle: Handle) -> Result<Handle, JsValue> {
    use achronyme_types::complex::Complex;

    HANDLES.with(|h| {
        // NO ? inside borrow scope
        let result = {
            let handles = h.borrow();
            match handles.get(handle) {
                Some(value) => {
                    match value {
                        Value::Matrix(mat) => {
                            if mat.cols != 2 {
                                Err(JsValue::from_str("IFFT requires matrix with 2 columns (real, imag)"))
                            } else {
                                let n = mat.rows;
                                let mut spectrum = Vec::with_capacity(n);
                                for i in 0..n {
                                    let re = mat.data[i * 2];
                                    let im = mat.data[i * 2 + 1];
                                    spectrum.push(Complex::new(re, im));
                                }
                                let signal = achronyme_dsp::ifft_real(&spectrum);
                                Ok(signal)
                            }
                        }
                        _ => Err(JsValue::from_str("IFFT requires matrix"))
                    }
                }
                None => Err(JsValue::from_str("Invalid handle"))
            }
        }?;

        Ok(h.borrow_mut().create(Value::Vector(result)))
    })
}

// ============================================================================
// Vector Operations (Optimized WASM implementations)
// ============================================================================

/// Binary operation helper for two vectors
fn apply_binary_op<F>(handle1: Handle, handle2: Handle, f: F) -> Result<Handle, JsValue>
where
    F: Fn(&Vector, &Vector) -> Result<Vector, String>,
{
    HANDLES.with(|h| {
        let mut handles = h.borrow_mut();
        let v1 = handles
            .get(handle1)
            .cloned()
            .ok_or_else(|| JsValue::from_str("Invalid handle 1"))?;
        let v2 = handles
            .get(handle2)
            .cloned()
            .ok_or_else(|| JsValue::from_str("Invalid handle 2"))?;

        let result_vec = match (v1, v2) {
            (Value::Vector(vec1), Value::Vector(vec2)) => {
                f(&vec1, &vec2).map_err(|e| JsValue::from_str(&e))?
            }
            _ => return Err(JsValue::from_str("Binary operation requires two vectors")),
        };

        Ok(handles.create(Value::Vector(result_vec)))
    })
}

/// Vector addition: v1 + v2
#[wasm_bindgen]
pub fn vadd(handle1: Handle, handle2: Handle) -> Result<Handle, JsValue> {
    apply_binary_op(handle1, handle2, |v1, v2| {
        if v1.len() != v2.len() {
            return Err(format!("Vector length mismatch: {} vs {}", v1.len(), v2.len()));
        }

        let result: Vec<f64> = v1.data().iter()
            .zip(v2.data().iter())
            .map(|(a, b)| a + b)
            .collect();

        Ok(Vector::new(result))
    })
}

/// Vector subtraction: v1 - v2
#[wasm_bindgen]
pub fn vsub(handle1: Handle, handle2: Handle) -> Result<Handle, JsValue> {
    apply_binary_op(handle1, handle2, |v1, v2| {
        if v1.len() != v2.len() {
            return Err(format!("Vector length mismatch: {} vs {}", v1.len(), v2.len()));
        }

        let result: Vec<f64> = v1.data().iter()
            .zip(v2.data().iter())
            .map(|(a, b)| a - b)
            .collect();

        Ok(Vector::new(result))
    })
}

/// Element-wise vector multiplication: v1 .* v2
#[wasm_bindgen]
pub fn vmul(handle1: Handle, handle2: Handle) -> Result<Handle, JsValue> {
    apply_binary_op(handle1, handle2, |v1, v2| {
        if v1.len() != v2.len() {
            return Err(format!("Vector length mismatch: {} vs {}", v1.len(), v2.len()));
        }

        let result: Vec<f64> = v1.data().iter()
            .zip(v2.data().iter())
            .map(|(a, b)| a * b)
            .collect();

        Ok(Vector::new(result))
    })
}

/// Element-wise vector division: v1 ./ v2
#[wasm_bindgen]
pub fn vdiv(handle1: Handle, handle2: Handle) -> Result<Handle, JsValue> {
    apply_binary_op(handle1, handle2, |v1, v2| {
        if v1.len() != v2.len() {
            return Err(format!("Vector length mismatch: {} vs {}", v1.len(), v2.len()));
        }

        let result: Vec<f64> = v1.data().iter()
            .zip(v2.data().iter())
            .map(|(a, b)| a / b)
            .collect();

        Ok(Vector::new(result))
    })
}

/// Dot product: v1 · v2
#[wasm_bindgen]
pub fn dot(handle1: Handle, handle2: Handle) -> Result<f64, JsValue> {
    HANDLES.with(|h| {
        let handles = h.borrow();
        let v1 = handles.get(handle1)
            .ok_or_else(|| JsValue::from_str("Invalid handle 1"))?;
        let v2 = handles.get(handle2)
            .ok_or_else(|| JsValue::from_str("Invalid handle 2"))?;

        match (v1, v2) {
            (Value::Vector(vec1), Value::Vector(vec2)) => {
                if vec1.len() != vec2.len() {
                    return Err(JsValue::from_str(&format!(
                        "Vector length mismatch: {} vs {}", vec1.len(), vec2.len()
                    )));
                }

                let result: f64 = vec1.data().iter()
                    .zip(vec2.data().iter())
                    .map(|(a, b)| a * b)
                    .sum();

                Ok(result)
            }
            _ => Err(JsValue::from_str("Dot product requires two vectors"))
        }
    })
}

/// Vector L2 norm (Euclidean): ||v||₂
#[wasm_bindgen]
pub fn norm(handle: Handle) -> Result<f64, JsValue> {
    HANDLES.with(|h| {
        let handles = h.borrow();
        let value = handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

        match value {
            Value::Vector(vec) => {
                let sum_squares: f64 = vec.data().iter()
                    .map(|x| x * x)
                    .sum();
                Ok(sum_squares.sqrt())
            }
            _ => Err(JsValue::from_str("Norm requires vector"))
        }
    })
}

/// Vector L1 norm: ||v||₁
#[wasm_bindgen]
pub fn norm_l1(handle: Handle) -> Result<f64, JsValue> {
    HANDLES.with(|h| {
        let handles = h.borrow();
        let value = handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

        match value {
            Value::Vector(vec) => {
                let sum: f64 = vec.data().iter()
                    .map(|x| x.abs())
                    .sum();
                Ok(sum)
            }
            _ => Err(JsValue::from_str("Norm L1 requires vector"))
        }
    })
}

// ============================================================================
// Statistics Operations (Optimized WASM implementations)
// ============================================================================

/// Sum of vector elements
#[wasm_bindgen]
pub fn sum(handle: Handle) -> Result<f64, JsValue> {
    HANDLES.with(|h| {
        let handles = h.borrow();
        let value = handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

        match value {
            Value::Vector(vec) => {
                let result: f64 = vec.data().iter().sum();
                Ok(result)
            }
            _ => Err(JsValue::from_str("Sum requires vector"))
        }
    })
}

/// Mean (average) of vector elements
#[wasm_bindgen]
pub fn mean(handle: Handle) -> Result<f64, JsValue> {
    HANDLES.with(|h| {
        let handles = h.borrow();
        let value = handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

        match value {
            Value::Vector(vec) => {
                if vec.len() == 0 {
                    return Err(JsValue::from_str("Cannot compute mean of empty vector"));
                }

                let sum: f64 = vec.data().iter().sum();
                Ok(sum / vec.len() as f64)
            }
            _ => Err(JsValue::from_str("Mean requires vector"))
        }
    })
}

/// Standard deviation of vector elements
#[wasm_bindgen]
pub fn std(handle: Handle) -> Result<f64, JsValue> {
    HANDLES.with(|h| {
        let handles = h.borrow();
        let value = handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

        match value {
            Value::Vector(vec) => {
                if vec.len() == 0 {
                    return Err(JsValue::from_str("Cannot compute std of empty vector"));
                }

                // Compute mean
                let data = vec.data();
                let n = data.len() as f64;
                let mean: f64 = data.iter().sum::<f64>() / n;

                // Compute variance
                let variance: f64 = data.iter()
                    .map(|x| {
                        let diff = x - mean;
                        diff * diff
                    })
                    .sum::<f64>() / n;

                Ok(variance.sqrt())
            }
            _ => Err(JsValue::from_str("Std requires vector"))
        }
    })
}

/// Minimum value in vector
#[wasm_bindgen]
pub fn min(handle: Handle) -> Result<f64, JsValue> {
    HANDLES.with(|h| {
        let handles = h.borrow();
        let value = handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

        match value {
            Value::Vector(vec) => {
                vec.data().iter()
                    .fold(None, |min, &x| {
                        Some(match min {
                            None => x,
                            Some(m) => if x < m { x } else { m }
                        })
                    })
                    .ok_or_else(|| JsValue::from_str("Cannot compute min of empty vector"))
            }
            _ => Err(JsValue::from_str("Min requires vector"))
        }
    })
}

/// Maximum value in vector
#[wasm_bindgen]
pub fn max(handle: Handle) -> Result<f64, JsValue> {
    HANDLES.with(|h| {
        let handles = h.borrow();
        let value = handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

        match value {
            Value::Vector(vec) => {
                vec.data().iter()
                    .fold(None, |max, &x| {
                        Some(match max {
                            None => x,
                            Some(m) => if x > m { x } else { m }
                        })
                    })
                    .ok_or_else(|| JsValue::from_str("Cannot compute max of empty vector"))
            }
            _ => Err(JsValue::from_str("Max requires vector"))
        }
    })
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
        // First, borrow immutably to read and compute
        let (l, u, p_matrix) = {
            let handles = h.borrow();
            let value = handles.get(handle)
                .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

            match value {
                Value::Matrix(m) => {
                    let (l, u, p) = achronyme_linalg::lu_decomposition(m)
                        .map_err(|e| JsValue::from_str(&e))?;

                    // Convert permutation vector to permutation matrix
                    let n = p.len();
                    let mut p_data = vec![0.0; n * n];
                    for (i, &pi) in p.iter().enumerate() {
                        p_data[i * n + pi] = 1.0;
                    }
                    let p_matrix = Matrix::new(n, n, p_data)
                        .map_err(|e| JsValue::from_str(&e.to_string()))?;

                    Ok((l, u, p_matrix))
                }
                _ => Err(JsValue::from_str("LU decomposition requires matrix"))
            }
        }?; // Immutable borrow is dropped here

        // Now borrow mutably to create handles
        let mut handles_mut = h.borrow_mut();
        let L = handles_mut.create(Value::Matrix(l));
        let U = handles_mut.create(Value::Matrix(u));
        let P = handles_mut.create(Value::Matrix(p_matrix));

        Ok(LUResult { L, U, P })
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
        // First, borrow immutably to read and compute
        let (q, r) = {
            let handles = h.borrow();
            let value = handles.get(handle)
                .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

            match value {
                Value::Matrix(m) => {
                    achronyme_linalg::qr_decomposition(m)
                        .map_err(|e| JsValue::from_str(&e))
                }
                _ => Err(JsValue::from_str("QR decomposition requires matrix"))
            }
        }?; // Immutable borrow is dropped here

        // Now borrow mutably to create handles
        let mut handles_mut = h.borrow_mut();
        let Q = handles_mut.create(Value::Matrix(q));
        let R = handles_mut.create(Value::Matrix(r));

        Ok(QRResult { Q, R })
    })
}

// ============================================================================
// Cholesky Decomposition
// ============================================================================

#[wasm_bindgen]
pub fn cholesky_decomposition_js(handle: Handle) -> Result<Handle, JsValue> {
    HANDLES.with(|h| {
        // First, borrow immutably to read and compute
        let l = {
            let handles = h.borrow();
            let value = handles.get(handle)
                .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

            match value {
                Value::Matrix(m) => {
                    achronyme_linalg::cholesky_decomposition(m)
                        .map_err(|e| JsValue::from_str(&e))
                }
                _ => Err(JsValue::from_str("Cholesky decomposition requires matrix"))
            }
        }?; // Immutable borrow is dropped here

        // Now borrow mutably to create handle
        Ok(h.borrow_mut().create(Value::Matrix(l)))
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
        // First, borrow immutably to read and compute
        let (u, s_vec, v) = {
            let handles = h.borrow();
            let value = handles.get(handle)
                .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

            match value {
                Value::Matrix(m) => {
                    achronyme_linalg::svd_decomposition(m)
                        .map_err(|e| JsValue::from_str(&format!("{}", e)))
                }
                _ => Err(JsValue::from_str("SVD requires matrix"))
            }
        }?; // Immutable borrow is dropped here

        // Now borrow mutably to create handles
        let mut handles_mut = h.borrow_mut();
        let U = handles_mut.create(Value::Matrix(u));
        let S = handles_mut.create(Value::Vector(Vector::new(s_vec)));
        let V = handles_mut.create(Value::Matrix(v));

        Ok(SVDResult { U, S, V })
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
        // First, borrow immutably to read and compute
        let (eigenvalue, eigenvector_matrix) = {
            let handles = h.borrow();
            let value = handles.get(handle)
                .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

            match value {
                Value::Matrix(m) => {
                    achronyme_linalg::power_iteration(m, max_iterations, tolerance)
                        .map_err(|e| JsValue::from_str(&format!("{}", e)))
                }
                _ => Err(JsValue::from_str("Power iteration requires matrix"))
            }
        }?; // Immutable borrow is dropped here

        // Now borrow mutably to create handle
        let eigenvector_handle = h.borrow_mut().create(Value::Matrix(eigenvector_matrix));

        Ok(PowerIterationResult {
            eigenvalue,
            eigenvector: eigenvector_handle,
        })
    })
}

#[wasm_bindgen]
pub fn qr_eigenvalues_js(
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
                Value::Matrix(m) => {
                    achronyme_linalg::qr_eigenvalues(m, max_iterations, tolerance)
                        .map_err(|e| JsValue::from_str(&format!("{}", e)))
                }
                _ => Err(JsValue::from_str("QR eigenvalues requires matrix"))
            }
        }?; // Immutable borrow is dropped here

        // Now borrow mutably to create handle
        Ok(h.borrow_mut().create(Value::Vector(Vector::new(eigenvalues_vec))))
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
        // First, borrow immutably to read and compute
        let (eigenvalues_vec, eigenvectors) = {
            let handles = h.borrow();
            let value = handles.get(handle)
                .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

            match value {
                Value::Matrix(m) => {
                    achronyme_linalg::eigen_symmetric(m, max_iterations, tolerance)
                        .map_err(|e| JsValue::from_str(&format!("{}", e)))
                }
                _ => Err(JsValue::from_str("Eigen symmetric requires matrix"))
            }
        }?; // Immutable borrow is dropped here

        // Now borrow mutably to create handles
        let mut handles_mut = h.borrow_mut();
        let eigenvalues_handle = handles_mut.create(Value::Vector(Vector::new(eigenvalues_vec)));
        let eigenvectors_handle = handles_mut.create(Value::Matrix(eigenvectors));

        Ok(EigenResult {
            eigenvalues: eigenvalues_handle,
            eigenvectors: eigenvectors_handle,
        })
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
