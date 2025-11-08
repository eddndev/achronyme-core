use crate::functions::FunctionRegistry;
use achronyme_types::value::Value;
use achronyme_types::complex_vector::ComplexVector;

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
        Value::Vector(vec) => {
            if !Value::is_numeric_vector(vec) {
                return Err("fft() requires a numeric vector".to_string());
            }
            let real_vec = Value::to_real_vector(vec).map_err(|e| e.to_string())?;
            let spectrum = achronyme_dsp::fft::fft_real(&real_vec);
            Ok(Value::from_complex_vector(ComplexVector::new(spectrum)))
        }
        _ => Err("fft() requires a vector".to_string()),
    }
}

fn ifft(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Vector(vec) => {
            if !Value::is_numeric_vector(vec) {
                return Err("ifft() requires a numeric vector".to_string());
            }
            let complex_vec = Value::to_complex_vector(vec).map_err(|e| e.to_string())?;
            let result = achronyme_dsp::fft::ifft_real(complex_vec.data());
            Ok(Value::from_real_vector(result))
        }
        _ => Err("ifft() requires a ComplexVector".to_string()),
    }
}

fn fft_mag(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Vector(vec) => {
            if !Value::is_numeric_vector(vec) {
                return Err("fft_mag() requires a numeric vector".to_string());
            }
            if vec.iter().any(|v| matches!(v, Value::Complex(_))) {
                let complex_vec = Value::to_complex_vector(vec).map_err(|e| e.to_string())?;
                let magnitudes: Vec<f64> = complex_vec.data().iter().map(|c| c.norm()).collect();
                let values: Vec<Value> = magnitudes.into_iter().map(Value::Number).collect();
                Ok(Value::Vector(values))
            } else {
                let real_vec = Value::to_real_vector(vec).map_err(|e| e.to_string())?;
                let spectrum = achronyme_dsp::fft::fft_real(&real_vec);
                let magnitudes: Vec<f64> = spectrum.iter().map(|c| c.norm()).collect();
                let values: Vec<Value> = magnitudes.into_iter().map(Value::Number).collect();
                Ok(Value::Vector(values))
            }
        }
        _ => Err("fft_mag() requires a vector or complex vector".to_string()),
    }
}

fn fft_phase(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Vector(vec) => {
            if !Value::is_numeric_vector(vec) {
                return Err("fft_phase() requires a numeric vector".to_string());
            }
            if vec.iter().any(|v| matches!(v, Value::Complex(_))) {
                let complex_vec = Value::to_complex_vector(vec).map_err(|e| e.to_string())?;
                let phases: Vec<f64> = complex_vec.data().iter().map(|c| c.arg()).collect();
                let values: Vec<Value> = phases.into_iter().map(Value::Number).collect();
                Ok(Value::Vector(values))
            } else {
                let real_vec = Value::to_real_vector(vec).map_err(|e| e.to_string())?;
                let spectrum = achronyme_dsp::fft::fft_real(&real_vec);
                let phases: Vec<f64> = spectrum.iter().map(|c| c.arg()).collect();
                let values: Vec<Value> = phases.into_iter().map(Value::Number).collect();
                Ok(Value::Vector(values))
            }
        }
        _ => Err("fft_phase() requires a vector or complex vector".to_string()),
    }
}

fn conv(args: &[Value]) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Vector(signal_vec), Value::Vector(kernel_vec)) => {
            if !Value::is_numeric_vector(signal_vec) || !Value::is_numeric_vector(kernel_vec) {
                return Err("conv() requires numeric vectors".to_string());
            }
            let signal = Value::to_real_vector(signal_vec).map_err(|e| e.to_string())?;
            let kernel = Value::to_real_vector(kernel_vec).map_err(|e| e.to_string())?;
            let result = achronyme_dsp::convolution::convolve(&signal, &kernel);
            Ok(Value::from_real_vector(result))
        }
        _ => Err("conv() requires two vectors".to_string()),
    }
}

fn conv_fft(args: &[Value]) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Vector(signal_vec), Value::Vector(kernel_vec)) => {
            if !Value::is_numeric_vector(signal_vec) || !Value::is_numeric_vector(kernel_vec) {
                return Err("conv_fft() requires numeric vectors".to_string());
            }
            let signal = Value::to_real_vector(signal_vec).map_err(|e| e.to_string())?;
            let kernel = Value::to_real_vector(kernel_vec).map_err(|e| e.to_string())?;
            let result = achronyme_dsp::convolution::convolve_fft(&signal, &kernel);
            Ok(Value::from_real_vector(result))
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
            let window = achronyme_dsp::windows::hanning_window(*n as usize);
            Ok(Value::from_real_vector(window))
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
            let window = achronyme_dsp::windows::hamming_window(*n as usize);
            Ok(Value::from_real_vector(window))
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
            let window = achronyme_dsp::windows::blackman_window(*n as usize);
            Ok(Value::from_real_vector(window))
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
            let window = achronyme_dsp::windows::rectangular_window(*n as usize);
            Ok(Value::from_real_vector(window))
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
            let result: Vec<Value> = (0..count)
                .map(|i| Value::Number(start + step * i as f64))
                .collect();
            Ok(Value::Vector(result))
        }
        _ => Err("linspace() requires three numbers (start, end, count)".to_string()),
    }
}
