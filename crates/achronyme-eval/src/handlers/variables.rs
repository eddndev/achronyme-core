use achronyme_parser::ast::AstNode;
use achronyme_types::complex::Complex;
use achronyme_types::value::Value;

use crate::evaluator::Evaluator;

/// Evaluate a variable declaration (let statement)
pub fn evaluate_declaration(
    evaluator: &mut Evaluator,
    name: &str,
    initializer: &AstNode,
) -> Result<Value, String> {
    // Evaluate the initializer
    let value = evaluator.evaluate(initializer)?;

    // Define the variable in the environment
    evaluator.environment_mut().define(name.to_string(), value.clone())?;

    Ok(value)
}

/// Evaluate a variable reference
pub fn evaluate_reference(evaluator: &Evaluator, name: &str) -> Result<Value, String> {
    // Check if it's a variable first
    if evaluator.environment().has(name) {
        return evaluator.environment().get(name);
    }

    // Special case for imaginary unit 'i'
    if name.to_lowercase() == "i" {
        return Ok(Value::Complex(Complex::new(0.0, 1.0)));
    }

    // Check if it's a constant
    if evaluator.constants().has(name) {
        return Ok(Value::Number(evaluator.constants().get(name)?));
    }

    // Check if it's a built-in function
    if is_builtin_function(name) {
        use achronyme_types::function::Function;
        return Ok(Value::Function(Function::builtin(name.to_string())));
    }

    // Not found
    Err(format!("Undefined variable or constant: {}", name))
}

/// Check if a name corresponds to a built-in function
fn is_builtin_function(name: &str) -> bool {
    matches!(
        name,
        // Higher-order functions
        "map" | "filter" | "reduce" | "pipe" |
        // Math functions
        "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | "atan2" |
        "sinh" | "cosh" | "tanh" | "asinh" | "acosh" | "atanh" |
        "exp" | "ln" | "log" | "log10" | "log2" | "sqrt" | "cbrt" |
        "floor" | "ceil" | "round" | "abs" | "sign" |
        // Statistics
        "sum" | "mean" | "median" | "std" | "variance" | "min" | "max" |
        // Calculus
        "diff" | "diff2" | "diff3" | "gradient" | "integral" | "trapz" |
        "simpson" | "romberg" | "solve" | "derivative" |
        // Linear algebra
        "dot" | "cross" | "norm" | "det" | "inv" | "transpose" | "trace" |
        "eigenvalues" | "eigenvectors" | "qr" | "svd" | "lu" | "cholesky" |
        // DSP
        "fft" | "ifft" | "convolve" | "correlate" | "filter_signal" |
        // String functions
        "len" | "concat" | "split" | "join" | "upper" | "lower" | "trim" |
        // Complex numbers
        "re" | "im" | "arg" | "conj" | "polar" |
        // Graph functions - Network operations
        "network" | "nodes" | "edges" | "neighbors" | "degree" |
        // Graph algorithms - Traversal
        "bfs" | "dfs" | "bfs_path" |
        // Graph algorithms - Shortest paths
        "dijkstra" |
        // Graph algorithms - Cycles
        "has_cycle" |
        // Graph algorithms - MST
        "kruskal" | "prim" |
        // Graph algorithms - Connectivity
        "connected_components" | "is_connected" |
        // Graph algorithms - Topological sort
        "topological_sort" |
        // PERT/CPM
        "forward_pass" | "backward_pass" | "calculate_slack" | "critical_path" |
        "all_critical_paths" | "project_duration" | "expected_time" | "task_variance" |
        "project_variance" | "project_std_dev" | "completion_probability" |
        "time_for_probability" | "pert_analysis" |
        // Debug functions
        "describe"
    )
}
