use wasm_bindgen::prelude::*;
use crate::state::{HANDLES, Handle};
use achronyme_types::value::Value;
use achronyme_types::vector::Vector;
use achronyme_types::complex::Complex;
use achronyme_types::complex_vector::ComplexVector;
use achronyme_types::matrix::Matrix;

// ============================================================================
// Handle-Based API (Fast Path - Same as C++)
// ============================================================================

#[wasm_bindgen(js_name = createVectorFromBuffer)]
pub fn create_vector_from_buffer(data_ptr: *const f64, len: usize) -> Handle {
    unsafe {
        let data = std::slice::from_raw_parts(data_ptr, len).to_vec();
        let vector = Vector::new(data);
        HANDLES.with(|h| h.borrow_mut().create(Value::Vector(vector)))
    }
}

/// Create vector from JavaScript array (easier for testing)
#[wasm_bindgen(js_name = createVector)]
pub fn create_vector(data: Vec<f64>) -> Handle {
    let vector = Vector::new(data);
    HANDLES.with(|h| h.borrow_mut().create(Value::Vector(vector)))
}

/// Get vector data from handle (for verification/extraction)
#[wasm_bindgen(js_name = getVector)]
pub fn get_vector(handle: Handle) -> Result<Vec<f64>, JsValue> {
    HANDLES.with(|handles| {
        let h = handles.borrow();
        match h.get(handle) {
            Some(Value::Vector(v)) => Ok(v.data().to_vec()),
            Some(Value::Matrix(m)) => {
                // También permitir obtener datos de matrices (aplanados)
                Ok(m.data.clone())
            }
            Some(_) => Err(JsValue::from_str("Handle does not reference a vector or matrix")),
            None => Err(JsValue::from_str("Invalid handle")),
        }
    })
}

/// Get matrix data and dimensions from handle
#[wasm_bindgen(js_name = getMatrix)]
pub fn get_matrix(handle: Handle) -> Result<JsValue, JsValue> {
    HANDLES.with(|handles| {
        let h = handles.borrow();
        match h.get(handle) {
            Some(Value::Matrix(m)) => {
                let data = m.data.clone();
                let rows = m.rows;
                let cols = m.cols;

                // Return as JavaScript object
                let obj = js_sys::Object::new();
                js_sys::Reflect::set(&obj, &"data".into(), &serde_wasm_bindgen::to_value(&data)?)?;
                js_sys::Reflect::set(&obj, &"rows".into(), &JsValue::from_f64(rows as f64))?;
                js_sys::Reflect::set(&obj, &"cols".into(), &JsValue::from_f64(cols as f64))?;

                Ok(obj.into())
            }
            Some(_) => Err(JsValue::from_str("Handle does not reference a matrix")),
            None => Err(JsValue::from_str("Invalid handle")),
        }
    })
}

/// Create matrix from JavaScript array (row-major order)
#[wasm_bindgen(js_name = createMatrix)]
pub fn create_matrix(data: Vec<f64>, rows: usize, cols: usize) -> Result<Handle, JsValue> {
    if data.len() != rows * cols {
        return Err(JsValue::from_str(&format!(
            "Data length {} does not match dimensions {}x{}",
            data.len(), rows, cols
        )));
    }
    let matrix = Matrix::new(rows, cols, data)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(HANDLES.with(|h| h.borrow_mut().create(Value::Matrix(matrix))))
}

#[wasm_bindgen(js_name = createMatrixFromBuffer)]
pub fn create_matrix_from_buffer(data_ptr: *const f64, rows: usize, cols: usize) -> Result<Handle, JsValue> {
    unsafe {
        let len = rows * cols;
        let data = std::slice::from_raw_parts(data_ptr, len).to_vec();
        let matrix = Matrix::new(rows, cols, data)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(HANDLES.with(|h| h.borrow_mut().create(Value::Matrix(matrix))))
    }
}

#[wasm_bindgen(js_name = bindVariableToHandle)]
pub fn bind_variable_to_handle(name: &str, handle: Handle) -> Result<(), JsValue> {
    // First, get value from HANDLES
    let value = HANDLES.with(|h| {
        let handles = h.borrow();
        handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))
            .map(|v| v.clone())
    })?;

    // Now bind to EVALUATOR (HANDLES borrow is dropped)
    use crate::state::EVALUATOR;
    EVALUATOR.with(|e| {
        e.borrow_mut().environment_mut().define(name.to_string(), value)
            .map_err(|err| JsValue::from_str(&err))
    })
}

#[wasm_bindgen(js_name = releaseHandle)]
pub fn release_handle(handle: Handle) {
    // Direct release - no auto-cleanup, only manual via dispose() or session.use()
    HANDLES.with(|h| {
        h.borrow_mut().release(handle);
    });
}

// ============================================================================
// Complex Vector API
// ============================================================================

/// Create complex vector from interleaved real/imaginary components
/// data format: [re0, im0, re1, im1, ...]
#[wasm_bindgen(js_name = createComplexVector)]
pub fn create_complex_vector(data: Vec<f64>) -> Result<Handle, JsValue> {
    if data.len() % 2 != 0 {
        return Err(JsValue::from_str("Complex vector data must have even length (re, im pairs)"));
    }

    let complex_data: Vec<Complex> = data
        .chunks(2)
        .map(|chunk| Complex::new(chunk[0], chunk[1]))
        .collect();

    let vector = ComplexVector::new(complex_data);
    Ok(HANDLES.with(|h| h.borrow_mut().create(Value::ComplexVector(vector))))
}

/// Get complex vector data as interleaved real/imaginary components
/// Returns format: [re0, im0, re1, im1, ...]
#[wasm_bindgen(js_name = getComplexVector)]
pub fn get_complex_vector(handle: Handle) -> Result<Vec<f64>, JsValue> {
    HANDLES.with(|handles| {
        let h = handles.borrow();
        match h.get(handle) {
            Some(Value::ComplexVector(cv)) => {
                let data: Vec<f64> = cv.data()
                    .iter()
                    .flat_map(|c| vec![c.re, c.im])
                    .collect();
                Ok(data)
            }
            Some(_) => Err(JsValue::from_str("Handle does not reference a complex vector")),
            None => Err(JsValue::from_str("Invalid handle")),
        }
    })
}
