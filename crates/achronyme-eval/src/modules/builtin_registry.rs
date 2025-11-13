// ============================================================================
// Built-in Function Organization
// ============================================================================
// This module organizes all built-in functions into:
// - Prelude: 39 core functions always available
// - Modules: Domain-specific functions requiring import
// ============================================================================

use super::{Module, ModuleRegistry};
use crate::functions::FunctionRegistry;

/// Create and populate a module registry with all built-in functions
///
/// Organizes functions into:
/// - Prelude (39 functions): Core functions always available
/// - Modules (60+ functions): Domain-specific functions
pub fn create_builtin_registry() -> ModuleRegistry {
    let mut registry = ModuleRegistry::new();

    // Create a temporary FunctionRegistry to access all built-in functions
    let func_registry = FunctionRegistry::new();

    // Register prelude functions (always available)
    register_prelude(&mut registry, &func_registry);

    // Register modules (require import)
    register_math_module(&mut registry, &func_registry);
    register_stats_module(&mut registry, &func_registry);
    register_linalg_module(&mut registry, &func_registry);
    register_dsp_module(&mut registry, &func_registry);
    register_numerical_module(&mut registry, &func_registry);
    register_graph_module(&mut registry, &func_registry);
    register_pert_module(&mut registry, &func_registry);
    register_optimization_module(&mut registry, &func_registry);
    register_complex_module(&mut registry, &func_registry);
    register_strings_module(&mut registry, &func_registry);
    register_arrays_module(&mut registry, &func_registry);
    register_records_module(&mut registry, &func_registry);

    registry
}

// ============================================================================
// Prelude Registration (39 functions)
// ============================================================================

fn register_prelude(registry: &mut ModuleRegistry, func_registry: &FunctionRegistry) {
    // === MATHEMATICS (15) ===
    // Basic trigonometry
    register_if_exists(registry, func_registry, "sin");
    register_if_exists(registry, func_registry, "cos");
    register_if_exists(registry, func_registry, "tan");

    // Exponential & roots
    register_if_exists(registry, func_registry, "sqrt");
    register_if_exists(registry, func_registry, "exp");
    register_if_exists(registry, func_registry, "ln");
    register_if_exists(registry, func_registry, "pow");

    // Rounding
    register_if_exists(registry, func_registry, "abs");
    register_if_exists(registry, func_registry, "floor");
    register_if_exists(registry, func_registry, "ceil");
    register_if_exists(registry, func_registry, "round");

    // Comparison
    register_if_exists(registry, func_registry, "min");
    register_if_exists(registry, func_registry, "max");

    // Constants (handled separately in evaluator via constants registry)
    // pi, e

    // === ARRAYS & HOF (14) ===
    // Core HOF - handled in handlers/hof.rs (not registered here)
    // map, filter, reduce, pipe

    // Predicates - handled in handlers/hof.rs
    // any, all, find, findIndex, count

    // Array utilities
    register_if_exists(registry, func_registry, "sum");
    register_if_exists(registry, func_registry, "len");
    register_if_exists(registry, func_registry, "range");
    register_if_exists(registry, func_registry, "contains");

    // === CONTROL FLOW (2) ===
    // if, piecewise - handled specially in evaluator (not registered here)

    // === I/O & INSPECTION (3) ===
    register_if_exists(registry, func_registry, "print");
    register_if_exists(registry, func_registry, "type");
    register_if_exists(registry, func_registry, "str");

    // === STRINGS (5) ===
    register_if_exists(registry, func_registry, "concat");
    register_if_exists(registry, func_registry, "split");
    register_if_exists(registry, func_registry, "join");
    register_if_exists(registry, func_registry, "upper");
    register_if_exists(registry, func_registry, "lower");
}

/// Helper to register a function in the prelude if it exists in FunctionRegistry
fn register_if_exists(
    registry: &mut ModuleRegistry,
    func_registry: &FunctionRegistry,
    name: &str,
) {
    if let Some((func, arity)) = func_registry.get(name) {
        registry.register_prelude(name, func, arity);
    }
}

// ============================================================================
// Module Registrations
// ============================================================================

fn register_math_module(registry: &mut ModuleRegistry, func_registry: &FunctionRegistry) {
    let mut module = Module::new("math");

    // Inverse trigonometry
    register_to_module(&mut module, func_registry, "asin");
    register_to_module(&mut module, func_registry, "acos");
    register_to_module(&mut module, func_registry, "atan");
    register_to_module(&mut module, func_registry, "atan2");

    // Hyperbolic functions
    register_to_module(&mut module, func_registry, "sinh");
    register_to_module(&mut module, func_registry, "cosh");
    register_to_module(&mut module, func_registry, "tanh");

    // Logarithms
    register_to_module(&mut module, func_registry, "log10");
    register_to_module(&mut module, func_registry, "log2");

    // Additional math functions
    register_to_module(&mut module, func_registry, "cbrt");
    register_to_module(&mut module, func_registry, "sign");
    register_to_module(&mut module, func_registry, "trunc");

    // Angle conversion
    register_to_module(&mut module, func_registry, "deg");
    register_to_module(&mut module, func_registry, "rad");

    registry.register_module(module);
}

fn register_stats_module(registry: &mut ModuleRegistry, func_registry: &FunctionRegistry) {
    let mut module = Module::new("stats");

    register_to_module(&mut module, func_registry, "mean");
    register_to_module(&mut module, func_registry, "std");

    registry.register_module(module);
}

fn register_linalg_module(registry: &mut ModuleRegistry, func_registry: &FunctionRegistry) {
    let mut module = Module::new("linalg");

    register_to_module(&mut module, func_registry, "dot");
    register_to_module(&mut module, func_registry, "cross");
    register_to_module(&mut module, func_registry, "norm");
    register_to_module(&mut module, func_registry, "normalize");
    register_to_module(&mut module, func_registry, "transpose");
    register_to_module(&mut module, func_registry, "det");
    register_to_module(&mut module, func_registry, "trace");

    registry.register_module(module);
}

fn register_dsp_module(registry: &mut ModuleRegistry, func_registry: &FunctionRegistry) {
    let mut module = Module::new("dsp");

    // FFT functions
    register_to_module(&mut module, func_registry, "fft");
    register_to_module(&mut module, func_registry, "ifft");
    register_to_module(&mut module, func_registry, "fft_mag");
    register_to_module(&mut module, func_registry, "fft_phase");

    // Convolution
    register_to_module(&mut module, func_registry, "conv");
    register_to_module(&mut module, func_registry, "conv_fft");

    // Window functions
    register_to_module(&mut module, func_registry, "hanning");
    register_to_module(&mut module, func_registry, "hamming");
    register_to_module(&mut module, func_registry, "blackman");
    register_to_module(&mut module, func_registry, "rectangular");

    // Utilities
    register_to_module(&mut module, func_registry, "linspace");

    registry.register_module(module);
}

fn register_numerical_module(registry: &mut ModuleRegistry, _func_registry: &FunctionRegistry) {
    let module = Module::new("numerical");

    // Differentiation - handled in handlers/numerical.rs (special forms)
    // diff, diff2, diff3, gradient

    // Integration - handled in handlers/numerical.rs (special forms)
    // integral, simpson, romberg, quad

    // Root finding - handled in handlers/numerical.rs (special forms)
    // solve, newton, secant

    // Note: These are special forms that require lazy evaluation
    // They are registered differently in the function call dispatcher

    registry.register_module(module);
}

fn register_graph_module(registry: &mut ModuleRegistry, func_registry: &FunctionRegistry) {
    let mut module = Module::new("graph");

    // Network creation
    register_to_module(&mut module, func_registry, "network");

    // Network properties
    register_to_module(&mut module, func_registry, "nodes");
    register_to_module(&mut module, func_registry, "edges");
    register_to_module(&mut module, func_registry, "neighbors");
    register_to_module(&mut module, func_registry, "degree");

    // Traversal
    register_to_module(&mut module, func_registry, "bfs");
    register_to_module(&mut module, func_registry, "dfs");
    register_to_module(&mut module, func_registry, "bfs_path");
    register_to_module(&mut module, func_registry, "topological_sort");

    // Shortest paths
    register_to_module(&mut module, func_registry, "dijkstra");

    // MST
    register_to_module(&mut module, func_registry, "kruskal");
    register_to_module(&mut module, func_registry, "prim");

    // Connectivity
    register_to_module(&mut module, func_registry, "connected_components");
    register_to_module(&mut module, func_registry, "is_connected");
    register_to_module(&mut module, func_registry, "has_cycle");

    registry.register_module(module);
}

fn register_pert_module(registry: &mut ModuleRegistry, func_registry: &FunctionRegistry) {
    let mut module = Module::new("pert");

    register_to_module(&mut module, func_registry, "pert_analysis");
    register_to_module(&mut module, func_registry, "forward_pass");
    register_to_module(&mut module, func_registry, "backward_pass");
    register_to_module(&mut module, func_registry, "critical_path");
    register_to_module(&mut module, func_registry, "all_critical_paths");
    register_to_module(&mut module, func_registry, "calculate_slack");
    register_to_module(&mut module, func_registry, "project_duration");
    register_to_module(&mut module, func_registry, "project_variance");
    register_to_module(&mut module, func_registry, "project_std_dev");
    register_to_module(&mut module, func_registry, "expected_time");
    register_to_module(&mut module, func_registry, "task_variance");
    register_to_module(&mut module, func_registry, "completion_probability");
    register_to_module(&mut module, func_registry, "time_for_probability");

    registry.register_module(module);
}

fn register_optimization_module(registry: &mut ModuleRegistry, _func_registry: &FunctionRegistry) {
    let module = Module::new("optimization");

    // Linear programming - handled in handlers/optimization.rs (special forms)
    // simplex, dual_simplex, two_phase_simplex, revised_simplex
    // linprog, objective_value, shadow_price, sensitivity_c, sensitivity_b

    // Note: These are special forms that require complex argument evaluation
    // They are registered differently in the function call dispatcher

    registry.register_module(module);
}

fn register_complex_module(registry: &mut ModuleRegistry, func_registry: &FunctionRegistry) {
    let mut module = Module::new("complex");

    register_to_module(&mut module, func_registry, "complex");
    register_to_module(&mut module, func_registry, "real");
    register_to_module(&mut module, func_registry, "imag");
    register_to_module(&mut module, func_registry, "arg");
    register_to_module(&mut module, func_registry, "conj");
    register_to_module(&mut module, func_registry, "rectangular");

    registry.register_module(module);
}

fn register_strings_module(registry: &mut ModuleRegistry, func_registry: &FunctionRegistry) {
    let mut module = Module::new("strings");

    // Advanced string operations (prelude has: concat, split, join, upper, lower)
    register_to_module(&mut module, func_registry, "trim");
    register_to_module(&mut module, func_registry, "trim_start");
    register_to_module(&mut module, func_registry, "trim_end");
    register_to_module(&mut module, func_registry, "starts_with");
    register_to_module(&mut module, func_registry, "ends_with");
    register_to_module(&mut module, func_registry, "replace");
    register_to_module(&mut module, func_registry, "pad_start");
    register_to_module(&mut module, func_registry, "pad_end");
    register_to_module(&mut module, func_registry, "length");

    registry.register_module(module);
}

fn register_arrays_module(registry: &mut ModuleRegistry, func_registry: &FunctionRegistry) {
    let mut module = Module::new("arrays");

    // Advanced array operations (prelude has: sum, len, range, contains)
    register_to_module(&mut module, func_registry, "reverse");
    register_to_module(&mut module, func_registry, "product");

    registry.register_module(module);
}

fn register_records_module(registry: &mut ModuleRegistry, func_registry: &FunctionRegistry) {
    let mut module = Module::new("records");

    register_to_module(&mut module, func_registry, "keys");
    register_to_module(&mut module, func_registry, "values");
    register_to_module(&mut module, func_registry, "has_field");

    registry.register_module(module);
}

/// Helper to register a function to a module if it exists in FunctionRegistry
fn register_to_module(
    module: &mut Module,
    func_registry: &FunctionRegistry,
    name: &str,
) {
    if let Some((func, arity)) = func_registry.get(name) {
        module.register(name, func, arity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_registry_creation() {
        let registry = create_builtin_registry();

        // Should have modules registered
        assert!(registry.has_module("math"));
        assert!(registry.has_module("stats"));
        assert!(registry.has_module("linalg"));
        assert!(registry.has_module("dsp"));
    }

    #[test]
    fn test_modules_registered() {
        let registry = create_builtin_registry();

        let expected_modules = vec![
            "math", "stats", "linalg", "dsp", "numerical",
            "graph", "pert", "optimization", "complex",
            "strings", "arrays", "records"
        ];

        for module_name in expected_modules {
            assert!(
                registry.has_module(module_name),
                "Module '{}' should be registered",
                module_name
            );
        }
    }

    #[test]
    fn test_module_count() {
        let registry = create_builtin_registry();
        let module_names = registry.module_names();

        // Should have 12 modules
        assert_eq!(module_names.len(), 12);
    }
}
