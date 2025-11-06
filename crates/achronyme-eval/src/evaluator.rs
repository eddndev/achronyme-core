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
