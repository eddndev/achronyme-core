use achronyme_parser::ast::{AstNode, BinaryOp, UnaryOp};
use achronyme_types::complex::Complex;
use achronyme_types::matrix::Matrix;
use achronyme_types::value::Value;
use achronyme_types::vector::Vector;

use crate::constants::ConstantsRegistry;
use crate::environment::Environment;
use crate::functions::FunctionRegistry;

/// Evaluator
///
/// Walks the AST and computes the result.
/// Uses a post-order traversal (visit children before parent).
///
/// Example:
/// ```text
///       +
///      / \
///     2   *
///        / \
///       3   4
///
/// Evaluation order:
///   1. eval(2) → 2
///   2. eval(3) → 3
///   3. eval(4) → 4
///   4. eval(3*4) → 12
///   5. eval(2+12) → 14
/// ```
pub struct Evaluator {
    env: Environment,
    constants: ConstantsRegistry,
    functions: FunctionRegistry,
}

impl Evaluator {
    /// Create a new evaluator
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
            constants: ConstantsRegistry::new(),
            functions: FunctionRegistry::new(),
        }
    }

    /// Get the environment (for testing/debugging)
    pub fn environment(&self) -> &Environment {
        &self.env
    }

    /// Get mutable environment (for testing/debugging)
    pub fn environment_mut(&mut self) -> &mut Environment {
        &mut self.env
    }

    /// Evaluate an AST and return the result
    pub fn evaluate(&mut self, node: &AstNode) -> Result<Value, String> {
        match node {
            AstNode::Number(n) => self.evaluate_number(*n),
            AstNode::BinaryOp { op, left, right } => self.evaluate_binary_op(op, left, right),
            AstNode::UnaryOp { op, operand } => self.evaluate_unary_op(op, operand),
            AstNode::FunctionCall { name, args } => self.evaluate_function_call(name, args),
            AstNode::ComplexLiteral { re, im } => self.evaluate_complex_literal(*re, *im),
            AstNode::VectorLiteral(elements) => self.evaluate_vector_literal(elements),
            AstNode::MatrixLiteral(rows) => self.evaluate_matrix_literal(rows),
            AstNode::VariableDecl { name, initializer } => {
                self.evaluate_variable_declaration(name, initializer)
            }
            AstNode::VariableRef(name) => self.evaluate_variable_reference(name),
            AstNode::Lambda { params, body } => self.evaluate_lambda(params, body),
        }
    }

    // ========================================================================
    // Node Evaluation Methods
    // ========================================================================

    fn evaluate_number(&self, n: f64) -> Result<Value, String> {
        Ok(Value::Number(n))
    }

    fn evaluate_binary_op(
        &mut self,
        op: &BinaryOp,
        left: &AstNode,
        right: &AstNode,
    ) -> Result<Value, String> {
        // Post-order: evaluate children first
        let left_val = self.evaluate(left)?;
        let right_val = self.evaluate(right)?;

        // Apply operator
        match op {
            BinaryOp::Add => self.apply_add(left_val, right_val),
            BinaryOp::Subtract => self.apply_subtract(left_val, right_val),
            BinaryOp::Multiply => self.apply_multiply(left_val, right_val),
            BinaryOp::Divide => self.apply_divide(left_val, right_val),
            BinaryOp::Power => self.apply_power(left_val, right_val),
            BinaryOp::Modulo => self.apply_modulo(left_val, right_val),
            BinaryOp::Gt => self.apply_gt(left_val, right_val),
            BinaryOp::Lt => self.apply_lt(left_val, right_val),
            BinaryOp::Gte => self.apply_gte(left_val, right_val),
            BinaryOp::Lte => self.apply_lte(left_val, right_val),
            BinaryOp::Eq => self.apply_eq(left_val, right_val),
            BinaryOp::Neq => self.apply_neq(left_val, right_val),
        }
    }

    fn evaluate_unary_op(&mut self, op: &UnaryOp, operand: &AstNode) -> Result<Value, String> {
        let operand_val = self.evaluate(operand)?;

        match op {
            UnaryOp::Negate => match operand_val {
                Value::Number(n) => Ok(Value::Number(-n)),
                Value::Complex(c) => Ok(Value::Complex(Complex::new(-c.re, -c.im))),
                Value::Vector(v) => Ok(Value::Vector(v.negate())),
                Value::Matrix(m) => Ok(Value::Matrix(m.negate())),
                _ => Err("Cannot negate this type".to_string()),
            },
        }
    }

    fn evaluate_function_call(&mut self, name: &str, args: &[AstNode]) -> Result<Value, String> {
        // Check if it's a constant (zero arguments)
        if args.is_empty() {
            if self.constants.has(name) {
                return Ok(Value::Number(self.constants.get(name)?));
            }
        }

        // Check if it's a lambda stored in a variable
        if self.env.has(name) {
            let var_value = self.env.get(name)?;
            if let Value::Function(ref func) = var_value {
                // Evaluate arguments
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.evaluate(arg)?);
                }
                // Apply lambda function
                let func_clone = func.clone();
                return self.apply_lambda(&func_clone, arg_values);
            }
        }

        // Check for higher-order functions (need evaluator access)
        match name {
            "map" => return self.hof_map(args),
            "filter" => return self.hof_filter(args),
            "reduce" => return self.hof_reduce(args),
            "pipe" => return self.hof_pipe(args),
            _ => {}
        }

        // Otherwise, it's a built-in function call
        if !self.functions.has(name) {
            return Err(format!("Unknown function or constant: {}", name));
        }

        // Evaluate all arguments
        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.evaluate(arg)?);
        }

        // Check arity (if not variadic)
        if let Some(expected_arity) = self.functions.arity(name) {
            if expected_arity >= 0 && arg_values.len() != expected_arity as usize {
                return Err(format!(
                    "Function {} expects {} arguments, got {}",
                    name,
                    expected_arity,
                    arg_values.len()
                ));
            }
        }

        // Call the function
        self.functions.call(name, &arg_values)
    }

    fn evaluate_complex_literal(&self, re: f64, im: f64) -> Result<Value, String> {
        Ok(Value::Complex(Complex::new(re, im)))
    }

    fn evaluate_vector_literal(&mut self, elements: &[AstNode]) -> Result<Value, String> {
        let mut data = Vec::new();

        for element in elements {
            let value = self.evaluate(element)?;

            // For now, only support numbers in vectors
            match value {
                Value::Number(n) => data.push(n),
                _ => return Err("Vector elements must be numbers".to_string()),
            }
        }

        Ok(Value::Vector(Vector::new(data)))
    }

    fn evaluate_matrix_literal(&mut self, rows: &[Vec<AstNode>]) -> Result<Value, String> {
        if rows.is_empty() {
            return Err("Matrix cannot be empty".to_string());
        }

        let num_rows = rows.len();
        let num_cols = rows[0].len();

        // Flatten matrix data (row-major)
        let mut data = Vec::new();

        for row in rows {
            for element in row {
                let value = self.evaluate(element)?;

                // For now, only support numbers in matrices
                match value {
                    Value::Number(n) => data.push(n),
                    _ => return Err("Matrix elements must be numbers".to_string()),
                }
            }
        }

        Matrix::new(num_rows, num_cols, data)
            .map(Value::Matrix)
            .map_err(|e| e.to_string())
    }

    fn evaluate_variable_declaration(
        &mut self,
        name: &str,
        initializer: &AstNode,
    ) -> Result<Value, String> {
        // Evaluate the initializer
        let value = self.evaluate(initializer)?;

        // Define the variable in the environment
        self.env.define(name.to_string(), value.clone())?;

        // Return the value (so "let x = 5" evaluates to 5)
        Ok(value)
    }

    fn evaluate_variable_reference(&self, name: &str) -> Result<Value, String> {
        // Check if it's a variable first
        if self.env.has(name) {
            return self.env.get(name);
        }

        // Otherwise, check if it's a constant
        if self.constants.has(name) {
            return Ok(Value::Number(self.constants.get(name)?));
        }

        // Not found
        Err(format!("Undefined variable or constant: {}", name))
    }

    fn evaluate_lambda(&self, params: &[String], body: &AstNode) -> Result<Value, String> {
        // Capture the current environment (closure)
        let captured_vars = self.env.snapshot();

        // Create a Function value
        use achronyme_types::function::Function;
        let function = Function::new(params.to_vec(), body.clone(), captured_vars);

        Ok(Value::Function(function))
    }

    /// Apply a lambda function to arguments
    fn apply_lambda(&mut self, function: &achronyme_types::function::Function, args: Vec<Value>) -> Result<Value, String> {
        // Check arity
        if args.len() != function.arity() {
            return Err(format!(
                "Lambda expects {} arguments, got {}",
                function.arity(),
                args.len()
            ));
        }

        // Save current environment
        let saved_env = self.env.clone();

        // Create new environment from closure
        self.env = Environment::from_snapshot(function.captured_vars.clone());

        // Bind parameters to arguments
        for (param, arg) in function.params.iter().zip(args.iter()) {
            self.env.define(param.clone(), arg.clone())?;
        }

        // Evaluate the body
        let result = self.evaluate(&function.body);

        // Restore environment
        self.env = saved_env;

        result
    }

    // ========================================================================
    // Higher-Order Functions
    // ========================================================================

    /// map(f, coll1, coll2, ...) - Apply function to elements
    ///
    /// Multi-collection support:
    ///   map(f, [1,2,3]) → applies f(x) to each element
    ///   map(f, [1,2], [3,4]) → applies f(x,y) to pairs
    ///
    /// Truncates to shortest collection.
    fn hof_map(&mut self, args: &[AstNode]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("map requires at least 2 arguments: function and collection(s)".to_string());
        }

        // Evaluate first argument (must be a function)
        let func_value = self.evaluate(&args[0])?;
        let func = match func_value {
            Value::Function(f) => f,
            _ => return Err("First argument to map must be a function".to_string()),
        };

        // Evaluate all collection arguments (must be vectors)
        let mut collections: Vec<Vector> = Vec::new();
        let mut min_length = usize::MAX;

        for arg in &args[1..] {
            let coll_value = self.evaluate(arg)?;
            match coll_value {
                Value::Vector(v) => {
                    min_length = min_length.min(v.len());
                    collections.push(v);
                }
                _ => return Err("map arguments must be vectors".to_string()),
            }
        }

        // Check arity matches number of collections
        if func.arity() != collections.len() {
            return Err(format!(
                "Function arity ({}) must match number of collections ({})",
                func.arity(),
                collections.len()
            ));
        }

        // Apply function to each element
        let mut results = Vec::new();
        for i in 0..min_length {
            // Gather arguments for this iteration
            let mut func_args = Vec::new();
            for coll in &collections {
                func_args.push(Value::Number(coll.data()[i]));
            }

            // Apply function
            let result = self.apply_lambda(&func, func_args)?;

            // Result must be a number (for now)
            match result {
                Value::Number(n) => results.push(n),
                _ => return Err("map function must return numbers".to_string()),
            }
        }

        Ok(Value::Vector(Vector::new(results)))
    }

    /// filter(predicate, collection) - Filter elements
    ///
    /// Returns elements where predicate returns true (non-zero).
    fn hof_filter(&mut self, args: &[AstNode]) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("filter requires 2 arguments: predicate and collection".to_string());
        }

        // Evaluate first argument (must be a function)
        let predicate_value = self.evaluate(&args[0])?;
        let predicate = match predicate_value {
            Value::Function(f) => f,
            _ => return Err("First argument to filter must be a function".to_string()),
        };

        // Evaluate second argument (must be a vector)
        let collection_value = self.evaluate(&args[1])?;
        let collection = match collection_value {
            Value::Vector(v) => v,
            _ => return Err("Second argument to filter must be a vector".to_string()),
        };

        // Predicate must be unary
        if predicate.arity() != 1 {
            return Err("filter predicate must take exactly 1 argument".to_string());
        }

        // Filter elements
        let mut results = Vec::new();
        for i in 0..collection.len() {
            let elem = Value::Number(collection.data()[i]);

            // Apply predicate
            let result = self.apply_lambda(&predicate, vec![elem])?;

            // Check result (non-zero = true)
            match result {
                Value::Number(n) => {
                    if n != 0.0 {
                        results.push(collection.data()[i]);
                    }
                }
                _ => return Err("filter predicate must return a number".to_string()),
            }
        }

        Ok(Value::Vector(Vector::new(results)))
    }

    /// reduce(f, init, collection) - Reduce collection to single value
    ///
    /// Applies f(accumulator, element) repeatedly.
    fn hof_reduce(&mut self, args: &[AstNode]) -> Result<Value, String> {
        if args.len() != 3 {
            return Err("reduce requires 3 arguments: function, initial value, and collection".to_string());
        }

        // Evaluate first argument (must be a function)
        let func_value = self.evaluate(&args[0])?;
        let func = match func_value {
            Value::Function(f) => f,
            _ => return Err("First argument to reduce must be a function".to_string()),
        };

        // Evaluate second argument (initial value, must be number for now)
        let init_value = self.evaluate(&args[1])?;
        let mut accumulator = match init_value {
            Value::Number(n) => n,
            _ => return Err("reduce initial value must be a number".to_string()),
        };

        // Evaluate third argument (must be a vector)
        let collection_value = self.evaluate(&args[2])?;
        let collection = match collection_value {
            Value::Vector(v) => v,
            _ => return Err("Third argument to reduce must be a vector".to_string()),
        };

        // Function must be binary
        if func.arity() != 2 {
            return Err("reduce function must take exactly 2 arguments".to_string());
        }

        // Reduce elements
        for i in 0..collection.len() {
            let acc_val = Value::Number(accumulator);
            let elem_val = Value::Number(collection.data()[i]);

            // Apply function
            let result = self.apply_lambda(&func, vec![acc_val, elem_val])?;

            // Result must be number
            match result {
                Value::Number(n) => accumulator = n,
                _ => return Err("reduce function must return a number".to_string()),
            }
        }

        Ok(Value::Number(accumulator))
    }

    /// pipe(value, f1, f2, ...) - Apply functions left-to-right
    ///
    /// pipe(x, f, g, h) = h(g(f(x)))
    ///
    /// First argument is the initial value, rest are unary functions.
    fn hof_pipe(&mut self, args: &[AstNode]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("pipe requires at least 2 arguments: value and function(s)".to_string());
        }

        // Evaluate first argument (initial value)
        let mut result = self.evaluate(&args[0])?;

        // Apply each function left-to-right
        for arg in &args[1..] {
            let func_value = self.evaluate(arg)?;
            let func = match func_value {
                Value::Function(f) => f,
                _ => return Err("pipe arguments after the first must be functions".to_string()),
            };

            // Check arity
            if func.arity() != 1 {
                return Err("pipe only supports unary functions".to_string());
            }

            // Apply function to current result
            result = self.apply_lambda(&func, vec![result])?;
        }

        Ok(result)
    }

    // ========================================================================
    // Binary Operation Helpers
    // ========================================================================

    fn apply_add(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::Vector(a), Value::Vector(b)) => a
                .add(&b)
                .map(Value::Vector)
                .map_err(|e| e.to_string()),
            (Value::Matrix(a), Value::Matrix(b)) => a
                .add(&b)
                .map(Value::Matrix)
                .map_err(|e| e.to_string()),
            (Value::Complex(a), Value::Complex(b)) => Ok(Value::Complex(a + b)),
            // Type promotion: Number → Complex
            (Value::Number(a), Value::Complex(b)) => {
                Ok(Value::Complex(Complex::from_real(a) + b))
            }
            (Value::Complex(a), Value::Number(b)) => {
                Ok(Value::Complex(a + Complex::from_real(b)))
            }
            _ => Err("Incompatible types for addition".to_string()),
        }
    }

    fn apply_subtract(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            (Value::Vector(a), Value::Vector(b)) => a
                .sub(&b)
                .map(Value::Vector)
                .map_err(|e| e.to_string()),
            (Value::Matrix(a), Value::Matrix(b)) => a
                .sub(&b)
                .map(Value::Matrix)
                .map_err(|e| e.to_string()),
            (Value::Complex(a), Value::Complex(b)) => Ok(Value::Complex(a - b)),
            (Value::Number(a), Value::Complex(b)) => {
                Ok(Value::Complex(Complex::from_real(a) - b))
            }
            (Value::Complex(a), Value::Number(b)) => {
                Ok(Value::Complex(a - Complex::from_real(b)))
            }
            _ => Err("Incompatible types for subtraction".to_string()),
        }
    }

    fn apply_multiply(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            (Value::Vector(a), Value::Vector(b)) => a
                .mul(&b)
                .map(Value::Vector)
                .map_err(|e| e.to_string()),
            (Value::Matrix(a), Value::Matrix(b)) => a
                .mul(&b)
                .map(Value::Matrix)
                .map_err(|e| e.to_string()),
            (Value::Complex(a), Value::Complex(b)) => Ok(Value::Complex(a * b)),
            (Value::Number(a), Value::Complex(b)) => {
                Ok(Value::Complex(Complex::from_real(a) * b))
            }
            (Value::Complex(a), Value::Number(b)) => {
                Ok(Value::Complex(a * Complex::from_real(b)))
            }
            _ => Err("Incompatible types for multiplication".to_string()),
        }
    }

    fn apply_divide(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                if b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Number(a / b))
                }
            }
            (Value::Vector(a), Value::Vector(b)) => a
                .div(&b)
                .map(Value::Vector)
                .map_err(|e| e.to_string()),
            (Value::Complex(a), Value::Complex(b)) => Ok(Value::Complex(a / b)),
            (Value::Number(a), Value::Complex(b)) => {
                Ok(Value::Complex(Complex::from_real(a) / b))
            }
            (Value::Complex(a), Value::Number(b)) => {
                Ok(Value::Complex(a / Complex::from_real(b)))
            }
            _ => Err("Incompatible types for division".to_string()),
        }
    }

    fn apply_power(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.powf(b))),
            (Value::Complex(a), Value::Number(b)) => Ok(Value::Complex(a.pow(b))),
            _ => Err("Incompatible types for power".to_string()),
        }
    }

    fn apply_modulo(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                if b == 0.0 {
                    Err("Modulo by zero".to_string())
                } else {
                    Ok(Value::Number(a % b))
                }
            }
            _ => Err("Modulo operator currently only supports numbers".to_string()),
        }
    }

    // Comparison operators (return 1.0 for true, 0.0 for false)
    fn apply_gt(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                Ok(Value::Number(if a > b { 1.0 } else { 0.0 }))
            }
            _ => Err("Comparison operators currently only support numbers".to_string()),
        }
    }

    fn apply_lt(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                Ok(Value::Number(if a < b { 1.0 } else { 0.0 }))
            }
            _ => Err("Comparison operators currently only support numbers".to_string()),
        }
    }

    fn apply_gte(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                Ok(Value::Number(if a >= b { 1.0 } else { 0.0 }))
            }
            _ => Err("Comparison operators currently only support numbers".to_string()),
        }
    }

    fn apply_lte(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                Ok(Value::Number(if a <= b { 1.0 } else { 0.0 }))
            }
            _ => Err("Comparison operators currently only support numbers".to_string()),
        }
    }

    fn apply_eq(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                Ok(Value::Number(if a == b { 1.0 } else { 0.0 }))
            }
            _ => Err("Comparison operators currently only support numbers".to_string()),
        }
    }

    fn apply_neq(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                Ok(Value::Number(if a != b { 1.0 } else { 0.0 }))
            }
            _ => Err("Comparison operators currently only support numbers".to_string()),
        }
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use achronyme_parser::lexer::Lexer;
    use achronyme_parser::parser::Parser;

    fn eval(source: &str) -> Result<Value, String> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;
        let mut evaluator = Evaluator::new();
        evaluator.evaluate(&ast)
    }

    #[test]
    fn test_number() {
        let result = eval("42").unwrap();
        assert_eq!(result, Value::Number(42.0));
    }

    #[test]
    fn test_addition() {
        let result = eval("2 + 3").unwrap();
        assert_eq!(result, Value::Number(5.0));
    }

    #[test]
    fn test_precedence() {
        let result = eval("2 + 3 * 4").unwrap();
        assert_eq!(result, Value::Number(14.0)); // 2 + (3 * 4) = 2 + 12 = 14
    }

    #[test]
    fn test_power() {
        let result = eval("2 ^ 3").unwrap();
        assert_eq!(result, Value::Number(8.0));
    }

    #[test]
    fn test_negation() {
        let result = eval("-5").unwrap();
        assert_eq!(result, Value::Number(-5.0));
    }

    #[test]
    fn test_function_sin() {
        let result = eval("sin(0)").unwrap();
        match result {
            Value::Number(x) => assert!(x.abs() < 1e-10),
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_constant_pi() {
        let result = eval("PI").unwrap();
        match result {
            Value::Number(x) => assert!((x - std::f64::consts::PI).abs() < 1e-10),
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_variable() {
        let result = eval("let x = 5").unwrap();
        assert_eq!(result, Value::Number(5.0));
    }

    #[test]
    fn test_vector() {
        let result = eval("[1, 2, 3]").unwrap();
        match result {
            Value::Vector(v) => assert_eq!(v.len(), 3),
            _ => panic!("Expected vector"),
        }
    }

    #[test]
    fn test_comparison() {
        let result = eval("5 > 3").unwrap();
        assert_eq!(result, Value::Number(1.0)); // true
        let result = eval("5 < 3").unwrap();
        assert_eq!(result, Value::Number(0.0)); // false
    }

    // ========================================================================
    // Lambda and Closure Tests
    // ========================================================================

    #[test]
    fn test_lambda_creation() {
        // Create a lambda
        let mut lexer = Lexer::new("x => x * 2");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let mut evaluator = Evaluator::new();
        let result = evaluator.evaluate(&ast).unwrap();

        // Should be a function value
        match result {
            Value::Function(_) => {}, // Success
            _ => panic!("Expected function value"),
        }
    }

    #[test]
    fn test_lambda_call() {
        // Define lambda and call it
        let mut lexer = Lexer::new("let f = x => x * 2");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let mut evaluator = Evaluator::new();
        evaluator.evaluate(&ast).unwrap();

        // Now call it
        let mut lexer = Lexer::new("f(5)");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let result = evaluator.evaluate(&ast).unwrap();

        assert_eq!(result, Value::Number(10.0));
    }

    #[test]
    fn test_lambda_closure() {
        // Lambda captures variable from outer scope
        let source = "let x = 10";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let mut evaluator = Evaluator::new();
        evaluator.evaluate(&ast).unwrap();

        // Create lambda that uses x
        let source = "let f = y => x + y";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        evaluator.evaluate(&ast).unwrap();

        // Call lambda
        let source = "f(5)";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let result = evaluator.evaluate(&ast).unwrap();

        assert_eq!(result, Value::Number(15.0)); // 10 + 5
    }

    #[test]
    fn test_lambda_multi_param() {
        // Lambda with multiple parameters
        let source = "let add = (x, y) => x + y";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let mut evaluator = Evaluator::new();
        evaluator.evaluate(&ast).unwrap();

        // Call it
        let source = "add(3, 4)";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let result = evaluator.evaluate(&ast).unwrap();

        assert_eq!(result, Value::Number(7.0));
    }

    #[test]
    fn test_lambda_arity_check() {
        // Lambda arity mismatch should fail
        let source = "let f = x => x * 2";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let mut evaluator = Evaluator::new();
        evaluator.evaluate(&ast).unwrap();

        // Call with wrong number of args
        let source = "f(1, 2)";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let result = evaluator.evaluate(&ast);

        assert!(result.is_err());
    }

    #[test]
    fn test_lambda_nested() {
        // Nested lambda (higher-order function)
        let source = "let makeAdder = x => (y => x + y)";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let mut evaluator = Evaluator::new();
        evaluator.evaluate(&ast).unwrap();

        // Get an adder function
        let source = "let add5 = makeAdder(5)";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        evaluator.evaluate(&ast).unwrap();

        // Use it
        let source = "add5(3)";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let result = evaluator.evaluate(&ast).unwrap();

        assert_eq!(result, Value::Number(8.0)); // 5 + 3
    }

    // ========================================================================
    // Higher-Order Functions Tests
    // ========================================================================

    #[test]
    fn test_map_single_collection() {
        // map(x => x * 2, [1,2,3]) → [2,4,6]
        let result = eval("map(x => x * 2,[1,2,3])").unwrap();
        match result {
            Value::Vector(v) => {
                assert_eq!(v.len(), 3);
                assert_eq!(v.data()[0], 2.0);
                assert_eq!(v.data()[1], 4.0);
                assert_eq!(v.data()[2], 6.0);
            }
            _ => panic!("Expected vector"),
        }
    }

    #[test]
    fn test_map_multi_collection() {
        // map((x,y) => x + y, [1,2,3], [4,5,6]) → [5,7,9]
        let result = eval("map((x,y) => x + y,[1,2,3],[4,5,6])").unwrap();
        match result {
            Value::Vector(v) => {
                assert_eq!(v.len(), 3);
                assert_eq!(v.data()[0], 5.0);
                assert_eq!(v.data()[1], 7.0);
                assert_eq!(v.data()[2], 9.0);
            }
            _ => panic!("Expected vector"),
        }
    }

    #[test]
    fn test_map_truncates_to_shortest() {
        // map((x,y) => x + y, [1,2], [3,4,5,6]) → [4,6] (truncates)
        let result = eval("map((x,y) => x + y,[1,2],[3,4,5,6])").unwrap();
        match result {
            Value::Vector(v) => {
                assert_eq!(v.len(), 2);
                assert_eq!(v.data()[0], 4.0);
                assert_eq!(v.data()[1], 6.0);
            }
            _ => panic!("Expected vector"),
        }
    }

    #[test]
    fn test_filter() {
        // filter(x => x > 2, [1,2,3,4,5]) → [3,4,5]
        let result = eval("filter(x => x > 2,[1,2,3,4,5])").unwrap();
        match result {
            Value::Vector(v) => {
                assert_eq!(v.len(), 3);
                assert_eq!(v.data()[0], 3.0);
                assert_eq!(v.data()[1], 4.0);
                assert_eq!(v.data()[2], 5.0);
            }
            _ => panic!("Expected vector"),
        }
    }

    #[test]
    fn test_filter_even_numbers() {
        // filter(x => x % 2 == 0, [1,2,3,4,5,6]) → [2,4,6]
        // Note: == returns 1.0 for true, 0.0 for false
        let result = eval("filter(x => (x % 2) == 0,[1,2,3,4,5,6])").unwrap();
        match result {
            Value::Vector(v) => {
                assert_eq!(v.len(), 3);
                assert_eq!(v.data()[0], 2.0);
                assert_eq!(v.data()[1], 4.0);
                assert_eq!(v.data()[2], 6.0);
            }
            _ => panic!("Expected vector"),
        }
    }

    #[test]
    fn test_reduce_sum() {
        // reduce((acc, x) => acc + x, 0, [1,2,3,4]) → 10
        let result = eval("reduce((acc,x) => acc + x,0,[1,2,3,4])").unwrap();
        assert_eq!(result, Value::Number(10.0));
    }

    #[test]
    fn test_reduce_product() {
        // reduce((acc, x) => acc * x, 1, [2,3,4]) → 24
        let result = eval("reduce((acc,x) => acc * x,1,[2,3,4])").unwrap();
        assert_eq!(result, Value::Number(24.0));
    }

    #[test]
    fn test_reduce_max() {
        // reduce((acc, x) => max(acc, x), 0, [3,1,4,1,5,9]) → 9
        let result = eval("reduce((acc,x) => max(acc,x),0,[3,1,4,1,5,9])").unwrap();
        assert_eq!(result, Value::Number(9.0));
    }

    #[test]
    fn test_pipe_simple() {
        // pipe(5, x => x * 2, x => x + 1) → 11
        let result = eval("pipe(5,x => x * 2,x => x + 1)").unwrap();
        assert_eq!(result, Value::Number(11.0));
    }

    #[test]
    fn test_pipe_multiple_functions() {
        // pipe(2, x => x + 1, x => x * 2, x => x ^ 2) → 36
        // 2 → 3 → 6 → 36
        let result = eval("pipe(2,x => x + 1,x => x * 2,x => x ^ 2)").unwrap();
        assert_eq!(result, Value::Number(36.0));
    }

    #[test]
    fn test_hof_composition() {
        // Test combining HOFs
        // Get squares of even numbers: filter(even) then map(square)
        let mut lexer = Lexer::new("let evens = filter(x => (x % 2) == 0,[1,2,3,4,5,6])");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let mut evaluator = Evaluator::new();
        evaluator.evaluate(&ast).unwrap();

        // Now map square over evens
        let source = "map(x => x ^ 2,evens)";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let result = evaluator.evaluate(&ast).unwrap();

        match result {
            Value::Vector(v) => {
                assert_eq!(v.len(), 3);
                assert_eq!(v.data()[0], 4.0);  // 2^2
                assert_eq!(v.data()[1], 16.0); // 4^2
                assert_eq!(v.data()[2], 36.0); // 6^2
            }
            _ => panic!("Expected vector"),
        }
    }

    #[test]
    fn test_map_arity_mismatch() {
        // map with wrong function arity should fail
        let result = eval("map((x,y) => x + y,[1,2,3])");
        assert!(result.is_err());
    }

    #[test]
    fn test_filter_non_unary_predicate() {
        // filter with non-unary predicate should fail
        let result = eval("filter((x,y) => x + y,[1,2,3])");
        assert!(result.is_err());
    }

    #[test]
    fn test_reduce_non_binary_function() {
        // reduce with non-binary function should fail
        let result = eval("reduce(x => x * 2,0,[1,2,3])");
        assert!(result.is_err());
    }

    #[test]
    fn test_pipe_non_unary_function() {
        // pipe with non-unary function should fail
        let result = eval("pipe(5,(x,y) => x + y)");
        assert!(result.is_err());
    }
}
