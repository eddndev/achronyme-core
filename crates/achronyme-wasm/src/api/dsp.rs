use wasm_bindgen::prelude::*;
use crate::state::{Handle, HANDLES};
use achronyme_types::value::Value;
use achronyme_types::tensor::{RealTensor, ComplexTensor};

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
                Value::Tensor(t) => {
                    if !t.is_vector() {
                        return Err(JsValue::from_str("dspFft requires a vector (rank-1 tensor)"));
                    }
                    let spectrum = achronyme_dsp::fft::fft_real(t.data());
                    let result = ComplexTensor::vector(spectrum);
                    Ok(Value::ComplexTensor(result))
                }
                Value::Vector(v) => {
                    // Support generic vectors with numbers
                    let real_vec = Value::to_real_tensor(v)
                        .map_err(|e| JsValue::from_str(&e.to_string()))?;
                    let spectrum = achronyme_dsp::fft::fft_real(real_vec.data());
                    let result = ComplexTensor::vector(spectrum);
                    Ok(Value::ComplexTensor(result))
                }
                _ => Err(JsValue::from_str("dspFft requires a numeric vector")),
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
                Value::Tensor(t) => {
                    if !t.is_vector() {
                        return Err(JsValue::from_str("dspFftMag requires a vector (rank-1 tensor)"));
                    }
                    let spectrum = achronyme_dsp::fft::fft_real(t.data());
                    let magnitudes: Vec<f64> = spectrum.iter().map(|c| c.norm()).collect();
                    let result = RealTensor::vector(magnitudes);
                    Ok(Value::Tensor(result))
                }
                Value::Vector(v) => {
                    // Support generic vectors with numbers
                    let real_vec = Value::to_real_tensor(v)
                        .map_err(|e| JsValue::from_str(&e.to_string()))?;
                    let spectrum = achronyme_dsp::fft::fft_real(real_vec.data());
                    let magnitudes: Vec<f64> = spectrum.iter().map(|c| c.norm()).collect();
                    let result = RealTensor::vector(magnitudes);
                    Ok(Value::Tensor(result))
                }
                _ => Err(JsValue::from_str("dspFftMag requires a numeric vector")),
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
                Value::ComplexTensor(t) => {
                    if !t.is_vector() {
                        return Err(JsValue::from_str("ifft requires a vector (rank-1 tensor)"));
                    }
                    let signal = achronyme_dsp::fft::ifft_real(t.data());
                    let result = RealTensor::vector(signal);
                    Ok(Value::Tensor(result))
                }
                Value::Vector(v) => {
                    // Support generic vectors with complex numbers
                    let complex_vec = Value::to_complex_tensor(v)
                        .map_err(|e| JsValue::from_str(&e.to_string()))?;
                    let signal = achronyme_dsp::fft::ifft_real(complex_vec.data());
                    let result = RealTensor::vector(signal);
                    Ok(Value::Tensor(result))
                }
                _ => Err(JsValue::from_str("ifft requires a complex vector")),
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
    let tensor = RealTensor::vector(window);
    HANDLES.with(|h| h.borrow_mut().create(Value::Tensor(tensor)))
}

/// Hamming window
#[wasm_bindgen(js_name = hammingWindow)]
pub fn hamming_window(n: usize) -> Handle {
    let window = achronyme_dsp::windows::hamming_window(n);
    let tensor = RealTensor::vector(window);
    HANDLES.with(|h| h.borrow_mut().create(Value::Tensor(tensor)))
}

/// Blackman window
#[wasm_bindgen(js_name = blackmanWindow)]
pub fn blackman_window(n: usize) -> Handle {
    let window = achronyme_dsp::windows::blackman_window(n);
    let tensor = RealTensor::vector(window);
    HANDLES.with(|h| h.borrow_mut().create(Value::Tensor(tensor)))
}
