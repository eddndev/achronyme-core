use wasm_bindgen::prelude::*;
use crate::state::{Handle, HANDLES};
use achronyme_types::value::Value;
use achronyme_types::complex_vector::ComplexVector;
use achronyme_types::vector::Vector;

// ============================================================================
// DSP Operations
// ============================================================================

#[wasm_bindgen(js_name = "dspFft")]
pub fn dsp_fft(handle: Handle) -> Result<Handle, JsValue> {
    HANDLES.with(|h| {
        let result_vector = {
            let handles = h.borrow();
            let value = handles.get(handle)
                .ok_or_else(|| JsValue::from_str("Invalid handle for dspFft"))?;

            match value {
                Value::Vector(v) => {
                    let real_vec = Value::to_real_vector(v)
                        .map_err(|e| JsValue::from_str(&e.to_string()))?;
                    let spectrum = achronyme_dsp::fft::fft_real(real_vec.data());
                    Ok(Value::from_complex_vector(ComplexVector::new(spectrum)))
                }
                _ => Err(JsValue::from_str("dspFft requires a vector handle")),
            }
        }?;

        Ok(h.borrow_mut().create(result_vector))
    })
}

#[wasm_bindgen(js_name = "dspFftMag")]
pub fn dsp_fft_mag(handle: Handle) -> Result<Handle, JsValue> {
    HANDLES.with(|h| {
        let result_vector = {
            let handles = h.borrow();
            let value = handles.get(handle)
                .ok_or_else(|| JsValue::from_str("Invalid handle for dspFftMag"))?;

            match value {
                Value::Vector(v) => {
                    let real_vec = Value::to_real_vector(v)
                        .map_err(|e| JsValue::from_str(&e.to_string()))?;
                    let spectrum = achronyme_dsp::fft::fft_real(real_vec.data());
                    let magnitudes: Vec<Value> = spectrum.iter()
                        .map(|c| Value::Number(c.norm()))
                        .collect();
                    Ok(Value::Vector(magnitudes))
                }
                _ => Err(JsValue::from_str("dspFftMag requires a vector handle")),
            }
        }?;

        Ok(h.borrow_mut().create(result_vector))
    })
}

/// Inverse Fast Fourier Transform
#[wasm_bindgen]
pub fn ifft(handle: Handle) -> Result<Handle, JsValue> {
    HANDLES.with(|h| {
        let result_value = {
            let handles = h.borrow();
            let value = handles.get(handle)
                .ok_or_else(|| JsValue::from_str("Invalid handle for ifft"))?;

            match value {
                Value::Vector(v) => {
                    let complex_vec = Value::to_complex_vector(v)
                        .map_err(|e| JsValue::from_str(&e.to_string()))?;
                    let signal = achronyme_dsp::fft::ifft_real(complex_vec.data());
                    Ok(Value::from_real_vector(Vector::new(signal)))
                }
                _ => Err(JsValue::from_str("ifft requires a complex vector handle")),
            }
        }?;

        Ok(h.borrow_mut().create(result_value))
    })
}

// ============================================================================
// DSP Window Functions
// ============================================================================

/// Hanning window
#[wasm_bindgen(js_name = hanningWindow)]
pub fn hanning_window(n: usize) -> Handle {
    let window = achronyme_dsp::windows::hanning_window(n);
    HANDLES.with(|h| h.borrow_mut().create(Value::from_real_vector(Vector::new(window))))
}

/// Hamming window
#[wasm_bindgen(js_name = hammingWindow)]
pub fn hamming_window(n: usize) -> Handle {
    let window = achronyme_dsp::windows::hamming_window(n);
    HANDLES.with(|h| h.borrow_mut().create(Value::from_real_vector(Vector::new(window))))
}

/// Blackman window
#[wasm_bindgen(js_name = blackmanWindow)]
pub fn blackman_window(n: usize) -> Handle {
    let window = achronyme_dsp::windows::blackman_window(n);
    HANDLES.with(|h| h.borrow_mut().create(Value::from_real_vector(Vector::new(window))))
}
