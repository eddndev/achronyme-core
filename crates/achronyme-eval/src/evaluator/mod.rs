//! Evaluator for SOC (Scientific Operations Calculator)
//!
//! The evaluator walks the AST and computes results using a post-order traversal
//! (visit children before parent).
//!
//! Example evaluation order:
//! ```text
//!       +
//!      / \
//!     2   *
//!        / \
//!       3   4
//!
//! Evaluation order:
//!   1. eval(2) → 2
//!   2. eval(3) → 3
//!   3. eval(4) → 4
//!   4. eval(3*4) → 12
//!   5. eval(2+12) → 14
//! ```

use achronyme_types::Environment;
use achronyme_types::value::Value;
use std::collections::HashMap;

use crate::constants::ConstantsRegistry;
use crate::functions::FunctionRegistry;
use crate::modules::{ModuleRegistry, create_builtin_registry};

// Module declarations
mod state;
mod modules;
mod lambda_eval;
mod dispatcher;

/// Evaluator
///
/// Walks the AST and computes the result.
pub struct Evaluator {
    pub(crate) env: Environment,
    pub(crate) constants: ConstantsRegistry,
    pub(crate) functions: FunctionRegistry,
    /// Module registry for organizing functions into modules
    pub(crate) module_registry: ModuleRegistry,
    /// Track which modules have been imported
    /// Format: local_name -> (module_name, original_name)
    pub(crate) imported_modules: HashMap<String, (String, String)>,
    /// Track exported values from current module (for user-defined modules)
    /// Format: name -> Value
    pub(crate) exported_values: HashMap<String, Value>,
    /// Cache of loaded user modules to avoid re-parsing
    /// Format: module_path -> HashMap<name, Value>
    pub(crate) module_cache: HashMap<String, HashMap<String, Value>>,
    /// Current file being evaluated (for relative imports)
    /// This is the directory path of the file currently being evaluated
    pub(crate) current_file_dir: Option<String>,
    /// Flag to enable tail call optimization mode
    /// When true, CallExpression with rec will return TailCall markers
    pub(crate) tco_mode: bool,
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
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}
