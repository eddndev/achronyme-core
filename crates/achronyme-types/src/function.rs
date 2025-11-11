use achronyme_parser::ast::AstNode;
use std::collections::HashMap;
use std::rc::Rc;
use crate::environment::Environment;

/// Function representation - can be either a user-defined lambda or a built-in function
///
/// User-defined lambda example:
/// ```text
/// let x = 10;
/// let f = y => x + y;  // Captures 'x' from outer scope
/// f(5)  // Returns 15
/// ```
///
/// Built-in function example:
/// ```text
/// let f = sin;  // References built-in sin function
/// f(0)  // Returns 0
/// ```
///
/// PERFORMANCE OPTIMIZATION: The UserDefined variant now captures an Rc<Environment>
/// instead of a HashMap. This makes closure creation O(1) instead of O(n) where n
/// is the number of variables in scope. This is a MAJOR performance improvement for
/// recursive functions and deeply nested scopes.
#[derive(Debug, Clone)]
pub enum Function {
    /// User-defined lambda with closure
    UserDefined {
        params: Vec<String>,
        body: Rc<AstNode>,
        /// Captured environment - using Rc makes this extremely cheap to clone
        closure_env: Rc<Environment>,
    },
    /// Built-in function by name
    Builtin(String),
}

impl Function {
    /// Create a new user-defined lambda function (DEPRECATED - use new_with_env)
    ///
    /// This method is kept for backward compatibility but creates an Environment
    /// from the HashMap which is less efficient. Prefer `new_with_env`.
    pub fn new(
        params: Vec<String>,
        body: AstNode,
        captured_vars: HashMap<String, crate::value::Value>,
    ) -> Self {
        let env = Environment::from_snapshot(captured_vars);
        Function::UserDefined {
            params,
            body: Rc::new(body),
            closure_env: Rc::new(env),
        }
    }

    /// Create a new user-defined lambda function with an environment (PREFERRED)
    ///
    /// This is the optimized way to create closures - it just increments the Rc counter
    /// instead of copying all variables.
    pub fn new_with_env(
        params: Vec<String>,
        body: AstNode,
        closure_env: Rc<Environment>,
    ) -> Self {
        Function::UserDefined {
            params,
            body: Rc::new(body),
            closure_env,
        }
    }

    /// Create a reference to a built-in function
    pub fn builtin(name: String) -> Self {
        Function::Builtin(name)
    }

    /// Get arity (number of parameters)
    /// Returns None for built-in functions (arity depends on the specific function)
    pub fn arity(&self) -> usize {
        match self {
            Function::UserDefined { params, .. } => params.len(),
            Function::Builtin(_) => 0, // Built-in functions handle their own arity checking
        }
    }

    /// Check if this is a built-in function
    pub fn is_builtin(&self) -> bool {
        matches!(self, Function::Builtin(_))
    }

    /// Get the name of a built-in function, if applicable
    pub fn builtin_name(&self) -> Option<&str> {
        match self {
            Function::Builtin(name) => Some(name),
            _ => None,
        }
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Function::UserDefined { params: p1, body: b1, closure_env: e1 },
             Function::UserDefined { params: p2, body: b2, closure_env: e2 }) => {
                p1 == p2 && Rc::ptr_eq(b1, b2) && Rc::ptr_eq(e1, e2)
            }
            (Function::Builtin(n1), Function::Builtin(n2)) => n1 == n2,
            _ => false,
        }
    }
}
