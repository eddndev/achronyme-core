use crate::value::Value;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

/// Environment for variable storage with scope support using linked list of scopes
///
/// This is a major performance optimization that uses Rc (reference counting) to
/// share environment data instead of cloning it. This is especially important for
/// recursive functions and closures.
///
/// The environment is now a linked list where each scope points to its parent.
/// When a new scope is created, we create a new Environment that references the
/// current environment as its parent using Rc. This makes scope creation O(1)
/// instead of O(n) where n is the total number of variables.
///
/// Example:
/// ```
/// use achronyme_eval::Environment;
/// use achronyme_types::value::Value;
///
/// let mut env = Environment::new();
/// env.define("x".to_string(), Value::Number(5.0));
///
/// // Create new scope for lambda (now cheap!)
/// env.push_scope();
/// env.define("x".to_string(), Value::Number(10.0)); // Shadows outer x
/// assert_eq!(env.get("x").unwrap(), Value::Number(10.0));
///
/// env.pop_scope();
/// assert_eq!(env.get("x").unwrap(), Value::Number(5.0)); // Back to outer x
/// ```
#[derive(Debug, Clone)]
pub struct Environment {
    /// Current scope variables
    variables: HashMap<String, Value>,
    /// Metadata: tracks which variables are mutable
    mutability: HashMap<String, bool>,
    /// Parent environment (if any)
    /// Now uses RefCell to allow mutation of parent scopes
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    /// Create a new empty environment (root scope)
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            mutability: HashMap::new(),
            parent: None,
        }
    }

    /// Create a child environment with this environment as parent
    ///
    /// This is now the primary way to create a new scope. It's O(1) because
    /// we just create a new empty HashMap and an Rc pointer to the parent.
    pub fn new_child(parent: Rc<RefCell<Environment>>) -> Self {
        Self {
            variables: HashMap::new(),
            mutability: HashMap::new(),
            parent: Some(parent),
        }
    }

    /// Push a new scope onto the stack
    ///
    /// This creates a new nested scope by replacing self with a child that
    /// points to the old self. Variables defined after this will be in the
    /// new scope and can shadow outer variables.
    ///
    /// NOTE: This is now implemented by creating a child and swapping.
    pub fn push_scope(&mut self) {
        let parent = Rc::new(RefCell::new(self.clone()));
        *self = Environment::new_child(parent);
    }

    /// Pop the current scope from the stack
    ///
    /// This removes the innermost scope and all variables defined in it.
    /// Panics if trying to pop the root scope.
    pub fn pop_scope(&mut self) {
        if let Some(parent) = self.parent.clone() {
            // Clone the parent out of the Rc<RefCell<>> and replace self
            *self = parent.borrow().clone();
        } else {
            panic!("Cannot pop root scope");
        }
    }

    /// Get the current scope depth (0 = root, 1+ = nested)
    pub fn scope_depth(&self) -> usize {
        let mut depth = 0;
        let mut current_parent = self.parent.clone();
        while let Some(parent) = current_parent {
            depth += 1;
            current_parent = parent.borrow().parent.clone();
        }
        depth
    }

    /// Define a new variable in the current scope (immutable by default)
    ///
    /// With shadowing enabled, this allows redefining variables from outer scopes.
    /// Within the same scope, redefinition is allowed (for `let x = ...; let x = ...`)
    ///
    /// # Arguments
    /// * `name` - Variable name
    /// * `value` - Initial value
    pub fn define(&mut self, name: String, value: Value) -> Result<(), String> {
        self.define_with_mutability(name, value, false)
    }

    /// Define a mutable variable in the current scope
    ///
    /// # Arguments
    /// * `name` - Variable name
    /// * `value` - Initial value
    pub fn define_mutable(&mut self, name: String, value: Value) -> Result<(), String> {
        self.define_with_mutability(name, value, true)
    }

    /// Internal: Define a variable with specified mutability
    fn define_with_mutability(&mut self, name: String, value: Value, is_mutable: bool) -> Result<(), String> {
        // Wrap in MutableRef if mutable
        let stored_value = if is_mutable {
            Value::new_mutable(value)
        } else {
            value
        };

        // Insert into current scope's variables
        self.variables.insert(name.clone(), stored_value);
        self.mutability.insert(name, is_mutable);
        Ok(())
    }

    /// Get a variable value, searching from current to parent scopes
    ///
    /// Auto-dereferences MutableRef values for transparent access
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
        // Check current scope first
        if let Some(value) = self.variables.get(name) {
            // Auto-deref MutableRef for transparent access
            return value.deref();
        }

        // Search parent scopes
        if let Some(ref parent) = self.parent {
            return parent.borrow().get(name);
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
        if self.variables.contains_key(name) {
            return true;
        }

        if let Some(ref parent) = self.parent {
            return parent.borrow().has(name);
        }

        false
    }

    /// Assign a new value to a mutable variable
    ///
    /// Searches current and parent scopes for the variable.
    /// Only works on variables declared with `mut`.
    ///
    /// # Arguments
    /// * `name` - Variable name
    /// * `value` - New value
    ///
    /// # Errors
    /// Returns error if:
    /// - Variable not found in any scope
    /// - Variable is immutable (not declared with `mut`)
    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        // Check current scope first
        if let Some(var_value) = self.variables.get(name) {
            let is_mutable = self.mutability.get(name).copied().unwrap_or(false);

            if !is_mutable {
                return Err(format!("Cannot assign to immutable variable '{}'", name));
            }

            // Assign to MutableRef
            var_value.assign(value)?;
            return Ok(());
        }

        // Search and mutate in parent scopes
        if let Some(ref parent) = self.parent {
            return parent.borrow_mut().assign(name, value);
        }

        Err(format!("Undefined variable '{}'", name))
    }

    /// Update an existing variable in the scope where it was defined
    ///
    /// # Arguments
    /// * `name` - Variable name
    /// * `value` - New value
    ///
    /// # Errors
    /// Returns error if variable not found in any scope
    ///
    /// NOTE: This is more complex with the linked structure because we need
    /// to modify the parent, which is behind an Rc. For now, we only allow
    /// setting variables in the current scope. This matches the semantics
    /// of most languages where assignment creates a new binding if it doesn't
    /// exist in the current scope.
    pub fn set(&mut self, name: &str, value: Value) -> Result<(), String> {
        // Check if variable exists in current scope
        if self.variables.contains_key(name) {
            self.variables.insert(name.to_string(), value);
            return Ok(());
        }

        // Check if it exists in parent scopes
        if let Some(ref parent) = self.parent {
            if parent.borrow().has(name) {
                // Variable exists in parent, but we shadow it in the current scope
                self.variables.insert(name.to_string(), value);
                return Ok(());
            }
        }

        Err(format!("Cannot assign to undefined variable '{}'", name))
    }

    /// Clear all variables in the current scope only
    pub fn clear(&mut self) {
        self.variables.clear();
        self.mutability.clear();
        self.parent = None;
    }

    /// Get total number of variables in current scope only
    /// (counting parent variables would require traversing the chain)
    pub fn len(&self) -> usize {
        self.variables.len()
    }

    /// Check if current scope has no variables
    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }

    /// Get a snapshot of all visible variables (for lambda closures)
    ///
    /// This flattens the scope chain, with inner scopes overriding outer ones.
    /// Used when creating a closure to capture the current environment.
    ///
    /// DEPRECATED: This is kept for backward compatibility but is expensive.
    /// New code should use `to_rc()` to capture the environment as Rc<RefCell<Environment>>.
    pub fn snapshot(&self) -> HashMap<String, Value> {
        let mut snapshot = HashMap::new();

        // Collect from parent first (so current scope can override)
        if let Some(ref parent) = self.parent {
            snapshot = parent.borrow().snapshot();
        }

        // Add/override with current scope (deref MutableRef values)
        for (name, value) in &self.variables {
            if let Ok(derefed) = value.deref() {
                snapshot.insert(name.clone(), derefed);
            } else {
                snapshot.insert(name.clone(), value.clone());
            }
        }

        snapshot
    }

    /// Create a new environment from a snapshot (single root scope)
    ///
    /// Used when restoring a closure's captured environment.
    ///
    /// DEPRECATED: This is kept for backward compatibility.
    pub fn from_snapshot(snapshot: HashMap<String, Value>) -> Self {
        Self {
            variables: snapshot,
            mutability: HashMap::new(), // No mutability info in snapshot
            parent: None,
        }
    }

    /// Convert this environment to an Rc<RefCell<>> for efficient sharing
    ///
    /// This is the preferred way to capture an environment for closures.
    pub fn to_rc(&self) -> Rc<RefCell<Environment>> {
        Rc::new(RefCell::new(self.clone()))
    }

    /// Create a new environment with a specific parent
    pub fn with_parent(parent: Rc<RefCell<Environment>>) -> Self {
        Self::new_child(parent)
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
        env.set("x", Value::Number(10.0)).unwrap(); // Shadows in current scope (new semantics)

        // In current scope, we see the new value
        assert_eq!(env.get("x").unwrap(), Value::Number(10.0));

        env.pop_scope();
        // After popping, we see the original value (shadowing, not mutation)
        assert_eq!(env.get("x").unwrap(), Value::Number(5.0));
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
