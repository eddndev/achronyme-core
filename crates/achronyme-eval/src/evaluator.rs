use achronyme_parser::ast::AstNode;
use achronyme_types::function::Function;
use achronyme_types::value::Value;
use achronyme_types::LambdaEvaluator;
use achronyme_types::Environment;

use std::collections::HashMap;

use crate::constants::ConstantsRegistry;
use crate::functions::FunctionRegistry;
use crate::modules::{ModuleRegistry, create_builtin_registry};
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
    /// Module registry for organizing functions into modules
    module_registry: ModuleRegistry,
    /// Track which modules have been imported
    /// Format: local_name -> (module_name, original_name)
    imported_modules: HashMap<String, (String, String)>,
    /// Track exported values from current module (for user-defined modules)
    /// Format: name -> Value
    exported_values: HashMap<String, Value>,
    /// Cache of loaded user modules to avoid re-parsing
    /// Format: module_path -> HashMap<name, Value>
    module_cache: HashMap<String, HashMap<String, Value>>,
    /// Current file being evaluated (for relative imports)
    /// This is the directory path of the file currently being evaluated
    current_file_dir: Option<String>,
    /// Flag to enable tail call optimization mode
    /// When true, CallExpression with rec will return TailCall markers
    tco_mode: bool,
}

impl Evaluator {
    /// Create a new evaluator
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
            constants: ConstantsRegistry::new(),
            functions: FunctionRegistry::new(),
            module_registry: create_builtin_registry(),
            imported_modules: HashMap::new(),
            exported_values: HashMap::new(),
            module_cache: HashMap::new(),
            current_file_dir: None,
            tco_mode: false,
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

    /// Get the module registry
    pub fn module_registry(&self) -> &ModuleRegistry {
        &self.module_registry
    }

    /// Get the imported modules map
    pub fn imported_modules(&self) -> &HashMap<String, (String, String)> {
        &self.imported_modules
    }

    /// Get the exported values map
    pub fn exported_values(&self) -> &HashMap<String, Value> {
        &self.exported_values
    }

    /// Check if TCO mode is enabled
    pub fn is_tco_mode(&self) -> bool {
        self.tco_mode
    }

    /// Set TCO mode (used by tail-recursive function execution)
    pub fn set_tco_mode(&mut self, enabled: bool) {
        self.tco_mode = enabled;
    }

    /// Set the current file directory (for relative imports)
    /// This should be called when loading a file from disk
    pub fn set_current_file_dir(&mut self, file_path: &str) {
        use std::path::Path;

        if let Some(parent) = Path::new(file_path).parent() {
            self.current_file_dir = Some(parent.to_string_lossy().to_string());
        }
    }

    /// Load and evaluate a user module from a file path
    /// Returns the exported values from the module
    pub fn load_user_module(&mut self, module_path: &str) -> Result<HashMap<String, Value>, String> {
        use std::fs;
        use std::path::Path;
        use achronyme_parser::parse;

        // Check cache first
        if let Some(cached_exports) = self.module_cache.get(module_path) {
            return Ok(cached_exports.clone());
        }

        // Add .soc extension if missing
        let module_path_with_ext = if module_path.ends_with(".soc") {
            module_path.to_string()
        } else {
            format!("{}.soc", module_path)
        };

        // Resolve path relative to current file directory
        let resolved_path = if let Some(ref current_dir) = self.current_file_dir {
            // If we have a current file directory, resolve relative to it
            let base_path = Path::new(current_dir);
            let module_file = Path::new(&module_path_with_ext);
            base_path.join(module_file)
                .to_string_lossy()
                .to_string()
        } else {
            // No current file context, use path as-is (relative to cwd)
            module_path_with_ext
        };

        // Read the file
        let file_content = fs::read_to_string(&resolved_path)
            .map_err(|e| format!("Failed to read module '{}': {}", resolved_path, e))?;

        // Parse the module
        let statements = parse(&file_content)?;

        // Save the current file directory and set new one for this module
        let old_file_dir = self.current_file_dir.clone();
        let module_dir = Path::new(&resolved_path)
            .parent()
            .map(|p| p.to_string_lossy().to_string());
        self.current_file_dir = module_dir;

        // Create a new scope for the module
        self.env.push_scope();

        // Evaluate all statements in the module
        for stmt in &statements {
            self.evaluate(stmt)?;
        }

        // Collect exported values from this module
        let module_exports = self.exported_values.clone();

        // Pop the module scope
        self.env.pop_scope();

        // Restore the previous file directory
        self.current_file_dir = old_file_dir;

        // Clear exported values (they've been captured)
        self.exported_values.clear();

        // Cache the module
        self.module_cache.insert(module_path.to_string(), module_exports.clone());

        Ok(module_exports)
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
            AstNode::MutableDecl { name, initializer } => {
                handlers::variables::evaluate_mutable_declaration(self, name, initializer)
            }
            AstNode::Assignment { target, value } => {
                handlers::assignment::evaluate_assignment(self, target, value)
            }
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
                        let field_value = map.get(field)
                            .cloned()
                            .ok_or_else(|| format!("Field '{}' not found in record", field))?;
                        // Auto-deref MutableRef when accessing fields
                        field_value.deref()
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

                // TCO OPTIMIZATION: Check if this is a tail call to 'rec' in TCO mode
                if self.is_tco_mode() && matches!(callee.as_ref(), AstNode::RecReference) {
                    // This is a tail call to rec - return TailCall marker instead of calling
                    let mut arg_values = Vec::new();
                    for arg in args {
                        arg_values.push(self.evaluate(arg)?);
                    }
                    return Ok(Value::TailCall(arg_values));
                }

                // If callee is a VariableRef, it might be a built-in function
                // Dispatch to function_call handler which checks module registry
                if let AstNode::VariableRef(name) = callee.as_ref() {
                    return handlers::function_call::dispatch(self, name, args);
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

            // Sequence: multiple statements separated by semicolons
            // Example: let a = 1; let b = 2; a + b
            // The last statement is the value of the sequence
            AstNode::Sequence { statements } => {
                if statements.is_empty() {
                    return Err("Empty sequence".to_string());
                }

                // Create a new scope for the sequence
                self.env.push_scope();

                // Evaluate all statements, keeping the last result
                // The result can be any type: Number, String, Vector, Function, etc.
                let mut result = None;
                for stmt in statements {
                    result = Some(self.evaluate(stmt)?);
                }

                // Pop the scope
                self.env.pop_scope();

                // Return the last evaluated expression
                result.ok_or_else(|| "Empty sequence (no statements)".to_string())
            }

            // DoBlock: do { statements }
            // Example: x => do { let a = x * 2; a + 10 }
            // Can return any type: Number, Vector, String, Function, Record, etc.
            AstNode::DoBlock { statements } => {
                if statements.is_empty() {
                    return Err("Empty do block".to_string());
                }

                // Create a new scope for the do block
                self.env.push_scope();

                // Evaluate all statements, keeping the last result
                // The result can be any type: Number, String, Vector, Function, etc.
                let mut result = None;
                for stmt in statements {
                    result = Some(self.evaluate(stmt)?);
                }

                // Pop the scope
                self.env.pop_scope();

                // Return the last evaluated expression
                result.ok_or_else(|| "Empty do block (no statements)".to_string())
            }

            // Module system
            AstNode::Import { items, module_path } => {
                // Check if this is a built-in module or a file-based user module
                if self.module_registry.has_module(module_path) {
                    // Built-in module: add to imported_modules map
                    for item in items {
                        let local_name = item.local_name();
                        let original_name = &item.name;

                        // Check if the function exists in the module
                        let module = self.module_registry.get_module(module_path).unwrap();
                        if !module.has(original_name) {
                            return Err(format!(
                                "Function '{}' not found in module '{}'",
                                original_name, module_path
                            ));
                        }

                        self.imported_modules.insert(
                            local_name.to_string(),
                            (module_path.clone(), original_name.clone())
                        );
                    }
                } else {
                    // User-defined module: load from file and import exported values
                    let exports = self.load_user_module(module_path)?;

                    for item in items {
                        let local_name = item.local_name();
                        let original_name = &item.name;

                        // Check if the value is exported from the module
                        let value = exports.get(original_name).ok_or_else(|| {
                            format!(
                                "'{}' is not exported from module '{}'",
                                original_name, module_path
                            )
                        })?;

                        // Add the imported value to the environment
                        // This allows both variables and functions to be imported
                        self.env.define(local_name.to_string(), value.clone())?;
                    }
                }

                // Import statements don't produce a value, return unit (true)
                Ok(Value::Boolean(true))
            }

            AstNode::Export { items } => {
                // Export statement: marks variables/functions for external use
                // export { mean, std, variance }

                for item in items {
                    let name = &item.name;
                    let export_name = item.local_name(); // Use alias if provided

                    // Check if the value exists in the environment
                    if !self.env.has(name) {
                        return Err(format!(
                            "Cannot export '{}': not found in current scope",
                            name
                        ));
                    }

                    // Get the value from environment
                    let value = self.env.get(name)?;

                    // Add to exported values
                    self.exported_values.insert(export_name.to_string(), value);
                }

                // Export statements don't produce a value, return unit (true)
                Ok(Value::Boolean(true))
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
