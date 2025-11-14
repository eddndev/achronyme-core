use achronyme_types::Environment;
use crate::constants::ConstantsRegistry;
use crate::functions::FunctionRegistry;
use crate::modules::ModuleRegistry;
use achronyme_types::value::Value;
use std::collections::HashMap;

use super::Evaluator;

/// State management methods for Evaluator
impl Evaluator {
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
}
