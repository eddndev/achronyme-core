use achronyme_parser::ast::AstNode;
use achronyme_types::complex::Complex;
use achronyme_types::matrix::Matrix;
use achronyme_types::value::Value;
use achronyme_types::vector::Vector;
use achronyme_types::function::Function;
use achronyme_types::LambdaEvaluator;

use crate::constants::ConstantsRegistry;
use crate::environment::Environment;
use crate::functions::FunctionRegistry;
use crate::handlers;

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

    /// Get the constants registry (for handlers)
    pub fn constants(&self) -> &ConstantsRegistry {
        &self.constants
    }

    /// Get the functions registry (for handlers)
    pub fn functions(&self) -> &FunctionRegistry {
        &self.functions
    }

    /// Get mutable functions registry (for handlers)
    pub fn functions_mut(&mut self) -> &mut FunctionRegistry {
        &mut self.functions
    }

    /// Evaluate a SOC expression string using the Pest parser (NEW)
    ///
    /// This is the new and recommended way to evaluate SOC expressions.
    /// It uses the Pest parser which is more robust and maintainable.
    ///
    /// Example:
    /// ```rust
    /// use achronyme_eval::Evaluator;
    ///
    /// let mut evaluator = Evaluator::new();
    /// let result = evaluator.eval_str("2 + 3 * 4").unwrap();
    /// ```
    pub fn eval_str(&mut self, source: &str) -> Result<Value, String> {
        use achronyme_parser::parse;

        let statements = parse(source)?;

        if statements.is_empty() {
            return Err("No statements to evaluate".to_string());
        }

        // Evaluate all statements, return the last one
        let mut result = Value::Number(0.0);
        for stmt in &statements {
            result = self.evaluate(stmt)?;
        }

        Ok(result)
    }

    /// Evaluate an AST and return the result
    pub fn evaluate(&mut self, node: &AstNode) -> Result<Value, String> {
        match node {
            AstNode::Number(n) => self.evaluate_number(*n),
            AstNode::Boolean(b) => self.evaluate_boolean(*b),
            AstNode::BinaryOp { op, left, right } => self.evaluate_binary_op(op, left, right),
            AstNode::UnaryOp { op, operand } => self.evaluate_unary_op(op, operand),
            AstNode::If { condition, then_expr, else_expr } => {
                self.evaluate_if(condition, then_expr, else_expr)
            }
            AstNode::Piecewise { cases, default } => {
                self.evaluate_piecewise(cases, default)
            }
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

    fn evaluate_boolean(&self, b: bool) -> Result<Value, String> {
        Ok(Value::Boolean(b))
    }

    fn evaluate_if(
        &mut self,
        condition: &AstNode,
        then_expr: &AstNode,
        else_expr: &AstNode,
    ) -> Result<Value, String> {
        // Evaluate condition
        let cond_val = self.evaluate(condition)?;

        // Convert to boolean
        let cond_bool = Self::value_to_bool(&cond_val)?;

        // Evaluate appropriate branch (short-circuit)
        if cond_bool {
            self.evaluate(then_expr)
        } else {
            self.evaluate(else_expr)
        }
    }

    fn evaluate_piecewise(
        &mut self,
        cases: &[(Box<AstNode>, Box<AstNode>)],
        default: &Option<Box<AstNode>>,
    ) -> Result<Value, String> {
        // Evaluate cases in order (short-circuit)
        for (condition, expression) in cases {
            let cond_val = self.evaluate(condition)?;
            let cond_bool = Self::value_to_bool(&cond_val)?;

            if cond_bool {
                return self.evaluate(expression);
            }
        }

        // If no condition was true, evaluate default if present
        if let Some(default_expr) = default {
            return self.evaluate(default_expr);
        }

        // No condition was true and no default provided
        Err("piecewise: no condition was true and no default value provided".to_string())
    }

    /// Helper to convert Value to bool
    /// Boolean values map directly, numbers: 0 = false, != 0 = true
    fn value_to_bool(value: &Value) -> Result<bool, String> {
        match value {
            Value::Boolean(b) => Ok(*b),
            Value::Number(n) => Ok(*n != 0.0),
            _ => Err(format!("Cannot convert {:?} to boolean", value)),
        }
    }

    fn evaluate_binary_op(
        &mut self,
        op: &achronyme_parser::ast::BinaryOp,
        left: &AstNode,
        right: &AstNode,
    ) -> Result<Value, String> {
        // Post-order: evaluate children first
        let left_val = self.evaluate(left)?;
        let right_val = self.evaluate(right)?;

        // Delegate to binary_ops handler
        handlers::binary_ops::apply(op, left_val, right_val)
    }

    fn evaluate_unary_op(&mut self, op: &achronyme_parser::ast::UnaryOp, operand: &AstNode) -> Result<Value, String> {
        let operand_val = self.evaluate(operand)?;

        // Delegate to unary_ops handler
        handlers::unary_ops::apply(op, operand_val)
    }

    fn evaluate_function_call(&mut self, name: &str, args: &[AstNode]) -> Result<Value, String> {
        // Delegate to function_call handler
        handlers::function_call::dispatch(self, name, args)
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
    pub fn apply_lambda(&mut self, function: &achronyme_types::function::Function, args: Vec<Value>) -> Result<Value, String> {
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
    // Helper Methods
    // ========================================================================

    /// Helper to evaluate a lambda function at a single point
    /// Used by LambdaEvaluator trait implementation
    fn eval_lambda_at(&mut self, func: &achronyme_types::function::Function, x: f64) -> Result<f64, String> {
        let result = self.apply_lambda(func, vec![Value::Number(x)])?;
        match result {
            Value::Number(n) => Ok(n),
            _ => Err("Lambda must return a number".to_string()),
        }
    }
}

// ============================================================================
// LambdaEvaluator Implementation
// ============================================================================

impl LambdaEvaluator for Evaluator {
    fn eval_at(&mut self, func: &Function, x: f64) -> Result<f64, String> {
        // Use the existing eval_lambda_at method
        self.eval_lambda_at(func, x)
    }

    fn eval_vec_at(&mut self, func: &Function, point: &[f64]) -> Result<f64, String> {
        // Create a vector value and apply the lambda
        let vec_arg = Value::Vector(Vector::new(point.to_vec()));
        let result = self.apply_lambda(func, vec![vec_arg])?;

        // Extract the numeric result
        match result {
            Value::Number(n) => Ok(n),
            _ => Err("Lambda function must return a number".to_string()),
        }
    }

    fn eval_at_nd(&mut self, func: &Function, args: &[f64]) -> Result<f64, String> {
        // Convert each f64 to a Value::Number
        let value_args: Vec<Value> = args.iter()
            .map(|&x| Value::Number(x))
            .collect();

        // Apply the lambda with multiple arguments
        let result = self.apply_lambda(func, value_args)?;

        // Extract the numeric result
        match result {
            Value::Number(n) => Ok(n),
            _ => Err("Lambda function must return a number".to_string()),
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
    use achronyme_parser::parse;

    fn eval(source: &str) -> Result<Value, String> {
        let statements = parse(source)?;
        let mut evaluator = Evaluator::new();

        // Evaluate all statements, return the last result
        let mut result = Value::Number(0.0);
        for stmt in &statements {
            result = evaluator.evaluate(stmt)?;
        }

        Ok(result)
    }

    /// Helper function for tests that need to maintain state across multiple eval calls
    fn eval_with_evaluator(evaluator: &mut Evaluator, source: &str) -> Result<Value, String> {
        let statements = parse(source)?;

        // Evaluate all statements, return the last result
        let mut result = Value::Number(0.0);
        for stmt in &statements {
            result = evaluator.evaluate(stmt)?;
        }

        Ok(result)
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
        assert_eq!(result, Value::Boolean(true));
        let result = eval("5 < 3").unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    // ========================================================================
    // Lambda and Closure Tests
    // ========================================================================

    #[test]
    fn test_lambda_creation() {
        // Create a lambda
        let result = eval("x => x * 2").unwrap();

        // Should be a function value
        match result {
            Value::Function(_) => {}, // Success
            _ => panic!("Expected function value"),
        }
    }

    #[test]
    fn test_lambda_call() {
        // Define lambda and call it
        let mut evaluator = Evaluator::new();
        eval_with_evaluator(&mut evaluator, "let f = x => x * 2").unwrap();

        // Now call it
        let result = eval_with_evaluator(&mut evaluator, "f(5)").unwrap();

        assert_eq!(result, Value::Number(10.0));
    }

    #[test]
    fn test_lambda_closure() {
        // Lambda captures variable from outer scope
        let mut evaluator = Evaluator::new();
        eval_with_evaluator(&mut evaluator, "let x = 10").unwrap();

        // Create lambda that uses x
        eval_with_evaluator(&mut evaluator, "let f = y => x + y").unwrap();

        // Call lambda
        let result = eval_with_evaluator(&mut evaluator, "f(5)").unwrap();

        assert_eq!(result, Value::Number(15.0)); // 10 + 5
    }

    #[test]
    fn test_lambda_multi_param() {
        // Lambda with multiple parameters
        let mut evaluator = Evaluator::new();
        eval_with_evaluator(&mut evaluator, "let add = (x, y) => x + y").unwrap();

        // Call it
        let result = eval_with_evaluator(&mut evaluator, "add(3, 4)").unwrap();

        assert_eq!(result, Value::Number(7.0));
    }

    #[test]
    fn test_lambda_arity_check() {
        // Lambda arity mismatch should fail
        let mut evaluator = Evaluator::new();
        eval_with_evaluator(&mut evaluator, "let f = x => x * 2").unwrap();

        // Call with wrong number of args
        let result = eval_with_evaluator(&mut evaluator, "f(1, 2)");

        assert!(result.is_err());
    }

    #[test]
    fn test_lambda_nested() {
        // Nested lambda (higher-order function)
        let mut evaluator = Evaluator::new();
        eval_with_evaluator(&mut evaluator, "let makeAdder = x => (y => x + y)").unwrap();

        // Get an adder function
        eval_with_evaluator(&mut evaluator, "let add5 = makeAdder(5)").unwrap();

        // Use it
        let result = eval_with_evaluator(&mut evaluator, "add5(3)").unwrap();

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
        let mut evaluator = Evaluator::new();
        eval_with_evaluator(&mut evaluator, "let evens = filter(x => (x % 2) == 0,[1,2,3,4,5,6])").unwrap();

        // Now map square over evens
        let result = eval_with_evaluator(&mut evaluator, "map(x => x ^ 2,evens)").unwrap();

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

    // ========================================================================
    // Pest Parser Tests (New)
    // ========================================================================

    #[test]
    fn test_pest_simple_arithmetic() {
        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_str("2 + 3 * 4").unwrap();
        assert_eq!(result, Value::Number(14.0)); // 2 + (3 * 4)
    }

    #[test]
    fn test_pest_power_right_associative() {
        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_str("2^3^2").unwrap();
        assert_eq!(result, Value::Number(512.0)); // 2^(3^2) = 2^9
    }

    #[test]
    fn test_pest_vector() {
        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_str("[1, 2, 3]").unwrap();
        match result {
            Value::Vector(v) => {
                assert_eq!(v.data(), &[1.0, 2.0, 3.0]);
            }
            _ => panic!("Expected vector"),
        }
    }

    #[test]
    fn test_pest_function_call() {
        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_str("sin(0)").unwrap();
        match result {
            Value::Number(x) => assert!(x.abs() < 1e-10),
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_pest_let_and_reference() {
        let mut evaluator = Evaluator::new();
        evaluator.eval_str("let x = 42").unwrap();
        let result = evaluator.eval_str("x").unwrap();
        assert_eq!(result, Value::Number(42.0));
    }

    #[test]
    fn test_pest_lambda() {
        let mut evaluator = Evaluator::new();
        evaluator.eval_str("let f = x => x^2").unwrap();
        // Lambda should be stored in environment
        assert!(evaluator.environment().get("f").is_ok());
    }

    #[test]
    fn test_pest_complex_expression() {
        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_str("(2 + 3) * (4 - 1)").unwrap();
        assert_eq!(result, Value::Number(15.0)); // 5 * 3
    }

    #[test]
    fn test_pest_matrix() {
        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_str("[[1, 2], [3, 4]]").unwrap();
        match result {
            Value::Matrix(_) => {}, // Success
            _ => panic!("Expected matrix"),
        }
    }

    #[test]
    fn test_pest_comparison() {
        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_str("5 > 3").unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_pest_multiple_statements() {
        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_str("let x = 10\nlet y = 20\nx + y").unwrap();
        assert_eq!(result, Value::Number(30.0));
    }

    #[test]
    fn test_pest_with_comments() {
        let mut evaluator = Evaluator::new();
        let source = "// This is a comment\nlet x = 42\n// Another comment\nx * 2";
        let result = evaluator.eval_str(source).unwrap();
        assert_eq!(result, Value::Number(84.0));
    }

    #[test]
    fn test_pest_soc_style_script() {
        let mut evaluator = Evaluator::new();
        let source = r#"
// Test simple
let x = 10
let y = 20
x + y
"#;
        let result = evaluator.eval_str(source).unwrap();
        assert_eq!(result, Value::Number(30.0));
    }

    // ========================================================================
    // Conditional Tests (if, boolean, logical operators)
    // ========================================================================

    #[test]
    fn test_boolean_literals() {
        assert_eq!(eval("true").unwrap(), Value::Boolean(true));
        assert_eq!(eval("false").unwrap(), Value::Boolean(false));
    }

    #[test]
    fn test_logical_and() {
        assert_eq!(eval("true && true").unwrap(), Value::Boolean(true));
        assert_eq!(eval("true && false").unwrap(), Value::Boolean(false));
        assert_eq!(eval("false && true").unwrap(), Value::Boolean(false));
        assert_eq!(eval("false && false").unwrap(), Value::Boolean(false));
    }

    #[test]
    fn test_logical_or() {
        assert_eq!(eval("true || true").unwrap(), Value::Boolean(true));
        assert_eq!(eval("true || false").unwrap(), Value::Boolean(true));
        assert_eq!(eval("false || true").unwrap(), Value::Boolean(true));
        assert_eq!(eval("false || false").unwrap(), Value::Boolean(false));
    }

    #[test]
    fn test_logical_not() {
        assert_eq!(eval("!true").unwrap(), Value::Boolean(false));
        assert_eq!(eval("!false").unwrap(), Value::Boolean(true));
        assert_eq!(eval("!!true").unwrap(), Value::Boolean(true));
        assert_eq!(eval("!!false").unwrap(), Value::Boolean(false));
    }

    #[test]
    fn test_comparison_returns_boolean() {
        assert_eq!(eval("5 > 3").unwrap(), Value::Boolean(true));
        assert_eq!(eval("5 < 3").unwrap(), Value::Boolean(false));
        assert_eq!(eval("5 >= 5").unwrap(), Value::Boolean(true));
        assert_eq!(eval("5 <= 3").unwrap(), Value::Boolean(false));
        assert_eq!(eval("5 == 5").unwrap(), Value::Boolean(true));
        assert_eq!(eval("5 != 3").unwrap(), Value::Boolean(true));
    }

    #[test]
    fn test_if_simple() {
        assert_eq!(eval("if(true, 1, 2)").unwrap(), Value::Number(1.0));
        assert_eq!(eval("if(false, 1, 2)").unwrap(), Value::Number(2.0));
    }

    #[test]
    fn test_if_with_comparison() {
        assert_eq!(eval("if(5 > 3, 100, 200)").unwrap(), Value::Number(100.0));
        assert_eq!(eval("if(2 > 10, 100, 200)").unwrap(), Value::Number(200.0));
    }

    #[test]
    fn test_if_with_logical_ops() {
        assert_eq!(eval("if(true && true, 1, 0)").unwrap(), Value::Number(1.0));
        assert_eq!(eval("if(true && false, 1, 0)").unwrap(), Value::Number(0.0));
        assert_eq!(eval("if(false || true, 1, 0)").unwrap(), Value::Number(1.0));
    }

    #[test]
    fn test_if_nested() {
        // if(x > 0, if(x > 10, 2, 1), 0)
        let mut evaluator = Evaluator::new();
        eval_with_evaluator(&mut evaluator, "let x = 15").unwrap();
        let result = eval_with_evaluator(&mut evaluator, "if(x > 0, if(x > 10, 2, 1), 0)").unwrap();
        assert_eq!(result, Value::Number(2.0));

        let mut evaluator = Evaluator::new();
        eval_with_evaluator(&mut evaluator, "let x = 5").unwrap();
        let result = eval_with_evaluator(&mut evaluator, "if(x > 0, if(x > 10, 2, 1), 0)").unwrap();
        assert_eq!(result, Value::Number(1.0));

        let mut evaluator = Evaluator::new();
        eval_with_evaluator(&mut evaluator, "let x = -5").unwrap();
        let result = eval_with_evaluator(&mut evaluator, "if(x > 0, if(x > 10, 2, 1), 0)").unwrap();
        assert_eq!(result, Value::Number(0.0));
    }

    #[test]
    fn test_if_in_lambda() {
        // abs function: x => if(x < 0, -x, x)
        let mut evaluator = Evaluator::new();
        eval_with_evaluator(&mut evaluator, "let abs = x => if(x < 0, -x, x)").unwrap();

        let result = eval_with_evaluator(&mut evaluator, "abs(-5)").unwrap();
        assert_eq!(result, Value::Number(5.0));

        let result = eval_with_evaluator(&mut evaluator, "abs(3)").unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    #[test]
    fn test_relu_activation() {
        // ReLU: x => if(x > 0, x, 0)
        let mut evaluator = Evaluator::new();
        eval_with_evaluator(&mut evaluator, "let relu = x => if(x > 0, x, 0)").unwrap();

        let result = eval_with_evaluator(&mut evaluator, "relu(5)").unwrap();
        assert_eq!(result, Value::Number(5.0));

        let result = eval_with_evaluator(&mut evaluator, "relu(-3)").unwrap();
        assert_eq!(result, Value::Number(0.0));

        let result = eval_with_evaluator(&mut evaluator, "relu(0)").unwrap();
        assert_eq!(result, Value::Number(0.0));
    }

    #[test]
    fn test_sign_function() {
        // sign: x => if(x < 0, -1, if(x > 0, 1, 0))
        let mut evaluator = Evaluator::new();
        eval_with_evaluator(&mut evaluator, "let sign = x => if(x < 0, -1, if(x > 0, 1, 0))").unwrap();

        let result = eval_with_evaluator(&mut evaluator, "sign(-10)").unwrap();
        assert_eq!(result, Value::Number(-1.0));

        let result = eval_with_evaluator(&mut evaluator, "sign(10)").unwrap();
        assert_eq!(result, Value::Number(1.0));

        let result = eval_with_evaluator(&mut evaluator, "sign(0)").unwrap();
        assert_eq!(result, Value::Number(0.0));
    }

    // ========================================================================
    // Piecewise Tests
    // ========================================================================

    #[test]
    fn test_piecewise_simple() {
        // piecewise([x < 0, -1], [x > 0, 1], 0)
        let mut evaluator = Evaluator::new();
        eval_with_evaluator(&mut evaluator, "let x = -5").unwrap();
        let result = eval_with_evaluator(&mut evaluator, "piecewise([x < 0, -1], [x > 0, 1], 0)").unwrap();
        assert_eq!(result, Value::Number(-1.0));

        let mut evaluator = Evaluator::new();
        eval_with_evaluator(&mut evaluator, "let x = 5").unwrap();
        let result = eval_with_evaluator(&mut evaluator, "piecewise([x < 0, -1], [x > 0, 1], 0)").unwrap();
        assert_eq!(result, Value::Number(1.0));

        let mut evaluator = Evaluator::new();
        eval_with_evaluator(&mut evaluator, "let x = 0").unwrap();
        let result = eval_with_evaluator(&mut evaluator, "piecewise([x < 0, -1], [x > 0, 1], 0)").unwrap();
        assert_eq!(result, Value::Number(0.0));
    }

    #[test]
    fn test_piecewise_no_default_error() {
        // piecewise without default should error if no condition is true
        let result = eval("piecewise([false, 1], [false, 2])");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("no condition was true"));
    }

    #[test]
    fn test_piecewise_abs() {
        // abs using piecewise: x => piecewise([x < 0, -x], x)
        let mut evaluator = Evaluator::new();
        eval_with_evaluator(&mut evaluator, "let abs = x => piecewise([x < 0, -x], x)").unwrap();

        let result = eval_with_evaluator(&mut evaluator, "abs(-5)").unwrap();
        assert_eq!(result, Value::Number(5.0));

        let result = eval_with_evaluator(&mut evaluator, "abs(3)").unwrap();
        assert_eq!(result, Value::Number(3.0));

        let result = eval_with_evaluator(&mut evaluator, "abs(0)").unwrap();
        assert_eq!(result, Value::Number(0.0));
    }

    #[test]
    fn test_piecewise_tax_bracket() {
        // Progressive tax:
        // income <= 10000: 10%
        // income <= 50000: 20%
        // else: 30%
        let mut evaluator = Evaluator::new();
        eval_with_evaluator(&mut evaluator, "let tax = income => piecewise([income <= 10000, income * 0.1], [income <= 50000, income * 0.2], income * 0.3)").unwrap();

        let result = eval_with_evaluator(&mut evaluator, "tax(5000)").unwrap();
        assert_eq!(result, Value::Number(500.0)); // 10%

        let result = eval_with_evaluator(&mut evaluator, "tax(30000)").unwrap();
        assert_eq!(result, Value::Number(6000.0)); // 20%

        let result = eval_with_evaluator(&mut evaluator, "tax(100000)").unwrap();
        assert_eq!(result, Value::Number(30000.0)); // 30%
    }

    #[test]
    fn test_piecewise_math_function() {
        // f(x) = { x^2    if x < -1
        //        { 2x+1   if -1 <= x < 1
        //        { x^3    if x >= 1
        let mut evaluator = Evaluator::new();
        eval_with_evaluator(&mut evaluator, "let f = x => piecewise([x < -1, x^2], [x < 1, 2*x + 1], x^3)").unwrap();

        // x < -1: x^2
        let result = eval_with_evaluator(&mut evaluator, "f(-2)").unwrap();
        assert_eq!(result, Value::Number(4.0));

        // -1 <= x < 1: 2x+1
        let result = eval_with_evaluator(&mut evaluator, "f(0)").unwrap();
        assert_eq!(result, Value::Number(1.0));

        let result = eval_with_evaluator(&mut evaluator, "f(-1)").unwrap();
        assert_eq!(result, Value::Number(-1.0)); // 2*(-1) + 1

        // x >= 1: x^3
        let result = eval_with_evaluator(&mut evaluator, "f(2)").unwrap();
        assert_eq!(result, Value::Number(8.0));
    }

    #[test]
    fn test_piecewise_heaviside() {
        // Heaviside step function: H(x) = { 0 if x < 0, 1 if x >= 0 }
        let mut evaluator = Evaluator::new();
        eval_with_evaluator(&mut evaluator, "let H = x => piecewise([x < 0, 0], 1)").unwrap();

        let result = eval_with_evaluator(&mut evaluator, "H(-5)").unwrap();
        assert_eq!(result, Value::Number(0.0));

        let result = eval_with_evaluator(&mut evaluator, "H(0)").unwrap();
        assert_eq!(result, Value::Number(1.0));

        let result = eval_with_evaluator(&mut evaluator, "H(5)").unwrap();
        assert_eq!(result, Value::Number(1.0));
    }

    #[test]
    fn test_piecewise_with_hof() {
        // Use piecewise in map
        let mut evaluator = Evaluator::new();
        eval_with_evaluator(&mut evaluator, "let classify = x => piecewise([x < 0, -1], [x > 0, 1], 0)").unwrap();
        let result = eval_with_evaluator(&mut evaluator, "map(classify, [-5, -2, 0, 3, 7])").unwrap();

        match result {
            Value::Vector(v) => {
                assert_eq!(v.data(), &[-1.0, -1.0, 0.0, 1.0, 1.0]);
            }
            _ => panic!("Expected vector"),
        }
    }

    #[test]
    fn test_piecewise_multivariable() {
        // Region classifier in 2D plane: inside circle (1), in square (2), outside (0)
        let mut evaluator = Evaluator::new();
        eval_with_evaluator(&mut evaluator, "let region = (x, y) => piecewise([x^2 + y^2 < 1, 1], [abs(x) < 2 && abs(y) < 2, 2], 0)").unwrap();

        // Inside circle
        let result = eval_with_evaluator(&mut evaluator, "region(0, 0)").unwrap();
        assert_eq!(result, Value::Number(1.0));

        // In square but outside circle
        let result = eval_with_evaluator(&mut evaluator, "region(1.5, 0)").unwrap();
        assert_eq!(result, Value::Number(2.0));

        // Outside both
        let result = eval_with_evaluator(&mut evaluator, "region(3, 3)").unwrap();
        assert_eq!(result, Value::Number(0.0));
    }

    #[test]
    fn test_piecewise_sequential_evaluation() {
        // Verify short-circuit: first true condition wins
        let mut evaluator = Evaluator::new();
        eval_with_evaluator(&mut evaluator, "let x = 5").unwrap();
        // x > 0 is true, so should return 100 (not 200)
        let result = eval_with_evaluator(&mut evaluator, "piecewise([x > 0, 100], [x > 3, 200], 0)").unwrap();
        assert_eq!(result, Value::Number(100.0));
    }

    #[test]
    fn test_piecewise_leaky_relu() {
        // Leaky ReLU: x > 0 ? x : 0.01*x
        let mut evaluator = Evaluator::new();
        eval_with_evaluator(&mut evaluator, "let leaky_relu = x => piecewise([x > 0, x], 0.01 * x)").unwrap();

        let result = eval_with_evaluator(&mut evaluator, "leaky_relu(10)").unwrap();
        assert_eq!(result, Value::Number(10.0));

        let result = eval_with_evaluator(&mut evaluator, "leaky_relu(-10)").unwrap();
        assert_eq!(result, Value::Number(-0.1));
    }
}

