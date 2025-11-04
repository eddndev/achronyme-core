use achronyme_types::value::Value;
use std::collections::HashMap;

/// Environment for variable storage (Phase 4A)
///
/// Stores variable bindings for let declarations.
/// Supports nested scopes for lambda expressions (Phase 4A+).
///
/// Example:
/// ```
/// use achronyme_eval::Environment;
/// use achronyme_types::Value;
///
/// let mut env = Environment::new();
/// env.define("x".to_string(), Value::Number(5.0));
/// let val = env.get("x").unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct Environment {
    variables: HashMap<String, Value>,
}

impl Environment {
    /// Create a new empty environment
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// Define a new variable in this environment
    ///
    /// # Arguments
    /// * `name` - Variable name
    /// * `value` - Initial value
    ///
    /// # Errors
    /// Returns error if variable already exists
    pub fn define(&mut self, name: String, value: Value) -> Result<(), String> {
        if self.variables.contains_key(&name) {
            return Err(format!("Variable '{}' already declared", name));
        }
        self.variables.insert(name, value);
        Ok(())
    }

    /// Get a variable value
    ///
    /// # Arguments
    /// * `name` - Variable name
    ///
    /// # Returns
    /// Variable value if found
    ///
    /// # Errors
    /// Returns error if variable not found
    pub fn get(&self, name: &str) -> Result<Value, String> {
        self.variables
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Undefined variable '{}'", name))
    }

    /// Check if a variable is defined
    ///
    /// # Arguments
    /// * `name` - Variable name
    ///
    /// # Returns
    /// true if variable exists
    pub fn has(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }

    /// Update an existing variable
    ///
    /// # Arguments
    /// * `name` - Variable name
    /// * `value` - New value
    ///
    /// # Errors
    /// Returns error if variable not found
    pub fn set(&mut self, name: &str, value: Value) -> Result<(), String> {
        if !self.variables.contains_key(name) {
            return Err(format!(
                "Cannot assign to undefined variable '{}'",
                name
            ));
        }
        self.variables.insert(name.to_string(), value);
        Ok(())
    }

    /// Clear all variables
    pub fn clear(&mut self) {
        self.variables.clear();
    }

    /// Get number of variables
    pub fn len(&self) -> usize {
        self.variables.len()
    }

    /// Check if environment is empty
    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }

    /// Get a snapshot of all variables (for lambda closures)
    pub fn snapshot(&self) -> HashMap<String, Value> {
        self.variables.clone()
    }

    /// Create a new environment from a snapshot
    pub fn from_snapshot(snapshot: HashMap<String, Value>) -> Self {
        Self {
            variables: snapshot,
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
    fn test_define_duplicate_fails() {
        let mut env = Environment::new();
        env.define("x".to_string(), Value::Number(5.0)).unwrap();
        let result = env.define("x".to_string(), Value::Number(10.0));
        assert!(result.is_err());
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
}
