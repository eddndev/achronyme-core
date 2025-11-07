use achronyme_types::value::Value;
use std::collections::HashMap;

/// Environment for variable storage with scope support
///
/// Stores variable bindings with a stack-based scope system.
/// This enables:
/// - Variable shadowing (inner scopes can redefine outer variables)
/// - Lambda parameter scoping
/// - Proper closure capture
///
/// Example:
/// ```
/// use achronyme_eval::Environment;
/// use achronyme_types::value::Value;
///
/// let mut env = Environment::new();
/// env.define("x".to_string(), Value::Number(5.0));
///
/// // Create new scope for lambda
/// env.push_scope();
/// env.define("x".to_string(), Value::Number(10.0)); // Shadows outer x
/// assert_eq!(env.get("x").unwrap(), Value::Number(10.0));
///
/// env.pop_scope();
/// assert_eq!(env.get("x").unwrap(), Value::Number(5.0)); // Back to outer x
/// ```
#[derive(Debug, Clone)]
pub struct Environment {
    /// Stack of scopes, where each scope is a HashMap
    /// Index 0 is the global scope, higher indices are nested scopes
    scopes: Vec<HashMap<String, Value>>,
}

impl Environment {
    /// Create a new empty environment with one global scope
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    /// Push a new scope onto the stack
    ///
    /// This creates a new nested scope. Variables defined after this
    /// will be in the new scope and can shadow outer variables.
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Pop the current scope from the stack
    ///
    /// This removes the innermost scope and all variables defined in it.
    /// Panics if trying to pop the global scope.
    pub fn pop_scope(&mut self) {
        if self.scopes.len() <= 1 {
            panic!("Cannot pop global scope");
        }
        self.scopes.pop();
    }

    /// Get the current scope depth (0 = global, 1+ = nested)
    pub fn scope_depth(&self) -> usize {
        self.scopes.len() - 1
    }

    /// Define a new variable in the current scope
    ///
    /// With shadowing enabled, this allows redefining variables from outer scopes.
    /// Within the same scope, redefinition is allowed (for `let x = ...; let x = ...`)
    ///
    /// # Arguments
    /// * `name` - Variable name
    /// * `value` - Initial value
    pub fn define(&mut self, name: String, value: Value) -> Result<(), String> {
        // Get current (innermost) scope
        let current_scope = self.scopes.last_mut().unwrap();

        // Allow redefinition in the same scope (shadowing)
        current_scope.insert(name, value);
        Ok(())
    }

    /// Get a variable value, searching from innermost to outermost scope
    ///
    /// # Arguments
    /// * `name` - Variable name
    ///
    /// # Returns
    /// Variable value if found in any scope
    ///
    /// # Errors
    /// Returns error if variable not found in any scope
    pub fn get(&self, name: &str) -> Result<Value, String> {
        // Search from innermost to outermost scope
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value.clone());
            }
        }
        Err(format!("Undefined variable '{}'", name))
    }

    /// Check if a variable is defined in any scope
    ///
    /// # Arguments
    /// * `name` - Variable name
    ///
    /// # Returns
    /// true if variable exists in any scope
    pub fn has(&self, name: &str) -> bool {
        self.scopes.iter().rev().any(|scope| scope.contains_key(name))
    }

    /// Update an existing variable in the scope where it was defined
    ///
    /// # Arguments
    /// * `name` - Variable name
    /// * `value` - New value
    ///
    /// # Errors
    /// Returns error if variable not found in any scope
    pub fn set(&mut self, name: &str, value: Value) -> Result<(), String> {
        // Search from innermost to outermost scope
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return Ok(());
            }
        }
        Err(format!("Cannot assign to undefined variable '{}'", name))
    }

    /// Clear all variables in all scopes
    pub fn clear(&mut self) {
        self.scopes.clear();
        self.scopes.push(HashMap::new());
    }

    /// Get total number of variables across all scopes
    pub fn len(&self) -> usize {
        self.scopes.iter().map(|scope| scope.len()).sum()
    }

    /// Check if environment has no variables in any scope
    pub fn is_empty(&self) -> bool {
        self.scopes.iter().all(|scope| scope.is_empty())
    }

    /// Get a snapshot of all visible variables (for lambda closures)
    ///
    /// This flattens the scope stack, with inner scopes overriding outer ones.
    /// Used when creating a closure to capture the current environment.
    pub fn snapshot(&self) -> HashMap<String, Value> {
        let mut snapshot = HashMap::new();

        // Iterate from outermost to innermost, so inner scopes override
        for scope in self.scopes.iter() {
            for (name, value) in scope {
                snapshot.insert(name.clone(), value.clone());
            }
        }

        snapshot
    }

    /// Create a new environment from a snapshot (single global scope)
    ///
    /// Used when restoring a closure's captured environment.
    pub fn from_snapshot(snapshot: HashMap<String, Value>) -> Self {
        Self {
            scopes: vec![snapshot],
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_and_get() {
        let mut env = Environment::new();
        env.define("x".to_string(), Value::Number(5.0)).unwrap();
        let val = env.get("x").unwrap();
        assert_eq!(val, Value::Number(5.0));
    }

    #[test]
    fn test_shadowing_in_new_scope() {
        let mut env = Environment::new();

        // Define in global scope
        env.define("x".to_string(), Value::Number(5.0)).unwrap();
        assert_eq!(env.get("x").unwrap(), Value::Number(5.0));

        // Push new scope and shadow
        env.push_scope();
        env.define("x".to_string(), Value::Number(10.0)).unwrap();
        assert_eq!(env.get("x").unwrap(), Value::Number(10.0));

        // Pop scope, back to original
        env.pop_scope();
        assert_eq!(env.get("x").unwrap(), Value::Number(5.0));
    }

    #[test]
    fn test_shadowing_in_same_scope() {
        let mut env = Environment::new();

        // First definition
        env.define("x".to_string(), Value::Number(5.0)).unwrap();
        assert_eq!(env.get("x").unwrap(), Value::Number(5.0));

        // Redefine in same scope (should work now)
        env.define("x".to_string(), Value::Number(10.0)).unwrap();
        assert_eq!(env.get("x").unwrap(), Value::Number(10.0));
    }

    #[test]
    fn test_nested_scopes() {
        let mut env = Environment::new();

        // Global: x=1, y=2
        env.define("x".to_string(), Value::Number(1.0)).unwrap();
        env.define("y".to_string(), Value::Number(2.0)).unwrap();

        // Level 1: x=10 (shadows), z=3
        env.push_scope();
        env.define("x".to_string(), Value::Number(10.0)).unwrap();
        env.define("z".to_string(), Value::Number(3.0)).unwrap();
        assert_eq!(env.get("x").unwrap(), Value::Number(10.0));
        assert_eq!(env.get("y").unwrap(), Value::Number(2.0)); // From global
        assert_eq!(env.get("z").unwrap(), Value::Number(3.0));

        // Level 2: y=20 (shadows)
        env.push_scope();
        env.define("y".to_string(), Value::Number(20.0)).unwrap();
        assert_eq!(env.get("x").unwrap(), Value::Number(10.0)); // From level 1
        assert_eq!(env.get("y").unwrap(), Value::Number(20.0)); // Current level
        assert_eq!(env.get("z").unwrap(), Value::Number(3.0));  // From level 1

        // Pop to level 1
        env.pop_scope();
        assert_eq!(env.get("y").unwrap(), Value::Number(2.0)); // Back to global

        // Pop to global
        env.pop_scope();
        assert_eq!(env.get("x").unwrap(), Value::Number(1.0));
        assert!(env.get("z").is_err()); // z no longer exists
    }

    #[test]
    fn test_get_undefined_fails() {
        let env = Environment::new();
        let result = env.get("x");
        assert!(result.is_err());
    }

    #[test]
    fn test_has() {
        let mut env = Environment::new();
        assert!(!env.has("x"));
        env.define("x".to_string(), Value::Number(5.0)).unwrap();
        assert!(env.has("x"));

        // Should find in outer scope too
        env.push_scope();
        assert!(env.has("x"));
    }

    #[test]
    fn test_set_existing() {
        let mut env = Environment::new();
        env.define("x".to_string(), Value::Number(5.0)).unwrap();
        env.set("x", Value::Number(10.0)).unwrap();
        let val = env.get("x").unwrap();
        assert_eq!(val, Value::Number(10.0));
    }

    #[test]
    fn test_set_in_outer_scope() {
        let mut env = Environment::new();
        env.define("x".to_string(), Value::Number(5.0)).unwrap();

        env.push_scope();
        env.set("x", Value::Number(10.0)).unwrap(); // Should modify outer scope

        env.pop_scope();
        assert_eq!(env.get("x").unwrap(), Value::Number(10.0));
    }

    #[test]
    fn test_set_undefined_fails() {
        let mut env = Environment::new();
        let result = env.set("x", Value::Number(5.0));
        assert!(result.is_err());
    }

    #[test]
    fn test_clear() {
        let mut env = Environment::new();
        env.define("x".to_string(), Value::Number(5.0)).unwrap();
        env.define("y".to_string(), Value::Number(10.0)).unwrap();
        assert_eq!(env.len(), 2);
        env.clear();
        assert_eq!(env.len(), 0);
        assert!(!env.has("x"));
    }

    #[test]
    fn test_snapshot_flattens_scopes() {
        let mut env = Environment::new();

        // Global: x=1, y=2
        env.define("x".to_string(), Value::Number(1.0)).unwrap();
        env.define("y".to_string(), Value::Number(2.0)).unwrap();

        // Nested: x=10 (shadows), z=3
        env.push_scope();
        env.define("x".to_string(), Value::Number(10.0)).unwrap();
        env.define("z".to_string(), Value::Number(3.0)).unwrap();

        let snapshot = env.snapshot();
        assert_eq!(snapshot.get("x").unwrap(), &Value::Number(10.0)); // Inner value
        assert_eq!(snapshot.get("y").unwrap(), &Value::Number(2.0));
        assert_eq!(snapshot.get("z").unwrap(), &Value::Number(3.0));
    }

    #[test]
    fn test_scope_depth() {
        let mut env = Environment::new();
        assert_eq!(env.scope_depth(), 0);

        env.push_scope();
        assert_eq!(env.scope_depth(), 1);

        env.push_scope();
        assert_eq!(env.scope_depth(), 2);

        env.pop_scope();
        assert_eq!(env.scope_depth(), 1);
    }
}
