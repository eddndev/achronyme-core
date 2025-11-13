use crate::state::{Handle, HANDLES};
use achronyme_types::value::Value;
use wasm_bindgen::prelude::*;

// ============================================================================
// Helper Functions
// ============================================================================

pub fn format_value(value: &Value) -> String {
    match value {
        Value::Number(n) => n.to_string(),
        Value::Boolean(b) => b.to_string(),
        Value::String(s) => format!("\"{}\"", s),
        Value::Complex(c) => {
            if c.im >= 0.0 {
                format!("{}+{}i", c.re, c.im)
            } else {
                format!("{}{}i", c.re, c.im)
            }
        }
        Value::Vector(v) => {
            let elements: Vec<String> = v.iter()
                .map(|val| format_value(val))
                .collect();
            format!("[{}]", elements.join(", "))
        }
        Value::Tensor(t) => {
            // Format tensor based on rank
            match t.rank() {
                0 => format!("{}", t.data()[0]),  // Scalar
                1 => {
                    // Vector
                    let elements: Vec<String> = t.data().iter()
                        .map(|&x| format!("{:.6}", x))
                        .collect();
                    format!("[{}]", elements.join(", "))
                }
                2 => {
                    // Matrix
                    let rows = t.shape()[0];
                    let cols = t.shape()[1];
                    let mut row_strings = Vec::new();
                    for i in 0..rows {
                        let mut row_elements = Vec::new();
                        for j in 0..cols {
                            if let Ok(val) = t.get(&[i, j]) {
                                row_elements.push(format!("{:.6}", val));
                            }
                        }
                        row_strings.push(format!("[{}]", row_elements.join(", ")));
                    }
                    format!("[{}]", row_strings.join(", "))
                }
                _ => {
                    // Higher-order tensor
                    format!("Tensor(shape: {:?})", t.shape())
                }
            }
        }
        Value::ComplexTensor(ct) => {
            // Format complex tensor
            match ct.rank() {
                0 => {
                    let c = &ct.data()[0];
                    if c.im >= 0.0 {
                        format!("{}+{}i", c.re, c.im)
                    } else {
                        format!("{}{}i", c.re, c.im)
                    }
                }
                1 => {
                    // Complex vector
                    let elements: Vec<String> = ct.data().iter()
                        .map(|c| {
                            if c.im >= 0.0 {
                                format!("{}+{}i", c.re, c.im)
                            } else {
                                format!("{}{}i", c.re, c.im)
                            }
                        })
                        .collect();
                    format!("[{}]", elements.join(", "))
                }
                _ => {
                    // Higher-order complex tensor
                    format!("ComplexTensor(shape: {:?})", ct.shape())
                }
            }
        }
        Value::Record(map) => {
            let mut fields: Vec<String> = map.iter()
                .map(|(k, v)| format!("{}: {}", k, format_value(v)))
                .collect();
            fields.sort(); // Sort for consistent output
            format!("{{ {} }}", fields.join(", "))
        }
        Value::Edge { from, to, directed, properties } => {
            let arrow = if *directed { "->" } else { "<>" };
            if properties.is_empty() {
                format!("{} {} {}", from, arrow, to)
            } else {
                let props: Vec<String> = properties.iter()
                    .map(|(k, v)| format!("{}: {}", k, format_value(v)))
                    .collect();
                format!("{} {} {}: {{ {} }}", from, arrow, to, props.join(", "))
            }
        }
        Value::Function(_) => "x => <function>".to_string(),

        Value::MutableRef(rc) => {
            let inner = rc.borrow();
            format!("mut {}", format_value(&inner))
        }

        Value::TailCall(_) => {
            // TailCall should never be visible to user code - it's an internal marker
            "<internal:TailCall>".to_string()
        }
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
            Some(Value::Vector(v)) => {
                // Convert Vec<Value> to Vec<f64>
                v.iter()
                    .map(|val| match val {
                        Value::Number(n) => Ok(*n),
                        _ => Err(JsValue::from_str(&format!("{} requires numeric vector", op_name))),
                    })
                    .collect()
            }
            Some(Value::Number(n)) => Ok(vec![*n]),
            Some(_) => Err(JsValue::from_str(&format!("{} requires a vector or number handle", op_name))),
            None => Err(JsValue::from_str(&format!("Invalid handle for {}", op_name))),
        }
    })?;

    // Step 2: Perform calculation. No borrows are held.
    let result_data: Vec<Value> = data_copy.iter()
        .map(|&x| Value::Number(f(x)))
        .collect();

    // Step 3: Create a new handle. This is a mutable borrow.
    Ok(HANDLES.with(|h| {
        h.borrow_mut().create(Value::Vector(result_data))
    }))
}

/// Generic helper for safe binary operations on handles.
pub fn apply_binary_op<F>(handle1: Handle, handle2: Handle, f: F) -> Result<Handle, JsValue>
where
    F: Fn(&Value, &Value) -> Result<Value, JsValue>,
{
    // Step 1: Borrow handles and obtain references to values
    let result = HANDLES.with(|h| {
        let handles = h.borrow();
        let val1 = handles.get(handle1)
            .ok_or_else(|| JsValue::from_str("Handle 1 is invalid"))?;
        let val2 = handles.get(handle2)
            .ok_or_else(|| JsValue::from_str("Handle 2 is invalid"))?;

        // Execute the operation
        f(val1, val2)
    })?;

    // Step 2: Create new handle with the result
    Ok(HANDLES.with(|h| h.borrow_mut().create(result)))
}
