use wasm_bindgen::prelude::*;
use crate::state::{Handle, HANDLES};
use achronyme_types::value::Value;
use achronyme_types::vector::Vector;
use achronyme_types::matrix::Matrix;
use achronyme_types::complex::Complex;

// ============================================================================
// DSP Operations
// ============================================================================

#[wasm_bindgen(js_name = "dspFft")]
pub fn dsp_fft(handle: Handle) -> Result<Handle, JsValue> {
    HANDLES.with(|h| {
        let result_matrix = {
            let handles = h.borrow();
            let value = handles.get(handle)
                .ok_or_else(|| JsValue::from_str("Invalid handle for dspFft"))?;

            match value {
                Value::Vector(v) => {
                    let spectrum = achronyme_dsp::fft_real(v);
                    let n = spectrum.len();
                    let mut data = Vec::with_capacity(n * 2);
                    for c in spectrum {
                        data.push(c.re);
                        data.push(c.im);
                    }
                    Matrix::new(n, 2, data).map_err(|e| JsValue::from_str(&e.to_string()))
                }
                _ => Err(JsValue::from_str("dspFft requires a vector handle")),
            }
        }?;

        Ok(h.borrow_mut().create(Value::Matrix(result_matrix)))
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
                    let spectrum = achronyme_dsp::fft_real(v);
                    let magnitudes: Vec<f64> = spectrum.iter().map(|c| c.magnitude()).collect();
                    Ok(Vector::new(magnitudes))
                }
                _ => Err(JsValue::from_str("dspFftMag requires a vector handle")),
            }
        }?;

        Ok(h.borrow_mut().create(Value::Vector(result_vector)))
    })
}

/// Inverse Fast Fourier Transform
#[wasm_bindgen]
pub fn ifft(handle: Handle) -> Result<Handle, JsValue> {
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
// DSP Window Functions
// ============================================================================

/// Hanning window
#[wasm_bindgen(js_name = hanningWindow)]
pub fn hanning_window(n: usize) -> Handle {
    let window = achronyme_dsp::hanning_window(n);
    HANDLES.with(|h| h.borrow_mut().create(Value::Vector(window)))
}

/// Hamming window
#[wasm_bindgen(js_name = hammingWindow)]
pub fn hamming_window(n: usize) -> Handle {
    let window = achronyme_dsp::hamming_window(n);
    HANDLES.with(|h| h.borrow_mut().create(Value::Vector(window)))
}

/// Blackman window
#[wasm_bindgen(js_name = blackmanWindow)]
pub fn blackman_window(n: usize) -> Handle {
    let window = achronyme_dsp::blackman_window(n);
    HANDLES.with(|h| h.borrow_mut().create(Value::Vector(window)))
}
