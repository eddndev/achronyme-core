// ============================================================================
// Module System - Core Infrastructure
// ============================================================================
// This module provides the infrastructure for organizing built-in functions
// into modules and managing imports.
//
// Design:
// - Prelude: ~39 functions always available globally
// - Modules: Organized groups of functions requiring explicit import
// - Module Registry: Maps module names to their exported functions
// ============================================================================

use std::collections::HashMap;
use crate::functions::BuiltinFunction;

pub mod builtin_registry;
pub use builtin_registry::create_builtin_registry;

/// Represents a single module with its exported functions
#[derive(Clone)]
pub struct Module {
    /// Module name (e.g., "math", "stats", "dsp")
    pub name: String,

    /// Functions exported by this module
    /// Key: function name, Value: (function pointer, arity)
    pub exports: HashMap<String, (BuiltinFunction, i32)>,
}

impl Module {
    /// Create a new empty module
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            exports: HashMap::new(),
        }
    }

    /// Register a function in this module
    pub fn register(&mut self, name: &str, func: BuiltinFunction, arity: i32) {
        self.exports.insert(name.to_string(), (func, arity));
    }

    /// Check if this module exports a function
    pub fn has(&self, name: &str) -> bool {
        self.exports.contains_key(name)
    }

    /// Get a function from this module
    pub fn get(&self, name: &str) -> Option<(BuiltinFunction, i32)> {
        self.exports.get(name).copied()
    }

    /// Get all exported function names
    pub fn function_names(&self) -> Vec<String> {
        self.exports.keys().cloned().collect()
    }
}

/// Registry for all built-in modules
///
/// Organizes built-in functions into:
/// - Prelude: Always available functions (no import needed)
/// - Modules: Domain-specific functions (require import)
#[derive(Clone)]
pub struct ModuleRegistry {
    /// Functions in the prelude (always available)
    /// These are the ~39 core functions that don't require imports
    prelude: HashMap<String, (BuiltinFunction, i32)>,

    /// Modules requiring explicit import
    /// Key: module name (e.g., "math", "stats")
    /// Value: Module containing functions
    modules: HashMap<String, Module>,
}

impl ModuleRegistry {
    /// Create a new empty module registry
    pub fn new() -> Self {
        Self {
            prelude: HashMap::new(),
            modules: HashMap::new(),
        }
    }

    /// Register a function in the prelude (always available)
    pub fn register_prelude(&mut self, name: &str, func: BuiltinFunction, arity: i32) {
        self.prelude.insert(name.to_string(), (func, arity));
    }

    /// Register a module
    pub fn register_module(&mut self, module: Module) {
        self.modules.insert(module.name.clone(), module);
    }

    /// Check if a name is in the prelude
    pub fn is_prelude(&self, name: &str) -> bool {
        self.prelude.contains_key(name)
    }

    /// Get a function from the prelude
    pub fn get_prelude(&self, name: &str) -> Option<(BuiltinFunction, i32)> {
        self.prelude.get(name).copied()
    }

    /// Get a module by name
    pub fn get_module(&self, name: &str) -> Option<&Module> {
        self.modules.get(name)
    }

    /// Check if a module exists
    pub fn has_module(&self, name: &str) -> bool {
        self.modules.contains_key(name)
    }

    /// Resolve a function name
    ///
    /// Resolution order:
    /// 1. Check prelude (always available)
    /// 2. Check imported modules
    /// 3. [Temporary] Check all modules (backward compatibility)
    ///
    /// # Arguments
    /// * `name` - The function name to resolve
    /// * `imported_modules` - Map of local_name -> (module_name, original_name)
    pub fn resolve(
        &self,
        name: &str,
        imported_modules: &HashMap<String, (String, String)>,
    ) -> Option<(BuiltinFunction, i32)> {
        // 1. Check prelude first (always available)
        if let Some(func) = self.prelude.get(name) {
            return Some(*func);
        }

        // 2. Check imported modules
        // imported_modules maps: local_name -> (module_name, original_name)
        if let Some((module_name, original_name)) = imported_modules.get(name) {
            if let Some(module) = self.modules.get(module_name) {
                return module.get(original_name);
            }
        }

        // 3. [REMOVED - Phase 3 complete]
        // Global fallback was removed - all functions now require explicit imports
        // (except prelude which is always available)

        None
    }

    /// Get all module names
    pub fn module_names(&self) -> Vec<String> {
        self.modules.keys().cloned().collect()
    }

    /// Get all prelude function names
    pub fn prelude_names(&self) -> Vec<String> {
        self.prelude.keys().cloned().collect()
    }

    /// Get total number of functions (prelude + all modules)
    pub fn total_functions(&self) -> usize {
        let module_count: usize = self.modules.values()
            .map(|m| m.exports.len())
            .sum();
        self.prelude.len() + module_count
    }
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Dummy function for testing
    fn dummy_func(_args: &[achronyme_types::value::Value], _env: &mut achronyme_types::Environment) -> Result<achronyme_types::value::Value, String> {
        Ok(achronyme_types::value::Value::Number(42.0))
    }

    #[test]
    fn test_module_creation() {
        let mut module = Module::new("math");
        assert_eq!(module.name, "math");
        assert_eq!(module.exports.len(), 0);

        module.register("sin", dummy_func, 1);
        assert!(module.has("sin"));
        assert!(!module.has("cos"));
        assert_eq!(module.function_names().len(), 1);
    }

    #[test]
    fn test_registry_prelude() {
        let mut registry = ModuleRegistry::new();
        registry.register_prelude("sin", dummy_func, 1);
        registry.register_prelude("cos", dummy_func, 1);

        assert!(registry.is_prelude("sin"));
        assert!(registry.is_prelude("cos"));
        assert!(!registry.is_prelude("mean"));

        assert_eq!(registry.prelude_names().len(), 2);
    }

    #[test]
    fn test_registry_modules() {
        let mut registry = ModuleRegistry::new();

        let mut math_module = Module::new("math");
        math_module.register("asin", dummy_func, 1);
        math_module.register("acos", dummy_func, 1);

        registry.register_module(math_module);

        assert!(registry.has_module("math"));
        assert!(!registry.has_module("stats"));

        let module = registry.get_module("math").unwrap();
        assert_eq!(module.function_names().len(), 2);
    }

    #[test]
    fn test_resolve_prelude() {
        let mut registry = ModuleRegistry::new();
        registry.register_prelude("sin", dummy_func, 1);

        let imports = HashMap::new();
        let result = registry.resolve("sin", &imports);

        assert!(result.is_some());
        let (_, arity) = result.unwrap();
        assert_eq!(arity, 1);
    }

    #[test]
    fn test_resolve_imported() {
        let mut registry = ModuleRegistry::new();

        let mut stats_module = Module::new("stats");
        stats_module.register("mean", dummy_func, 1);
        registry.register_module(stats_module);

        // Simulate import: import { mean } from "stats"
        let mut imports = HashMap::new();
        imports.insert("mean".to_string(), ("stats".to_string(), "mean".to_string()));

        let result = registry.resolve("mean", &imports);
        assert!(result.is_some());
    }

    #[test]
    fn test_resolve_aliased() {
        let mut registry = ModuleRegistry::new();

        let mut stats_module = Module::new("stats");
        stats_module.register("mean", dummy_func, 1);
        registry.register_module(stats_module);

        // Simulate import: import { mean as average } from "stats"
        let mut imports = HashMap::new();
        imports.insert("average".to_string(), ("stats".to_string(), "mean".to_string()));

        // Resolve using the alias
        let result = registry.resolve("average", &imports);
        assert!(result.is_some());

        // Original name shouldn't work without being imported
        let result = registry.resolve("mean", &imports);
        assert!(result.is_none());
    }

    #[test]
    fn test_prelude_takes_precedence() {
        let mut registry = ModuleRegistry::new();

        // Register in both prelude and a module
        registry.register_prelude("sum", dummy_func, 1);

        let mut custom_module = Module::new("custom");
        custom_module.register("sum", dummy_func, 1);
        registry.register_module(custom_module);

        let imports = HashMap::new();

        // Prelude should take precedence
        let result = registry.resolve("sum", &imports);
        assert!(result.is_some());
    }

    #[test]
    fn test_total_functions() {
        let mut registry = ModuleRegistry::new();

        registry.register_prelude("sin", dummy_func, 1);
        registry.register_prelude("cos", dummy_func, 1);

        let mut math_module = Module::new("math");
        math_module.register("asin", dummy_func, 1);
        registry.register_module(math_module);

        assert_eq!(registry.total_functions(), 3);
    }
}
