use wasm_bindgen::prelude::*;
use crate::state::{Handle, HANDLES};
use achronyme_types::value::Value;

// ============================================================================
// Statistics Operations (Optimized WASM implementations)
// ============================================================================

/// Helper: Extract f64 values from a generic vector
fn extract_f64_vec(vec: &[Value]) -> Result<Vec<f64>, JsValue> {
    vec.iter()
        .map(|val| match val {
            Value::Number(n) => Ok(*n),
            _ => Err(JsValue::from_str("Vector must contain only numbers")),
        })
        .collect()
}

/// Sum of vector elements
#[wasm_bindgen]
pub fn sum(handle: Handle) -> Result<f64, JsValue> {
    HANDLES.with(|h| {
        let handles = h.borrow();
        let value = handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

        match value {
            Value::Vector(vec) => {
                let data = extract_f64_vec(vec)?;
                let result: f64 = data.iter().sum();
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

                let data = extract_f64_vec(vec)?;
                let sum: f64 = data.iter().sum();
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
                let data = extract_f64_vec(vec)?;
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
                let data = extract_f64_vec(vec)?;
                data.iter()
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
                let data = extract_f64_vec(vec)?;
                data.iter()
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

/// Vector L2 norm (Euclidean): ||v||₂
#[wasm_bindgen]
pub fn norm(handle: Handle) -> Result<f64, JsValue> {
    HANDLES.with(|h| {
        let handles = h.borrow();
        let value = handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

        match value {
            Value::Vector(vec) => {
                let data = extract_f64_vec(vec)?;
                let sum_squares: f64 = data.iter()
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
                let data = extract_f64_vec(vec)?;
                let sum: f64 = data.iter()
                    .map(|x| x.abs())
                    .sum();
                Ok(sum)
            }
            _ => Err(JsValue::from_str("Norm L1 requires vector"))
        }
    })
}
