use achronyme_types::value::Value;
use achronyme_types::vector::Vector;
use achronyme_types::matrix::Matrix;
use achronyme_types::complex::Complex as AchronymeComplex;
use std::collections::HashMap;

/// Type for built-in function implementations
pub type BuiltinFunction = fn(&[Value]) -> Result<Value, String>;

/// Helper macro for unary functions that work on both scalars and vectors
macro_rules! unary_math_fn {
    ($name:expr, $f:expr, $arg:expr) => {
        match $arg {
            Value::Number(x) => Ok(Value::Number($f(*x))),
            Value::Vector(v) => {
                let result: Vec<f64> = v.data().iter().map(|&x| $f(x)).collect();
                Ok(Value::Vector(Vector::new(result)))
            }
            _ => Err(format!("{}() requires a number or vector", $name)),
        }
    };
}

/// Registry for built-in mathematical functions
///
/// # Example
/// ```
/// use achronyme_eval::functions::FunctionRegistry;
/// use achronyme_types::Value;
///
/// let registry = FunctionRegistry::new();
/// let args = vec![Value::Number(std::f64::consts::PI / 2.0)];
/// let result = registry.call("sin", &args).unwrap();
/// ```
#[derive(Clone)]
pub struct FunctionRegistry {
    functions: HashMap<String, (BuiltinFunction, i32)>, // (function, arity) -1 = variadic
}

impl FunctionRegistry {
    /// Create a new function registry with all standard functions
    pub fn new() -> Self {
        let mut registry = Self {
            functions: HashMap::new(),
        };

        // Register all standard mathematical functions
        registry.register_math_functions();

        registry
    }

    /// Register all standard math functions
    fn register_math_functions(&mut self) {
        // Trigonometric functions
        self.register("sin", sin, 1);
        self.register("cos", cos, 1);
        self.register("tan", tan, 1);
        self.register("asin", asin, 1);
        self.register("acos", acos, 1);
        self.register("atan", atan, 1);
        self.register("atan2", atan2, 2);

        // Hyperbolic functions
        self.register("sinh", sinh, 1);
        self.register("cosh", cosh, 1);
        self.register("tanh", tanh, 1);

        // Exponential and logarithmic
        self.register("exp", exp, 1);
        self.register("ln", ln, 1);
        self.register("log", ln, 1); // alias
        self.register("log10", log10, 1);
        self.register("log2", log2, 1);

        // Power and roots
        self.register("sqrt", sqrt, 1);
        self.register("cbrt", cbrt, 1);
        self.register("pow", pow, 2);

        // Rounding
        self.register("floor", floor, 1);
        self.register("ceil", ceil, 1);
        self.register("round", round, 1);
        self.register("trunc", trunc, 1);
        self.register("abs", abs, 1);
        self.register("sign", sign, 1);

        // Angle conversions
        self.register("deg", deg, 1);  // radians to degrees
        self.register("rad", rad, 1);  // degrees to radians

        // Min/Max (variadic)
        self.register("min", min, -1);
        self.register("max", max, -1);

        // Complex number functions
        self.register("complex", complex, 2);
        self.register("real", real, 1);
        self.register("imag", imag, 1);
        self.register("conj", conj, 1);
        self.register("arg", arg, 1);

        // Vector operations
        self.register("dot", dot, 2);
        self.register("cross", cross, 2);
        self.register("norm", norm, 1);
        self.register("normalize", normalize, 1);

        // Statistical functions
        self.register("sum", sum, 1);
        self.register("mean", mean, 1);
        self.register("std", std, 1);

        // DSP: FFT functions
        self.register("fft", fft, 1);
        self.register("ifft", ifft, 1);
        self.register("fft_mag", fft_mag, 1);
        self.register("fft_phase", fft_phase, 1);

        // DSP: Convolution
        self.register("conv", conv, 2);
        self.register("conv_fft", conv_fft, 2);

        // DSP: Window functions
        self.register("hanning", hanning, 1);
        self.register("hamming", hamming, 1);
        self.register("blackman", blackman, 1);
        self.register("rectangular", rectangular, 1);

        // DSP: Utilities
        self.register("linspace", linspace, 3);

        // Matrix operations
        self.register("transpose", transpose, 1);
        self.register("det", det, 1);
        self.register("trace", trace, 1);

        // NOTE: Numerical calculus functions (diff, integral, solve, etc.) are
        // implemented directly in the evaluator (evaluator.rs) because they need
        // access to the evaluator to call lambda functions, similar to HOF functions.
    }

    /// Register a function
    pub fn register(&mut self, name: &str, func: BuiltinFunction, arity: i32) {
        self.functions
            .insert(name.to_string(), (func, arity));
    }

    /// Check if a function is defined
    pub fn has(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    /// Get function arity (-1 for variadic)
    pub fn arity(&self, name: &str) -> Option<i32> {
        self.functions.get(name).map(|(_, arity)| *arity)
    }

    /// Call a function
    pub fn call(&self, name: &str, args: &[Value]) -> Result<Value, String> {
        let (func, arity) = self
            .functions
            .get(name)
            .ok_or_else(|| format!("Unknown function: {}", name))?;

        // Check arity (if not variadic)
        if *arity >= 0 && args.len() != *arity as usize {
            return Err(format!(
                "Function {} expects {} arguments, got {}",
                name,
                arity,
                args.len()
            ));
        }

        func(args)
    }
}

impl Default for FunctionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Mathematical Function Implementations
// ============================================================================

fn sin(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("sin", f64::sin, &args[0])
}

fn cos(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("cos", f64::cos, &args[0])
}

fn tan(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("tan", f64::tan, &args[0])
}

fn asin(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("asin", f64::asin, &args[0])
}

fn acos(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("acos", f64::acos, &args[0])
}

fn atan(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("atan", f64::atan, &args[0])
}

fn atan2(args: &[Value]) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Number(y), Value::Number(x)) => Ok(Value::Number(y.atan2(*x))),
        _ => Err("atan2() requires two numbers".to_string()),
    }
}

fn sinh(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("sinh", f64::sinh, &args[0])
}

fn cosh(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("cosh", f64::cosh, &args[0])
}

fn tanh(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("tanh", f64::tanh, &args[0])
}

fn exp(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("exp", f64::exp, &args[0])
}

fn ln(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("ln", f64::ln, &args[0])
}

fn log10(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("log10", f64::log10, &args[0])
}

fn log2(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("log2", f64::log2, &args[0])
}

fn sqrt(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("sqrt", f64::sqrt, &args[0])
}

fn cbrt(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("cbrt", f64::cbrt, &args[0])
}

fn pow(args: &[Value]) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x.powf(*y))),
        _ => Err("pow() requires two numbers".to_string()),
    }
}

fn floor(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("floor", f64::floor, &args[0])
}

fn ceil(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("ceil", f64::ceil, &args[0])
}

fn round(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("round", f64::round, &args[0])
}

fn abs(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("abs", f64::abs, &args[0])
}

fn trunc(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("trunc", f64::trunc, &args[0])
}

fn sign(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("sign", |x: f64| {
        if x > 0.0 {
            1.0
        } else if x < 0.0 {
            -1.0
        } else {
            0.0
        }
    }, &args[0])
}

fn deg(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("deg", |x: f64| x.to_degrees(), &args[0])
}

fn rad(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("rad", |x: f64| x.to_radians(), &args[0])
}

// ============================================================================
// Complex Number Functions
// ============================================================================

fn complex(args: &[Value]) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Number(re), Value::Number(im)) => {
            Ok(Value::Complex(achronyme_types::complex::Complex::new(*re, *im)))
        }
        _ => Err("complex() requires two numbers (real, imaginary)".to_string()),
    }
}

fn real(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Number(x) => Ok(Value::Number(*x)),
        Value::Complex(c) => Ok(Value::Number(c.re)),
        _ => Err("real() requires a number or complex number".to_string()),
    }
}

fn imag(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Number(_) => Ok(Value::Number(0.0)),
        Value::Complex(c) => Ok(Value::Number(c.im)),
        _ => Err("imag() requires a number or complex number".to_string()),
    }
}

fn conj(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Number(x) => Ok(Value::Number(*x)),
        Value::Complex(c) => Ok(Value::Complex(c.conjugate())),
        _ => Err("conj() requires a number or complex number".to_string()),
    }
}

fn arg(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Number(x) => Ok(Value::Number(if *x >= 0.0 { 0.0 } else { std::f64::consts::PI })),
        Value::Complex(c) => Ok(Value::Number(c.im.atan2(c.re))),
        _ => Err("arg() requires a number or complex number".to_string()),
    }
}

// ============================================================================
// Vector Operations
// ============================================================================

fn dot(args: &[Value]) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Vector(v1), Value::Vector(v2)) => {
            if v1.len() != v2.len() {
                return Err(format!("dot() requires vectors of same length ({} vs {})", v1.len(), v2.len()));
            }
            let result: f64 = v1.data().iter()
                .zip(v2.data().iter())
                .map(|(a, b)| a * b)
                .sum();
            Ok(Value::Number(result))
        }
        _ => Err("dot() requires two vectors".to_string()),
    }
}

fn cross(args: &[Value]) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Vector(v1), Value::Vector(v2)) => {
            if v1.len() != 3 || v2.len() != 3 {
                return Err("cross() requires two 3D vectors".to_string());
            }
            let a = v1.data();
            let b = v2.data();
            let result = vec![
                a[1] * b[2] - a[2] * b[1],
                a[2] * b[0] - a[0] * b[2],
                a[0] * b[1] - a[1] * b[0],
            ];
            Ok(Value::Vector(Vector::new(result)))
        }
        _ => Err("cross() requires two vectors".to_string()),
    }
}

fn norm(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Vector(v) => {
            let sum_squares: f64 = v.data().iter().map(|x| x * x).sum();
            Ok(Value::Number(sum_squares.sqrt()))
        }
        _ => Err("norm() requires a vector".to_string()),
    }
}

fn normalize(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Vector(v) => {
            let sum_squares: f64 = v.data().iter().map(|x| x * x).sum();
            let magnitude = sum_squares.sqrt();
            if magnitude < 1e-10 {
                return Err("normalize() cannot normalize zero vector".to_string());
            }
            let result: Vec<f64> = v.data().iter().map(|x| x / magnitude).collect();
            Ok(Value::Vector(Vector::new(result)))
        }
        _ => Err("normalize() requires a vector".to_string()),
    }
}

// ============================================================================
// Statistical Functions
// ============================================================================

fn sum(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Vector(v) => {
            let total: f64 = v.data().iter().sum();
            Ok(Value::Number(total))
        }
        _ => Err("sum() requires a vector".to_string()),
    }
}

fn mean(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Vector(v) => {
            if v.len() == 0 {
                return Err("mean() requires non-empty vector".to_string());
            }
            let total: f64 = v.data().iter().sum();
            Ok(Value::Number(total / v.len() as f64))
        }
        _ => Err("mean() requires a vector".to_string()),
    }
}

fn std(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Vector(v) => {
            if v.len() < 2 {
                return Err("std() requires vector with at least 2 elements".to_string());
            }
            let n = v.len() as f64;
            let mean_val: f64 = v.data().iter().sum::<f64>() / n;
            let variance: f64 = v.data().iter()
                .map(|x| (x - mean_val).powi(2))
                .sum::<f64>() / (n - 1.0);  // Sample standard deviation
            Ok(Value::Number(variance.sqrt()))
        }
        _ => Err("std() requires a vector".to_string()),
    }
}

// ============================================================================
// DSP Functions: FFT
// ============================================================================

fn fft(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Vector(v) => {
            // Perform FFT
            let spectrum = achronyme_dsp::fft_real(v);

            // Convert to Matrix [N x 2] with [real, imag] pairs
            let n = spectrum.len();
            let mut data = Vec::with_capacity(n * 2);
            for c in spectrum {
                data.push(c.re);
                data.push(c.im);
            }

            let matrix = Matrix::new(n, 2, data)
                .map_err(|e| format!("FFT result conversion failed: {}", e))?;
            Ok(Value::Matrix(matrix))
        }
        _ => Err("fft() requires a vector".to_string()),
    }
}

fn ifft(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Matrix(m) => {
            if m.cols != 2 {
                return Err("ifft() requires matrix with 2 columns [real, imag]".to_string());
            }

            // Convert matrix to complex spectrum
            let mut spectrum = Vec::with_capacity(m.rows);
            for i in 0..m.rows {
                let re = m.get(i, 0).map_err(|e| e.to_string())?;
                let im = m.get(i, 1).map_err(|e| e.to_string())?;
                spectrum.push(AchronymeComplex::new(re, im));
            }

            // Perform IFFT
            let result = achronyme_dsp::ifft_real(&spectrum);
            Ok(Value::Vector(result))
        }
        _ => Err("ifft() requires a matrix [N x 2]".to_string()),
    }
}

fn fft_mag(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Vector(v) => {
            let spectrum = achronyme_dsp::fft_real(v);
            let magnitudes: Vec<f64> = spectrum.iter()
                .map(|c| c.magnitude())
                .collect();
            Ok(Value::Vector(Vector::new(magnitudes)))
        }
        _ => Err("fft_mag() requires a vector".to_string()),
    }
}

fn fft_phase(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Vector(v) => {
            let spectrum = achronyme_dsp::fft_real(v);
            let phases: Vec<f64> = spectrum.iter()
                .map(|c| c.im.atan2(c.re))
                .collect();
            Ok(Value::Vector(Vector::new(phases)))
        }
        _ => Err("fft_phase() requires a vector".to_string()),
    }
}

// ============================================================================
// DSP Functions: Convolution
// ============================================================================

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

// ============================================================================
// DSP Functions: Windows
// ============================================================================

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

// ============================================================================
// DSP Utilities
// ============================================================================

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

// ============================================================================
// Matrix Operations
// ============================================================================

fn transpose(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Matrix(m) => {
            let transposed = m.transpose();
            Ok(Value::Matrix(transposed))
        }
        _ => Err("transpose() requires a matrix".to_string()),
    }
}

fn det(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Matrix(m) => {
            let determinant = achronyme_linalg::determinant_nd(m)
                .map_err(|e| format!("Determinant failed: {}", e))?;
            Ok(Value::Number(determinant))
        }
        _ => Err("det() requires a matrix".to_string()),
    }
}

fn trace(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Matrix(m) => {
            if m.rows != m.cols {
                return Err("trace() requires a square matrix".to_string());
            }

            let mut sum = 0.0;
            for i in 0..m.rows {
                sum += m.get(i, i).map_err(|e| e.to_string())?;
            }
            Ok(Value::Number(sum))
        }
        _ => Err("trace() requires a matrix".to_string()),
    }
}

fn min(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("min() requires at least one argument".to_string());
    }

    let mut result = match &args[0] {
        Value::Number(x) => *x,
        _ => return Err("min() requires numbers".to_string()),
    };

    for arg in &args[1..] {
        match arg {
            Value::Number(x) => {
                if *x < result {
                    result = *x;
                }
            }
            _ => return Err("min() requires numbers".to_string()),
        }
    }

    Ok(Value::Number(result))
}

fn max(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("max() requires at least one argument".to_string());
    }

    let mut result = match &args[0] {
        Value::Number(x) => *x,
        _ => return Err("max() requires numbers".to_string()),
    };

    for arg in &args[1..] {
        match arg {
            Value::Number(x) => {
                if *x > result {
                    result = *x;
                }
            }
            _ => return Err("max() requires numbers".to_string()),
        }
    }

    Ok(Value::Number(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sin() {
        let registry = FunctionRegistry::new();
        let args = vec![Value::Number(std::f64::consts::PI / 2.0)];
        let result = registry.call("sin", &args).unwrap();
        match result {
            Value::Number(x) => assert!((x - 1.0).abs() < 1e-10),
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_sqrt() {
        let registry = FunctionRegistry::new();
        let args = vec![Value::Number(16.0)];
        let result = registry.call("sqrt", &args).unwrap();
        assert_eq!(result, Value::Number(4.0));
    }

    #[test]
    fn test_min_max() {
        let registry = FunctionRegistry::new();
        let args = vec![
            Value::Number(3.0),
            Value::Number(1.0),
            Value::Number(4.0),
            Value::Number(1.5),
        ];
        let min_result = registry.call("min", &args).unwrap();
        let max_result = registry.call("max", &args).unwrap();
        assert_eq!(min_result, Value::Number(1.0));
        assert_eq!(max_result, Value::Number(4.0));
    }

    #[test]
    fn test_arity_check() {
        let registry = FunctionRegistry::new();
        let args = vec![Value::Number(1.0), Value::Number(2.0)];
        let result = registry.call("sin", &args);
        assert!(result.is_err());
    }

    // ========================================================================
    // Vector Support Tests
    // ========================================================================

    #[test]
    fn test_sin_vector() {
        let registry = FunctionRegistry::new();
        let vec = Vector::new(vec![0.0, std::f64::consts::PI / 2.0, std::f64::consts::PI]);
        let args = vec![Value::Vector(vec)];
        let result = registry.call("sin", &args).unwrap();
        match result {
            Value::Vector(v) => {
                assert!((v.data()[0] - 0.0).abs() < 1e-10);
                assert!((v.data()[1] - 1.0).abs() < 1e-10);
                assert!((v.data()[2] - 0.0).abs() < 1e-10);
            }
            _ => panic!("Expected vector"),
        }
    }

    #[test]
    fn test_sqrt_vector() {
        let registry = FunctionRegistry::new();
        let vec = Vector::new(vec![4.0, 9.0, 16.0]);
        let args = vec![Value::Vector(vec)];
        let result = registry.call("sqrt", &args).unwrap();
        match result {
            Value::Vector(v) => {
                assert_eq!(v.data()[0], 2.0);
                assert_eq!(v.data()[1], 3.0);
                assert_eq!(v.data()[2], 4.0);
            }
            _ => panic!("Expected vector"),
        }
    }

    // ========================================================================
    // New Math Functions Tests
    // ========================================================================

    #[test]
    fn test_trunc() {
        let registry = FunctionRegistry::new();
        assert_eq!(registry.call("trunc", &[Value::Number(3.7)]).unwrap(), Value::Number(3.0));
        assert_eq!(registry.call("trunc", &[Value::Number(-3.7)]).unwrap(), Value::Number(-3.0));
    }

    #[test]
    fn test_sign() {
        let registry = FunctionRegistry::new();
        assert_eq!(registry.call("sign", &[Value::Number(5.0)]).unwrap(), Value::Number(1.0));
        assert_eq!(registry.call("sign", &[Value::Number(-5.0)]).unwrap(), Value::Number(-1.0));
        assert_eq!(registry.call("sign", &[Value::Number(0.0)]).unwrap(), Value::Number(0.0));
    }

    #[test]
    fn test_deg_rad() {
        let registry = FunctionRegistry::new();
        let pi = std::f64::consts::PI;

        // rad to deg
        let deg_result = registry.call("deg", &[Value::Number(pi)]).unwrap();
        match deg_result {
            Value::Number(x) => assert!((x - 180.0).abs() < 1e-10),
            _ => panic!("Expected number"),
        }

        // deg to rad
        let rad_result = registry.call("rad", &[Value::Number(180.0)]).unwrap();
        match rad_result {
            Value::Number(x) => assert!((x - pi).abs() < 1e-10),
            _ => panic!("Expected number"),
        }
    }

    // ========================================================================
    // Complex Number Tests
    // ========================================================================

    #[test]
    fn test_complex() {
        let registry = FunctionRegistry::new();
        let result = registry.call("complex", &[Value::Number(3.0), Value::Number(4.0)]).unwrap();
        match result {
            Value::Complex(c) => {
                assert_eq!(c.re, 3.0);
                assert_eq!(c.im, 4.0);
            }
            _ => panic!("Expected complex"),
        }
    }

    #[test]
    fn test_real_imag() {
        let registry = FunctionRegistry::new();
        let c = achronyme_types::complex::Complex::new(3.0, 4.0);

        let real_result = registry.call("real", &[Value::Complex(c)]).unwrap();
        assert_eq!(real_result, Value::Number(3.0));

        let imag_result = registry.call("imag", &[Value::Complex(c)]).unwrap();
        assert_eq!(imag_result, Value::Number(4.0));
    }

    #[test]
    fn test_conj() {
        let registry = FunctionRegistry::new();
        let c = achronyme_types::complex::Complex::new(3.0, 4.0);
        let result = registry.call("conj", &[Value::Complex(c)]).unwrap();
        match result {
            Value::Complex(conjugate) => {
                assert_eq!(conjugate.re, 3.0);
                assert_eq!(conjugate.im, -4.0);
            }
            _ => panic!("Expected complex"),
        }
    }

    #[test]
    fn test_arg() {
        let registry = FunctionRegistry::new();
        let c = achronyme_types::complex::Complex::new(1.0, 1.0);
        let result = registry.call("arg", &[Value::Complex(c)]).unwrap();
        match result {
            Value::Number(angle) => {
                assert!((angle - std::f64::consts::PI / 4.0).abs() < 1e-10);
            }
            _ => panic!("Expected number"),
        }
    }

    // ========================================================================
    // Vector Operations Tests
    // ========================================================================

    #[test]
    fn test_dot() {
        let registry = FunctionRegistry::new();
        let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
        let v2 = Vector::new(vec![4.0, 5.0, 6.0]);
        let result = registry.call("dot", &[Value::Vector(v1), Value::Vector(v2)]).unwrap();
        // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
        assert_eq!(result, Value::Number(32.0));
    }

    #[test]
    fn test_cross() {
        let registry = FunctionRegistry::new();
        let v1 = Vector::new(vec![1.0, 0.0, 0.0]);
        let v2 = Vector::new(vec![0.0, 1.0, 0.0]);
        let result = registry.call("cross", &[Value::Vector(v1), Value::Vector(v2)]).unwrap();
        match result {
            Value::Vector(v) => {
                assert_eq!(v.data()[0], 0.0);
                assert_eq!(v.data()[1], 0.0);
                assert_eq!(v.data()[2], 1.0);
            }
            _ => panic!("Expected vector"),
        }
    }

    #[test]
    fn test_norm() {
        let registry = FunctionRegistry::new();
        let v = Vector::new(vec![3.0, 4.0]);
        let result = registry.call("norm", &[Value::Vector(v)]).unwrap();
        assert_eq!(result, Value::Number(5.0)); // sqrt(3^2 + 4^2) = 5
    }

    #[test]
    fn test_normalize() {
        let registry = FunctionRegistry::new();
        let v = Vector::new(vec![3.0, 4.0]);
        let result = registry.call("normalize", &[Value::Vector(v)]).unwrap();
        match result {
            Value::Vector(normalized) => {
                assert_eq!(normalized.data()[0], 0.6); // 3/5
                assert_eq!(normalized.data()[1], 0.8); // 4/5
            }
            _ => panic!("Expected vector"),
        }
    }

    // ========================================================================
    // Statistical Functions Tests
    // ========================================================================

    #[test]
    fn test_sum() {
        let registry = FunctionRegistry::new();
        let v = Vector::new(vec![1.0, 2.0, 3.0, 4.0]);
        let result = registry.call("sum", &[Value::Vector(v)]).unwrap();
        assert_eq!(result, Value::Number(10.0));
    }

    #[test]
    fn test_mean() {
        let registry = FunctionRegistry::new();
        let v = Vector::new(vec![1.0, 2.0, 3.0, 4.0]);
        let result = registry.call("mean", &[Value::Vector(v)]).unwrap();
        assert_eq!(result, Value::Number(2.5));
    }

    #[test]
    fn test_std() {
        let registry = FunctionRegistry::new();
        let v = Vector::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let result = registry.call("std", &[Value::Vector(v)]).unwrap();
        match result {
            Value::Number(std_dev) => {
                // Sample std dev of [1,2,3,4,5] is sqrt(2.5) ≈ 1.58
                assert!((std_dev - 1.5811388300841898).abs() < 1e-10);
            }
            _ => panic!("Expected number"),
        }
    }

    // ========================================================================
    // DSP Functions Tests
    // ========================================================================

    #[test]
    fn test_fft() {
        let registry = FunctionRegistry::new();
        // DC signal (constant)
        let signal = Vector::new(vec![1.0, 1.0, 1.0, 1.0]);
        let result = registry.call("fft", &[Value::Vector(signal)]).unwrap();
        match result {
            Value::Matrix(m) => {
                assert_eq!(m.rows, 4);
                assert_eq!(m.cols, 2);
                // DC component should be in first bin
                let dc_real = m.get(0, 0).unwrap();
                assert!((dc_real - 4.0).abs() < 1e-10);
            }
            _ => panic!("Expected matrix"),
        }
    }

    #[test]
    fn test_fft_mag() {
        let registry = FunctionRegistry::new();
        let signal = Vector::new(vec![1.0, 0.0, -1.0, 0.0]);
        let result = registry.call("fft_mag", &[Value::Vector(signal)]).unwrap();
        match result {
            Value::Vector(v) => {
                assert_eq!(v.len(), 4);
            }
            _ => panic!("Expected vector"),
        }
    }

    #[test]
    fn test_conv() {
        let registry = FunctionRegistry::new();
        let signal = Vector::new(vec![1.0, 2.0, 3.0]);
        let kernel = Vector::new(vec![1.0, 1.0]);
        let result = registry.call("conv", &[Value::Vector(signal), Value::Vector(kernel)]).unwrap();
        match result {
            Value::Vector(v) => {
                // Expected: [1, 3, 5, 3]
                assert_eq!(v.len(), 4);
                assert!((v.data()[0] - 1.0).abs() < 1e-10);
                assert!((v.data()[1] - 3.0).abs() < 1e-10);
                assert!((v.data()[2] - 5.0).abs() < 1e-10);
                assert!((v.data()[3] - 3.0).abs() < 1e-10);
            }
            _ => panic!("Expected vector"),
        }
    }

    #[test]
    fn test_hanning() {
        let registry = FunctionRegistry::new();
        let result = registry.call("hanning", &[Value::Number(5.0)]).unwrap();
        match result {
            Value::Vector(v) => {
                assert_eq!(v.len(), 5);
                // Hanning window starts and ends at 0
                assert!(v.data()[0].abs() < 1e-10);
                assert!(v.data()[4].abs() < 1e-10);
            }
            _ => panic!("Expected vector"),
        }
    }

    #[test]
    fn test_hamming() {
        let registry = FunctionRegistry::new();
        let result = registry.call("hamming", &[Value::Number(5.0)]).unwrap();
        match result {
            Value::Vector(v) => {
                assert_eq!(v.len(), 5);
                // Hamming window has non-zero endpoints
                assert!(v.data()[0] > 0.0);
            }
            _ => panic!("Expected vector"),
        }
    }

    #[test]
    fn test_linspace() {
        let registry = FunctionRegistry::new();
        let result = registry.call("linspace", &[
            Value::Number(0.0),
            Value::Number(10.0),
            Value::Number(5.0)
        ]).unwrap();
        match result {
            Value::Vector(v) => {
                assert_eq!(v.len(), 5);
                assert_eq!(v.data()[0], 0.0);
                assert_eq!(v.data()[1], 2.5);
                assert_eq!(v.data()[2], 5.0);
                assert_eq!(v.data()[3], 7.5);
                assert_eq!(v.data()[4], 10.0);
            }
            _ => panic!("Expected vector"),
        }
    }

    // ========================================================================
    // Matrix Operations Tests
    // ========================================================================

    #[test]
    fn test_transpose() {
        let registry = FunctionRegistry::new();
        let m = Matrix::new(2, 3, vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0
        ]).unwrap();
        let result = registry.call("transpose", &[Value::Matrix(m)]).unwrap();
        match result {
            Value::Matrix(transposed) => {
                assert_eq!(transposed.rows, 3);
                assert_eq!(transposed.cols, 2);
                assert_eq!(transposed.get(0, 0).unwrap(), 1.0);
                assert_eq!(transposed.get(0, 1).unwrap(), 4.0);
                assert_eq!(transposed.get(1, 0).unwrap(), 2.0);
                assert_eq!(transposed.get(1, 1).unwrap(), 5.0);
            }
            _ => panic!("Expected matrix"),
        }
    }

    #[test]
    fn test_det() {
        let registry = FunctionRegistry::new();
        let m = Matrix::new(2, 2, vec![
            4.0, 7.0,
            2.0, 6.0
        ]).unwrap();
        let result = registry.call("det", &[Value::Matrix(m)]).unwrap();
        // det = 4*6 - 7*2 = 24 - 14 = 10
        match result {
            Value::Number(det) => assert!((det - 10.0).abs() < 1e-10),
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_trace() {
        let registry = FunctionRegistry::new();
        let m = Matrix::new(3, 3, vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0
        ]).unwrap();
        let result = registry.call("trace", &[Value::Matrix(m)]).unwrap();
        // trace = 1 + 5 + 9 = 15
        assert_eq!(result, Value::Number(15.0));
    }
}
