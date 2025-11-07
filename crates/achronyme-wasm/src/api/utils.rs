use crate::state::{Handle, HANDLES};
use achronyme_types::value::Value;
use achronyme_types::vector::Vector;
use wasm_bindgen::prelude::*;

// ============================================================================
// Helper Functions
// ============================================================================

pub fn format_value(value: &Value) -> String {
    match value {
        Value::Number(n) => n.to_string(),
        Value::Boolean(b) => b.to_string(),
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

/// Generic helper for safe unary operations on handles.
pub fn apply_unary_op<F>(handle: Handle, op_name: &str, f: F) -> Result<Handle, JsValue>
where
    F: Fn(f64) -> f64,
{
    // Step 1: Read and copy the data out. This is an immutable borrow.
    let data_copy = HANDLES.with(|h| {
        let handles = h.borrow();
        match handles.get(handle) {
            Some(Value::Vector(v)) => Ok(v.data().to_vec()),
            Some(Value::Number(n)) => Ok(vec![*n]),
            Some(_) => Err(JsValue::from_str(&format!("{} requires a vector or number handle", op_name))),
            None => Err(JsValue::from_str(&format!("Invalid handle for {}", op_name))),
        }
    })?;

    // Step 2: Perform calculation. No borrows are held.
    let result_data: Vec<f64> = data_copy.iter().map(|&x| f(x)).collect();
    let result_vector = Vector::new(result_data);

    // Step 3: Create a new handle. This is a mutable borrow.
    Ok(HANDLES.with(|h| {
        h.borrow_mut().create(Value::Vector(result_vector))
    }))
}

/// Generic helper for safe binary operations on handles.
pub fn apply_binary_op<F>(handle1: Handle, handle2: Handle, f: F) -> Result<Handle, JsValue>
where
    F: Fn(&Vector, &Vector) -> Result<Vector, String>,
{
    // Step 1: Read and copy data for both handles
    let (data1, data2) = HANDLES.with(|h| {
        let handles = h.borrow();
        let v1 = match handles.get(handle1) {
            Some(Value::Vector(v)) => Ok(v.data().to_vec()),
            _ => Err(JsValue::from_str("Handle 1 is not a vector")),
        };
        let v2 = match handles.get(handle2) {
            Some(Value::Vector(v)) => Ok(v.data().to_vec()),
            _ => Err(JsValue::from_str("Handle 2 is not a vector")),
        };
        v1.and_then(|d1| v2.map(|d2| (d1, d2)))
    })?;

    // Step 2: Perform computation with no borrows held
    let vec1 = Vector::new(data1);
    let vec2 = Vector::new(data2);
    let result_vec = f(&vec1, &vec2).map_err(|e| JsValue::from_str(&e))?;

    // Step 3: Write result with a new mutable borrow
    Ok(HANDLES.with(|h| {
        h.borrow_mut().create(Value::Vector(result_vec))
    }))
}
