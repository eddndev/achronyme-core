use achronyme_parser::ast::AstNode;
use achronyme_types::function::Function;
use achronyme_types::value::Value;
use achronyme_types::LambdaEvaluator;
use achronyme_types::Environment;

use crate::constants::ConstantsRegistry;
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

    /// Evaluate a SOC expression string using the Pest parser
    ///
    /// This is the recommended way to evaluate SOC expressions.
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

    /// Evaluate an AST node and return the result
    ///
    /// This is the main dispatch method that routes AST nodes to their
    /// appropriate handler functions.
    pub fn evaluate(&mut self, node: &AstNode) -> Result<Value, String> {
        match node {
            // Literals
            AstNode::Number(n) => handlers::literals::evaluate_number(*n),
            AstNode::Boolean(b) => handlers::literals::evaluate_boolean(*b),
            AstNode::StringLiteral(s) => handlers::literals::evaluate_string(s),
            AstNode::ComplexLiteral { re, im } => handlers::literals::evaluate_complex(*re, *im),
            AstNode::ArrayLiteral(elements) => handlers::literals::evaluate_array(self, elements),
            AstNode::RecordLiteral(fields) => handlers::literals::evaluate_record(self, fields),

            // Variables
            AstNode::VariableDecl { name, initializer } => {
                handlers::variables::evaluate_declaration(self, name, initializer)
            }
            AstNode::VariableRef(name) => handlers::variables::evaluate_reference(self, name),
            AstNode::SelfReference => {
                // Look up 'self' in the environment
                self.env.get("self").map_err(|_| {
                    "'self' can only be used inside record methods".to_string()
                })
            }
            AstNode::RecReference => {
                // Look up 'rec' in the environment
                self.env.get("rec").map_err(|_| {
                    "'rec' can only be used inside functions".to_string()
                })
            }

            // Field access
            AstNode::FieldAccess { record, field } => {
                let record_value = self.evaluate(record)?;
                match record_value {
                    Value::Record(ref map) => {
                        map.get(field)
                            .cloned()
                            .ok_or_else(|| format!("Field '{}' not found in record", field))
                    }
                    Value::Edge { from, to, directed, properties } => {
                        // Handle special fields
                        match field.as_str() {
                            "from" => Ok(Value::String(from.clone())),
                            "to" => Ok(Value::String(to.clone())),
                            "directed" => Ok(Value::Boolean(directed)),
                            // Otherwise, look in properties
                            _ => properties.get(field)
                                .cloned()
                                .ok_or_else(|| format!("Field '{}' not found in edge", field))
                        }
                    }
                    _ => Err(format!("Cannot access field '{}' on non-record/edge value", field)),
                }
            }

            // Control flow
            AstNode::If {
                condition,
                then_expr,
                else_expr,
            } => handlers::control_flow::evaluate_if(self, condition, then_expr, else_expr),
            AstNode::Piecewise { cases, default } => {
                handlers::control_flow::evaluate_piecewise(self, cases, default)
            }

            // Operations
            AstNode::BinaryOp { op, left, right } => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                handlers::binary_ops::apply(op, left_val, right_val)
            }
            AstNode::UnaryOp { op, operand } => {
                let operand_val = self.evaluate(operand)?;
                handlers::unary_ops::apply(op, operand_val)
            }

            // Functions and lambdas
            AstNode::FunctionCall { name, args } => {
                handlers::function_call::dispatch(self, name, args)
            }
            AstNode::CallExpression { callee, args } => {
                // Special case: if callee is a field access (record.method), we need to inject 'self'
                if let AstNode::FieldAccess { record, field } = callee.as_ref() {
                    let record_value = self.evaluate(record)?;

                    match &record_value {
                        Value::Record(ref map) => {
                            let func_value = map.get(field)
                                .cloned()
                                .ok_or_else(|| format!("Field '{}' not found in record", field))?;

                            if let Value::Function(ref func) = func_value {
                                // Evaluate arguments
                                let mut arg_values = Vec::new();
                                for arg in args {
                                    arg_values.push(self.evaluate(arg)?);
                                }

                                // Inject 'self' for method calls
                                self.environment_mut().push_scope();
                                self.environment_mut().define("self".to_string(), record_value)?;

                                let func_clone = func.clone();
                                let result = self.apply_lambda(&func_clone, arg_values);

                                self.environment_mut().pop_scope();
                                return result;
                            } else {
                                return Err(format!("Field '{}' is not a function", field));
                            }
                        }
                        _ => return Err(format!("Cannot access field '{}' on non-record value", field)),
                    }
                }

                // Regular call expression - evaluate callee to get the function
                let func_value = self.evaluate(callee)?;

                // Must be a function
                match func_value {
                    Value::Function(ref func) => {
                        match func {
                            achronyme_types::function::Function::UserDefined { .. } => {
                                // User-defined lambda - evaluate args and apply
                                let mut arg_values = Vec::new();
                                for arg in args {
                                    arg_values.push(self.evaluate(arg)?);
                                }
                                let func_clone = func.clone();
                                self.apply_lambda(&func_clone, arg_values)
                            }
                            achronyme_types::function::Function::Builtin(name) => {
                                // Built-in function - dispatch without evaluating args (let handler do it)
                                handlers::function_call::dispatch(self, name, args)
                            }
                        }
                    }
                    _ => Err("CallExpression requires a function, got non-function value".to_string()),
                }
            }
            AstNode::Lambda { params, body } => {
                handlers::functions::evaluate_lambda(self, params, body)
            }

            // Edges (graph/network support)
            AstNode::Edge { from, to, directed, metadata } => {
                handlers::literals::evaluate_edge(self, from, to, *directed, metadata)
            }

            // Indexing and slicing
            AstNode::IndexAccess { object, indices } => {
                handlers::indexing::evaluate_index_access(self, object, indices)
            }
        }
    }

    /// Apply a lambda function to arguments
    pub fn apply_lambda(
        &mut self,
        function: &Function,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        handlers::functions::apply_lambda(self, function, args)
    }
}

// ============================================================================
// LambdaEvaluator Implementation
// ============================================================================

impl LambdaEvaluator for Evaluator {
    fn eval_at(&mut self, func: &Function, x: f64) -> Result<f64, String> {
        handlers::functions::eval_lambda_at(self, func, x)
    }

    fn eval_vec_at(&mut self, func: &Function, point: &[f64]) -> Result<f64, String> {
        // Create a vector value and apply the lambda
        let vec_arg = Value::Vector(point.iter().map(|&n| Value::Number(n)).collect());
        let result = self.apply_lambda(func, vec![vec_arg])?;

        // Extract the numeric result
        match result {
            Value::Number(n) => Ok(n),
            _ => Err("Lambda function must return a number".to_string()),
        }
    }

    fn eval_at_nd(&mut self, func: &Function, args: &[f64]) -> Result<f64, String> {
        // Convert each f64 to a Value::Number
        let value_args: Vec<Value> = args.iter().map(|&x| Value::Number(x)).collect();

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
