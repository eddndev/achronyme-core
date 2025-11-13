use crate::functions::FunctionRegistry;
use achronyme_types::value::Value;
use achronyme_types::Environment;

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

fn fft(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match &args[0] {
        // Tensor support (optimized path)
        Value::Tensor(t) => {
            if !t.is_vector() {
                return Err("fft() requires a rank-1 tensor (vector)".to_string());
            }
            let spectrum = achronyme_dsp::fft::fft_real(t.data());
            let result = achronyme_types::tensor::ComplexTensor::vector(spectrum);
            Ok(Value::ComplexTensor(result))
        }

        // Legacy Vector support (backward compatibility)
        Value::Vector(vec) => {
            if !Value::is_numeric_vector(vec) {
                return Err("fft() requires a numeric vector".to_string());
            }
            let tensor = Value::to_real_tensor(vec).map_err(|e| e.to_string())?;
            let spectrum = achronyme_dsp::fft::fft_real(tensor.data());
            let result = achronyme_types::tensor::ComplexTensor::vector(spectrum);
            Ok(Value::ComplexTensor(result))
        }

        _ => Err("fft() requires a vector or tensor".to_string()),
    }
}

fn ifft(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match &args[0] {
        // Tensor support (optimized path)
        Value::ComplexTensor(t) => {
            if !t.is_vector() {
                return Err("ifft() requires a rank-1 tensor (vector)".to_string());
            }
            let real_data = achronyme_dsp::fft::ifft_real(t.data());
            let result = achronyme_types::tensor::RealTensor::vector(real_data);
            Ok(Value::Tensor(result))
        }

        // Legacy Vector support (backward compatibility)
        Value::Vector(vec) => {
            if !Value::is_numeric_vector(vec) {
                return Err("ifft() requires a numeric vector".to_string());
            }
            let tensor = Value::to_complex_tensor(vec).map_err(|e| e.to_string())?;
            let real_data = achronyme_dsp::fft::ifft_real(tensor.data());
            let result = achronyme_types::tensor::RealTensor::vector(real_data);
            Ok(Value::Tensor(result))
        }

        _ => Err("ifft() requires a complex vector or complex tensor".to_string()),
    }
}

fn fft_mag(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match &args[0] {
        // Tensor support (optimized path) - Real input
        Value::Tensor(t) => {
            if !t.is_vector() {
                return Err("fft_mag() requires a rank-1 tensor (vector)".to_string());
            }
            let spectrum = achronyme_dsp::fft::fft_real(t.data());
            let magnitudes: Vec<f64> = spectrum.iter().map(|c| c.norm()).collect();
            let result = achronyme_types::tensor::RealTensor::vector(magnitudes);
            Ok(Value::Tensor(result))
        }

        // Tensor support (optimized path) - Complex input
        Value::ComplexTensor(t) => {
            if !t.is_vector() {
                return Err("fft_mag() requires a rank-1 tensor (vector)".to_string());
            }
            let magnitudes: Vec<f64> = t.data().iter().map(|c| c.norm()).collect();
            let result = achronyme_types::tensor::RealTensor::vector(magnitudes);
            Ok(Value::Tensor(result))
        }

        // Legacy Vector support (backward compatibility)
        Value::Vector(vec) => {
            if !Value::is_numeric_vector(vec) {
                return Err("fft_mag() requires a numeric vector".to_string());
            }
            if vec.iter().any(|v| matches!(v, Value::Complex(_))) {
                let tensor = Value::to_complex_tensor(vec).map_err(|e| e.to_string())?;
                let magnitudes: Vec<f64> = tensor.data().iter().map(|c| c.norm()).collect();
                let result = achronyme_types::tensor::RealTensor::vector(magnitudes);
                Ok(Value::Tensor(result))
            } else {
                let tensor = Value::to_real_tensor(vec).map_err(|e| e.to_string())?;
                let spectrum = achronyme_dsp::fft::fft_real(tensor.data());
                let magnitudes: Vec<f64> = spectrum.iter().map(|c| c.norm()).collect();
                let result = achronyme_types::tensor::RealTensor::vector(magnitudes);
                Ok(Value::Tensor(result))
            }
        }

        _ => Err("fft_mag() requires a vector, tensor, or complex tensor".to_string()),
    }
}

fn fft_phase(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match &args[0] {
        // Tensor support (optimized path) - Real input
        Value::Tensor(t) => {
            if !t.is_vector() {
                return Err("fft_phase() requires a rank-1 tensor (vector)".to_string());
            }
            let spectrum = achronyme_dsp::fft::fft_real(t.data());
            let phases: Vec<f64> = spectrum.iter().map(|c| c.arg()).collect();
            let result = achronyme_types::tensor::RealTensor::vector(phases);
            Ok(Value::Tensor(result))
        }

        // Tensor support (optimized path) - Complex input
        Value::ComplexTensor(t) => {
            if !t.is_vector() {
                return Err("fft_phase() requires a rank-1 tensor (vector)".to_string());
            }
            let phases: Vec<f64> = t.data().iter().map(|c| c.arg()).collect();
            let result = achronyme_types::tensor::RealTensor::vector(phases);
            Ok(Value::Tensor(result))
        }

        // Legacy Vector support (backward compatibility)
        Value::Vector(vec) => {
            if !Value::is_numeric_vector(vec) {
                return Err("fft_phase() requires a numeric vector".to_string());
            }
            if vec.iter().any(|v| matches!(v, Value::Complex(_))) {
                let tensor = Value::to_complex_tensor(vec).map_err(|e| e.to_string())?;
                let phases: Vec<f64> = tensor.data().iter().map(|c| c.arg()).collect();
                let result = achronyme_types::tensor::RealTensor::vector(phases);
                Ok(Value::Tensor(result))
            } else {
                let tensor = Value::to_real_tensor(vec).map_err(|e| e.to_string())?;
                let spectrum = achronyme_dsp::fft::fft_real(tensor.data());
                let phases: Vec<f64> = spectrum.iter().map(|c| c.arg()).collect();
                let result = achronyme_types::tensor::RealTensor::vector(phases);
                Ok(Value::Tensor(result))
            }
        }

        _ => Err("fft_phase() requires a vector, tensor, or complex tensor".to_string()),
    }
}

fn conv(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        // Tensor support (optimized path)
        (Value::Tensor(signal_t), Value::Tensor(kernel_t)) => {
            if !signal_t.is_vector() || !kernel_t.is_vector() {
                return Err("conv() requires rank-1 tensors (vectors)".to_string());
            }
            let result = achronyme_dsp::convolution::convolve(signal_t.data(), kernel_t.data());
            let tensor = achronyme_types::tensor::RealTensor::vector(result);
            Ok(Value::Tensor(tensor))
        }

        // Legacy Vector support (backward compatibility)
        (Value::Vector(signal_vec), Value::Vector(kernel_vec)) => {
            if !Value::is_numeric_vector(signal_vec) || !Value::is_numeric_vector(kernel_vec) {
                return Err("conv() requires numeric vectors".to_string());
            }
            let signal_t = Value::to_real_tensor(signal_vec).map_err(|e| e.to_string())?;
            let kernel_t = Value::to_real_tensor(kernel_vec).map_err(|e| e.to_string())?;
            let result = achronyme_dsp::convolution::convolve(signal_t.data(), kernel_t.data());
            let tensor = achronyme_types::tensor::RealTensor::vector(result);
            Ok(Value::Tensor(tensor))
        }

        _ => Err("conv() requires two vectors or tensors".to_string()),
    }
}

fn conv_fft(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        // Tensor support (optimized path)
        (Value::Tensor(signal_t), Value::Tensor(kernel_t)) => {
            if !signal_t.is_vector() || !kernel_t.is_vector() {
                return Err("conv_fft() requires rank-1 tensors (vectors)".to_string());
            }
            let result = achronyme_dsp::convolution::convolve_fft(signal_t.data(), kernel_t.data());
            let tensor = achronyme_types::tensor::RealTensor::vector(result);
            Ok(Value::Tensor(tensor))
        }

        // Legacy Vector support (backward compatibility)
        (Value::Vector(signal_vec), Value::Vector(kernel_vec)) => {
            if !Value::is_numeric_vector(signal_vec) || !Value::is_numeric_vector(kernel_vec) {
                return Err("conv_fft() requires numeric vectors".to_string());
            }
            let signal_t = Value::to_real_tensor(signal_vec).map_err(|e| e.to_string())?;
            let kernel_t = Value::to_real_tensor(kernel_vec).map_err(|e| e.to_string())?;
            let result = achronyme_dsp::convolution::convolve_fft(signal_t.data(), kernel_t.data());
            let tensor = achronyme_types::tensor::RealTensor::vector(result);
            Ok(Value::Tensor(tensor))
        }

        _ => Err("conv_fft() requires two vectors or tensors".to_string()),
    }
}

fn hanning(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match &args[0] {
        Value::Number(n) => {
            if *n < 0.0 || n.fract() != 0.0 {
                return Err("hanning() requires a non-negative integer".to_string());
            }
            let window = achronyme_dsp::windows::hanning_window(*n as usize);
            let tensor = achronyme_types::tensor::RealTensor::vector(window);
            Ok(Value::Tensor(tensor))
        }
        _ => Err("hanning() requires a number (window size)".to_string()),
    }
}

fn hamming(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match &args[0] {
        Value::Number(n) => {
            if *n < 0.0 || n.fract() != 0.0 {
                return Err("hamming() requires a non-negative integer".to_string());
            }
            let window = achronyme_dsp::windows::hamming_window(*n as usize);
            let tensor = achronyme_types::tensor::RealTensor::vector(window);
            Ok(Value::Tensor(tensor))
        }
        _ => Err("hamming() requires a number (window size)".to_string()),
    }
}

fn blackman(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match &args[0] {
        Value::Number(n) => {
            if *n < 0.0 || n.fract() != 0.0 {
                return Err("blackman() requires a non-negative integer".to_string());
            }
            let window = achronyme_dsp::windows::blackman_window(*n as usize);
            let tensor = achronyme_types::tensor::RealTensor::vector(window);
            Ok(Value::Tensor(tensor))
        }
        _ => Err("blackman() requires a number (window size)".to_string()),
    }
}

fn rectangular(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match &args[0] {
        Value::Number(n) => {
            if *n < 0.0 || n.fract() != 0.0 {
                return Err("rectangular() requires a non-negative integer".to_string());
            }
            let window = achronyme_dsp::windows::rectangular_window(*n as usize);
            let tensor = achronyme_types::tensor::RealTensor::vector(window);
            Ok(Value::Tensor(tensor))
        }
        _ => Err("rectangular() requires a number (window size)".to_string()),
    }
}

fn linspace(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match (&args[0], &args[1], &args[2]) {
        (Value::Number(start), Value::Number(end), Value::Number(n)) => {
            if *n < 2.0 || n.fract() != 0.0 {
                return Err("linspace() requires n >= 2 as integer".to_string());
            }
            let count = *n as usize;
            let step = (end - start) / (*n - 1.0);
            let data: Vec<f64> = (0..count)
                .map(|i| start + step * i as f64)
                .collect();
            let tensor = achronyme_types::tensor::RealTensor::vector(data);
            Ok(Value::Tensor(tensor))
        }
        _ => Err("linspace() requires three numbers (start, end, count)".to_string()),
    }
}
