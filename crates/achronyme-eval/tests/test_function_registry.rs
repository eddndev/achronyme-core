use achronyme_eval::Evaluator;
use achronyme_types::value::Value;

fn eval(source: &str) -> Result<Value, String> {
    let mut evaluator = Evaluator::new();
    evaluator.eval_str(source)
}

#[test]
fn test_all_dsp_functions_accessible() {
    let functions = vec![
        "fft", "ifft", "fft_mag", "fft_phase",
        "conv", "conv_fft",
        "hanning", "hamming", "blackman", "rectangular",
        "linspace"
    ];

    for func_name in functions {
        let code = format!("let f = {}; f", func_name);
        let result = eval(&code);
        assert!(result.is_ok(), "Function {} should be accessible as a first-class value", func_name);
        assert!(matches!(result.unwrap(), Value::Function(_)),
                "Function {} should be a Function value", func_name);
    }
}

#[test]
fn test_all_window_functions_callable() {
    let functions = vec!["hanning", "hamming", "blackman", "rectangular"];

    for func_name in functions {
        let code = format!("{}(4)", func_name);
        let result = eval(&code);
        assert!(result.is_ok(), "Function {} should be callable", func_name);
    }
}

#[test]
fn test_complex_functions_accessible() {
    let functions = vec!["complex", "real", "imag", "conj", "arg"];

    for func_name in functions {
        let code = format!("let f = {}; f", func_name);
        let result = eval(&code);
        assert!(result.is_ok(), "Function {} should be accessible", func_name);
        assert!(matches!(result.unwrap(), Value::Function(_)),
                "Function {} should be a Function value", func_name);
    }
}

#[test]
fn test_exponential_functions_accessible() {
    let functions = vec!["exp", "ln", "log", "log10", "log2", "sqrt", "cbrt", "pow"];

    for func_name in functions {
        let code = format!("let f = {}; f", func_name);
        let result = eval(&code);
        assert!(result.is_ok(), "Function {} should be accessible", func_name);
        assert!(matches!(result.unwrap(), Value::Function(_)),
                "Function {} should be a Function value", func_name);
    }
}

#[test]
fn test_trig_functions_accessible() {
    let functions = vec![
        "sin", "cos", "tan", "asin", "acos", "atan", "atan2",
        "sinh", "cosh", "tanh"
    ];

    for func_name in functions {
        let code = format!("let f = {}; f", func_name);
        let result = eval(&code);
        assert!(result.is_ok(), "Function {} should be accessible", func_name);
        assert!(matches!(result.unwrap(), Value::Function(_)),
                "Function {} should be a Function value", func_name);
    }
}

#[test]
fn test_rounding_functions_accessible() {
    let functions = vec!["floor", "ceil", "round", "abs", "sign"];

    for func_name in functions {
        let code = format!("let f = {}; f", func_name);
        let result = eval(&code);
        assert!(result.is_ok(), "Function {} should be accessible", func_name);
        assert!(matches!(result.unwrap(), Value::Function(_)),
                "Function {} should be a Function value", func_name);
    }
}

#[test]
fn test_stats_functions_accessible() {
    let functions = vec!["sum", "mean", "std"];

    for func_name in functions {
        let code = format!("let f = {}; f", func_name);
        let result = eval(&code);
        assert!(result.is_ok(), "Function {} should be accessible", func_name);
        assert!(matches!(result.unwrap(), Value::Function(_)),
                "Function {} should be a Function value", func_name);
    }
}

#[test]
fn test_matrix_functions_accessible() {
    let functions = vec![
        "dot", "cross", "norm", "det", "transpose", "trace"
    ];

    for func_name in functions {
        let code = format!("let f = {}; f", func_name);
        let result = eval(&code);
        assert!(result.is_ok(), "Function {} should be accessible", func_name);
        assert!(matches!(result.unwrap(), Value::Function(_)),
                "Function {} should be a Function value", func_name);
    }
}

#[test]
fn test_vector_functions_accessible() {
    let functions = vec!["map", "filter", "reduce", "pipe"];

    for func_name in functions {
        let code = format!("let f = {}; f", func_name);
        let result = eval(&code);
        assert!(result.is_ok(), "Function {} should be accessible", func_name);
        assert!(matches!(result.unwrap(), Value::Function(_)),
                "Function {} should be a Function value", func_name);
    }
}

#[test]
fn test_string_functions_accessible() {
    let functions = vec!["concat", "length"];

    for func_name in functions {
        let code = format!("let f = {}; f", func_name);
        let result = eval(&code);
        assert!(result.is_ok(), "Function {} should be accessible", func_name);
        assert!(matches!(result.unwrap(), Value::Function(_)),
                "Function {} should be a Function value", func_name);
    }
}

#[test]
fn test_record_functions_accessible() {
    let functions = vec!["keys", "values", "has_field"];

    for func_name in functions {
        let code = format!("let f = {}; f", func_name);
        let result = eval(&code);
        assert!(result.is_ok(), "Function {} should be accessible", func_name);
        assert!(matches!(result.unwrap(), Value::Function(_)),
                "Function {} should be a Function value", func_name);
    }
}

#[test]
fn test_graph_functions_accessible() {
    let functions = vec![
        "network", "nodes", "edges", "neighbors", "degree",
        "bfs", "dfs", "bfs_path",
        "dijkstra",
        "has_cycle",
        "kruskal", "prim",
        "connected_components", "is_connected",
        "topological_sort"
    ];

    for func_name in functions {
        let code = format!("let f = {}; f", func_name);
        let result = eval(&code);
        assert!(result.is_ok(), "Function {} should be accessible", func_name);
        assert!(matches!(result.unwrap(), Value::Function(_)),
                "Function {} should be a Function value", func_name);
    }
}

#[test]
fn test_pert_functions_accessible() {
    let functions = vec![
        "forward_pass", "backward_pass", "calculate_slack", "critical_path",
        "all_critical_paths", "project_duration", "expected_time", "task_variance",
        "project_variance", "project_std_dev", "completion_probability",
        "time_for_probability", "pert_analysis"
    ];

    for func_name in functions {
        let code = format!("let f = {}; f", func_name);
        let result = eval(&code);
        assert!(result.is_ok(), "Function {} should be accessible", func_name);
        assert!(matches!(result.unwrap(), Value::Function(_)),
                "Function {} should be a Function value", func_name);
    }
}

#[test]
fn test_numerical_functions_accessible() {
    let functions = vec![
        "diff", "diff2", "diff3", "gradient", "integral", "trapz",
        "simpson", "romberg", "solve", "derivative"
    ];

    for func_name in functions {
        let code = format!("let f = {}; f", func_name);
        let result = eval(&code);
        assert!(result.is_ok(), "Function {} should be accessible", func_name);
        assert!(matches!(result.unwrap(), Value::Function(_)),
                "Function {} should be a Function value", func_name);
    }
}

#[test]
fn test_debug_functions_accessible() {
    let functions = vec!["describe"];

    for func_name in functions {
        let code = format!("let f = {}; f", func_name);
        let result = eval(&code);
        assert!(result.is_ok(), "Function {} should be accessible", func_name);
        assert!(matches!(result.unwrap(), Value::Function(_)),
                "Function {} should be a Function value", func_name);
    }
}
