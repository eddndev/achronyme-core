use wasm_bindgen::prelude::*;
use crate::state::{Handle, HANDLES};
use crate::api::utils::{apply_unary_op, apply_binary_op};
use achronyme_types::value::Value;

// ============================================================================
// Fast Path Math Operations (Same API as C++)
// ============================================================================

#[wasm_bindgen(js_name = "mathSin")]
pub fn math_sin(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, "mathSin", |x| x.sin())
}

#[wasm_bindgen(js_name = "mathCos")]
pub fn math_cos(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, "mathCos", |x| x.cos())
}

#[wasm_bindgen(js_name = "mathTan")]
pub fn math_tan(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, "mathTan", |x| x.tan())
}

#[wasm_bindgen(js_name = "mathExp")]
pub fn math_exp(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, "mathExp", |x| x.exp())
}

#[wasm_bindgen(js_name = "mathLn")]
pub fn math_ln(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, "mathLn", |x| x.ln())
}

#[wasm_bindgen(js_name = "mathAbs")]
pub fn math_abs(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, "mathAbs", |x| x.abs())
}

#[wasm_bindgen(js_name = "mathSqrt")]
pub fn math_sqrt(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, "mathSqrt", |x| x.sqrt())
}

// Inverse Trigonometric Functions
#[wasm_bindgen(js_name = "mathAsin")]
pub fn math_asin(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, "mathAsin", |x| x.asin())
}

#[wasm_bindgen(js_name = "mathAcos")]
pub fn math_acos(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, "mathAcos", |x| x.acos())
}

#[wasm_bindgen(js_name = "mathAtan")]
pub fn math_atan(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, "mathAtan", |x| x.atan())
}

// Hyperbolic Functions
#[wasm_bindgen(js_name = "mathSinh")]
pub fn math_sinh(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, "mathSinh", |x| x.sinh())
}

#[wasm_bindgen(js_name = "mathCosh")]
pub fn math_cosh(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, "mathCosh", |x| x.cosh())
}

#[wasm_bindgen(js_name = "mathTanh")]
pub fn math_tanh(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, "mathTanh", |x| x.tanh())
}

#[wasm_bindgen(js_name = "mathAsinh")]
pub fn math_asinh(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, "mathAsinh", |x| x.asinh())
}

#[wasm_bindgen(js_name = "mathAcosh")]
pub fn math_acosh(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, "mathAcosh", |x| x.acosh())
}

#[wasm_bindgen(js_name = "mathAtanh")]
pub fn math_atanh(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, "mathAtanh", |x| x.atanh())
}

// Rounding Functions
#[wasm_bindgen(js_name = "mathCeil")]
pub fn math_ceil(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, "mathCeil", |x| x.ceil())
}

#[wasm_bindgen(js_name = "mathFloor")]
pub fn math_floor(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, "mathFloor", |x| x.floor())
}

#[wasm_bindgen(js_name = "mathRound")]
pub fn math_round(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, "mathRound", |x| x.round())
}

#[wasm_bindgen(js_name = "mathTrunc")]
pub fn math_trunc(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, "mathTrunc", |x| x.trunc())
}

// Other Math Functions
#[wasm_bindgen(js_name = "mathCbrt")]
pub fn math_cbrt(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, "mathCbrt", |x| x.cbrt())
}

#[wasm_bindgen(js_name = "mathLog10")]
pub fn math_log10(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, "mathLog10", |x| x.log10())
}

#[wasm_bindgen]
pub fn linspace(start: f64, end: f64, n: usize) -> Result<Handle, JsValue> {
    if n < 2 {
        return Err(JsValue::from_str("linspace requires n >= 2"));
    }

    let step = (end - start) / (n as f64 - 1.0);
    let result: Vec<Value> = (0..n)
        .map(|i| Value::Number(start + step * i as f64))
        .collect();

    Ok(HANDLES.with(|h| h.borrow_mut().create(Value::Vector(result))))
}

// ============================================================================
// Vector Operations (Optimized WASM implementations)
// ============================================================================

/// Helper to extract Vec<f64> from Value::Vector(Vec<Value>)
fn get_f64_vector(value: &Value) -> Result<&Vec<Value>, JsValue> {
    if let Value::Vector(v) = value {
        if v.iter().all(|x| matches!(x, Value::Number(_))) {
            Ok(v)
        } else {
            Err(JsValue::from_str("Vector must contain only numbers for this operation"))
        }
    } else {
        Err(JsValue::from_str("Expected a vector"))
    }
}

/// Vector addition: v1 + v2
#[wasm_bindgen]
pub fn vadd(handle1: Handle, handle2: Handle) -> Result<Handle, JsValue> {
    apply_binary_op(handle1, handle2, |v1, v2| {
        let vec1 = get_f64_vector(v1)?;
        let vec2 = get_f64_vector(v2)?;

        if vec1.len() != vec2.len() {
            return Err(JsValue::from_str(&format!(
                "Vector length mismatch: {} vs {}", vec1.len(), vec2.len()
            )));
        }

        let result: Vec<Value> = vec1.iter()
            .zip(vec2.iter())
            .map(|(a, b)| {
                if let (Value::Number(n1), Value::Number(n2)) = (a, b) {
                    Value::Number(n1 + n2)
                } else {
                    // This case should ideally be caught by get_f64_vector
                    unreachable!()
                }
            })
            .collect();

        Ok(Value::Vector(result))
    })
}

/// Vector subtraction: v1 - v2
#[wasm_bindgen]
pub fn vsub(handle1: Handle, handle2: Handle) -> Result<Handle, JsValue> {
    apply_binary_op(handle1, handle2, |v1, v2| {
        let vec1 = get_f64_vector(v1)?;
        let vec2 = get_f64_vector(v2)?;

        if vec1.len() != vec2.len() {
            return Err(JsValue::from_str(&format!(
                "Vector length mismatch: {} vs {}", vec1.len(), vec2.len()
            )));
        }

        let result: Vec<Value> = vec1.iter()
            .zip(vec2.iter())
            .map(|(a, b)| {
                if let (Value::Number(n1), Value::Number(n2)) = (a, b) {
                    Value::Number(n1 - n2)
                } else {
                    unreachable!()
                }
            })
            .collect();

        Ok(Value::Vector(result))
    })
}

/// Element-wise vector multiplication: v1 .* v2
#[wasm_bindgen]
pub fn vmul(handle1: Handle, handle2: Handle) -> Result<Handle, JsValue> {
    apply_binary_op(handle1, handle2, |v1, v2| {
        let vec1 = get_f64_vector(v1)?;
        let vec2 = get_f64_vector(v2)?;

        if vec1.len() != vec2.len() {
            return Err(JsValue::from_str(&format!(
                "Vector length mismatch: {} vs {}", vec1.len(), vec2.len()
            )));
        }

        let result: Vec<Value> = vec1.iter()
            .zip(vec2.iter())
            .map(|(a, b)| {
                if let (Value::Number(n1), Value::Number(n2)) = (a, b) {
                    Value::Number(n1 * n2)
                } else {
                    unreachable!()
                }
            })
            .collect();

        Ok(Value::Vector(result))
    })
}

/// Element-wise vector division: v1 ./ v2
#[wasm_bindgen]
pub fn vdiv(handle1: Handle, handle2: Handle) -> Result<Handle, JsValue> {
    apply_binary_op(handle1, handle2, |v1, v2| {
        let vec1 = get_f64_vector(v1)?;
        let vec2 = get_f64_vector(v2)?;

        if vec1.len() != vec2.len() {
            return Err(JsValue::from_str(&format!(
                "Vector length mismatch: {} vs {}", vec1.len(), vec2.len()
            )));
        }

        let result: Vec<Value> = vec1.iter()
            .zip(vec2.iter())
            .map(|(a, b)| {
                if let (Value::Number(n1), Value::Number(n2)) = (a, b) {
                    Value::Number(n1 / n2)
                } else {
                    unreachable!()
                }
            })
            .collect();

        Ok(Value::Vector(result))
    })
}

/// Dot product: v1 · v2
#[wasm_bindgen]
pub fn dot(handle1: Handle, handle2: Handle) -> Result<f64, JsValue> {
    HANDLES.with(|h| {
        let handles = h.borrow();
        let v1_val = handles.get(handle1)
            .ok_or_else(|| JsValue::from_str("Invalid handle 1"))?;
        let v2_val = handles.get(handle2)
            .ok_or_else(|| JsValue::from_str("Invalid handle 2"))?;

        let vec1 = get_f64_vector(v1_val)?;
        let vec2 = get_f64_vector(v2_val)?;

        if vec1.len() != vec2.len() {
            return Err(JsValue::from_str(&format!(
                "Vector length mismatch: {} vs {}", vec1.len(), vec2.len()
            )));
        }

        let result: f64 = vec1.iter()
            .zip(vec2.iter())
            .map(|(a, b)| {
                if let (Value::Number(n1), Value::Number(n2)) = (a, b) {
                    n1 * n2
                } else {
                    unreachable!()
                }
            })
            .sum();

        Ok(result)
    })
}
