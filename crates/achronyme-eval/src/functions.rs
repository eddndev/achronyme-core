use achronyme_types::value::Value;
use std::collections::HashMap;

use crate::function_modules;

/// Type for built-in function implementations
pub type BuiltinFunction = fn(&[Value]) -> Result<Value, String>;

/// Registry for built-in mathematical functions
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
        registry.register_all_modules();
        registry
    }

    /// Register all standard math functions by delegating to modules
    fn register_all_modules(&mut self) {
        function_modules::array::register_functions(self);
        function_modules::trig::register_functions(self);
        function_modules::exponential::register_functions(self);
        function_modules::rounding::register_functions(self);
        function_modules::complex::register_functions(self);
        function_modules::vector::register_functions(self);
        function_modules::matrix::register_functions(self);
        function_modules::stats::register_functions(self);
        function_modules::dsp::register_functions(self);
        function_modules::strings::register_functions(self);
        function_modules::records::register_functions(self);
        function_modules::graphs::register_functions(self);
        function_modules::utils::register_functions(self);
    }

    /// Register a function
    pub fn register(&mut self, name: &str, func: BuiltinFunction, arity: i32) {
        self.functions.insert(name.to_string(), (func, arity));
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

    #[test]
    fn test_sin_vector() {
        let registry = FunctionRegistry::new();
        let vec = vec![
            Value::Number(0.0),
            Value::Number(std::f64::consts::PI / 2.0),
            Value::Number(std::f64::consts::PI)
        ];
        let args = vec![Value::Vector(vec)];
        let result = registry.call("sin", &args).unwrap();
        match result {
            Value::Vector(v) => {
                match (&v[0], &v[1], &v[2]) {
                    (Value::Number(n0), Value::Number(n1), Value::Number(n2)) => {
                        assert!((n0 - 0.0).abs() < 1e-10);
                        assert!((n1 - 1.0).abs() < 1e-10);
                        assert!((n2 - 0.0).abs() < 1e-10);
                    }
                    _ => panic!("Expected numeric values"),
                }
            }
            _ => panic!("Expected vector"),
        }
    }
}