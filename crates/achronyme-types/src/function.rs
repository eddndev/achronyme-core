use achronyme_parser::ast::AstNode;
use std::collections::HashMap;
use std::rc::Rc;

/// Lambda function with closure support
///
/// Stores:
/// - params: Parameter names
/// - body: AST of the function body (shared ownership for efficient cloning)
/// - captured_vars: Variables captured from the surrounding scope
///
/// Example:
/// ```text
/// let x = 10;
/// let f = y => x + y;  // Captures 'x' from outer scope
/// f(5)  // Returns 15
/// ```
#[derive(Debug, Clone)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Rc<AstNode>,
    pub captured_vars: HashMap<String, crate::value::Value>,
}

impl Function {
    /// Create a new lambda function
    pub fn new(
        params: Vec<String>,
        body: AstNode,
        captured_vars: HashMap<String, crate::value::Value>,
    ) -> Self {
        Self {
            params,
            body: Rc::new(body),
            captured_vars,
        }
    }

    /// Get arity (number of parameters)
    pub fn arity(&self) -> usize {
        self.params.len()
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        // Functions are equal if they have the same params and body
        // (captured_vars compared by value)
        self.params == other.params
            && Rc::ptr_eq(&self.body, &other.body)
            && self.captured_vars == other.captured_vars
    }
}
