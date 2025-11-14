use achronyme_types::value::Value;
use std::collections::HashMap;

use super::Evaluator;

/// Module loading and management
impl Evaluator {
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
}
