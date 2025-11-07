use crate::functions::FunctionRegistry;
use achronyme_types::complex::Complex as AchronymeComplex;
use achronyme_types::complex_vector::ComplexVector;
use achronyme_types::value::Value;
use achronyme_types::vector::Vector;

pub fn register_functions(registry: &mut FunctionRegistry) {
    // FFT functions
    registry.register("fft", fft, 1);
    registry.register("ifft", ifft, 1);
    registry.register("fft_mag", fft_mag, 1);
    registry.register("fft_phase", fft_phase, 1);

    // Convolution
    registry.register("conv", conv, 2);
    registry.register("conv_fft", conv_fft, 2);

    // Window functions
    registry.register("hanning", hanning, 1);
    registry.register("hamming", hamming, 1);
    registry.register("blackman", blackman, 1);
    registry.register("rectangular", rectangular, 1);

    // Utilities
    registry.register("linspace", linspace, 3);
}

// Implementations

fn fft(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Vector(v) => {
            let spectrum = achronyme_dsp::fft_real(v);
            let complex_vector = ComplexVector::new(spectrum);
            Ok(Value::ComplexVector(complex_vector))
        }
        _ => Err("fft() requires a vector".to_string()),
    }
}

fn ifft(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::ComplexVector(cv) => {
            let spectrum: Vec<AchronymeComplex> = cv.data().to_vec();
            let result = achronyme_dsp::ifft_real(&spectrum);
            Ok(Value::Vector(result))
        }
        _ => Err("ifft() requires a ComplexVector".to_string()),
    }
}

fn fft_mag(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Vector(v) => {
            let spectrum = achronyme_dsp::fft_real(v);
            let magnitudes: Vec<f64> = spectrum.iter().map(|c| c.magnitude()).collect();
            Ok(Value::Vector(Vector::new(magnitudes)))
        }
        _ => Err("fft_mag() requires a vector".to_string()),
    }
}

fn fft_phase(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Vector(v) => {
            let spectrum = achronyme_dsp::fft_real(v);
            let phases: Vec<f64> = spectrum.iter().map(|c| c.im.atan2(c.re)).collect();
            Ok(Value::Vector(Vector::new(phases)))
        }
        _ => Err("fft_phase() requires a vector".to_string()),
    }
}

fn conv(args: &[Value]) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Vector(signal), Value::Vector(kernel)) => {
            let result = achronyme_dsp::convolve(signal, kernel);
            Ok(Value::Vector(result))
        }
        _ => Err("conv() requires two vectors".to_string()),
    }
}

fn conv_fft(args: &[Value]) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Vector(signal), Value::Vector(kernel)) => {
            let result = achronyme_dsp::convolve_fft(signal, kernel);
            Ok(Value::Vector(result))
        }
        _ => Err("conv_fft() requires two vectors".to_string()),
    }
}

fn hanning(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Number(n) => {
            if *n < 0.0 || n.fract() != 0.0 {
                return Err("hanning() requires a non-negative integer".to_string());
            }
            let window = achronyme_dsp::hanning_window(*n as usize);
            Ok(Value::Vector(window))
        }
        _ => Err("hanning() requires a number (window size)".to_string()),
    }
}

fn hamming(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Number(n) => {
            if *n < 0.0 || n.fract() != 0.0 {
                return Err("hamming() requires a non-negative integer".to_string());
            }
            let window = achronyme_dsp::hamming_window(*n as usize);
            Ok(Value::Vector(window))
        }
        _ => Err("hamming() requires a number (window size)".to_string()),
    }
}

fn blackman(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Number(n) => {
            if *n < 0.0 || n.fract() != 0.0 {
                return Err("blackman() requires a non-negative integer".to_string());
            }
            let window = achronyme_dsp::blackman_window(*n as usize);
            Ok(Value::Vector(window))
        }
        _ => Err("blackman() requires a number (window size)".to_string()),
    }
}

fn rectangular(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Number(n) => {
            if *n < 0.0 || n.fract() != 0.0 {
                return Err("rectangular() requires a non-negative integer".to_string());
            }
            let window = achronyme_dsp::rectangular_window(*n as usize);
            Ok(Value::Vector(window))
        }
        _ => Err("rectangular() requires a number (window size)".to_string()),
    }
}

fn linspace(args: &[Value]) -> Result<Value, String> {
    match (&args[0], &args[1], &args[2]) {
        (Value::Number(start), Value::Number(end), Value::Number(n)) => {
            if *n < 2.0 || n.fract() != 0.0 {
                return Err("linspace() requires n >= 2 as integer".to_string());
            }
            let count = *n as usize;
            let step = (end - start) / (*n - 1.0);
            let mut result = Vec::with_capacity(count);
            for i in 0..count {
                result.push(start + step * i as f64);
            }
            Ok(Value::Vector(Vector::new(result)))
        }
        _ => Err("linspace() requires three numbers (start, end, count)".to_string()),
    }
}
