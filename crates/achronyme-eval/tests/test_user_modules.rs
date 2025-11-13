// ============================================================================
// User-Defined Module System Tests
// ============================================================================
// Tests for export/import functionality with file-based user modules

use achronyme_eval::Evaluator;
use achronyme_types::value::Value;

// ============================================================================
// Basic Export/Import Tests
// ============================================================================

#[test]
fn test_export_and_import_function() {
    let mut evaluator = Evaluator::new();

    // Import a function from a user module
    evaluator.eval_str("import { double } from \"tests/test_modules/math_utils\"").unwrap();

    // Use the imported function
    let result = evaluator.eval_str("double(5)").unwrap();

    match result {
        Value::Number(n) => assert_eq!(n, 10.0),
        _ => panic!("Expected number result"),
    }
}

#[test]
fn test_import_multiple_functions() {
    let mut evaluator = Evaluator::new();

    // Import multiple functions from a user module
    evaluator.eval_str("import { double, triple, square } from \"tests/test_modules/math_utils\"").unwrap();

    // Use all imported functions
    let result1 = evaluator.eval_str("double(3)").unwrap();
    let result2 = evaluator.eval_str("triple(3)").unwrap();
    let result3 = evaluator.eval_str("square(3)").unwrap();

    match (result1, result2, result3) {
        (Value::Number(n1), Value::Number(n2), Value::Number(n3)) => {
            assert_eq!(n1, 6.0);
            assert_eq!(n2, 9.0);
            assert_eq!(n3, 9.0);
        }
        _ => panic!("Expected number results"),
    }
}

#[test]
fn test_import_with_alias() {
    let mut evaluator = Evaluator::new();

    // Import with alias
    evaluator.eval_str("import { double as twice } from \"tests/test_modules/math_utils\"").unwrap();

    // Use the aliased function
    let result = evaluator.eval_str("twice(7)").unwrap();

    match result {
        Value::Number(n) => assert_eq!(n, 14.0),
        _ => panic!("Expected number result"),
    }
}

#[test]
fn test_import_constants() {
    let mut evaluator = Evaluator::new();

    // Import constants
    evaluator.eval_str("import { goldenRatio, euler } from \"tests/test_modules/constants\"").unwrap();

    // Use imported constants
    let result1 = evaluator.eval_str("goldenRatio").unwrap();
    let result2 = evaluator.eval_str("euler").unwrap();

    match (result1, result2) {
        (Value::Number(n1), Value::Number(n2)) => {
            assert!((n1 - 1.618033988749895).abs() < 1e-10);
            assert!((n2 - 2.718281828459045).abs() < 1e-10);
        }
        _ => panic!("Expected number results"),
    }
}

#[test]
fn test_private_values_not_exported() {
    let mut evaluator = Evaluator::new();

    // Try to import a non-exported function
    let result = evaluator.eval_str("import { privateHelper } from \"tests/test_modules/math_utils\"");

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not exported"));
}

// ============================================================================
// Module Re-exports and Dependencies
// ============================================================================

#[test]
fn test_module_imports_from_built_in() {
    let mut evaluator = Evaluator::new();

    // Import from a user module that itself imports from built-in modules
    evaluator.eval_str("import { coefficientOfVariation } from \"tests/test_modules/stats_utils\"").unwrap();

    // Use the imported function
    let result = evaluator.eval_str("coefficientOfVariation([10, 20, 30, 40, 50])").unwrap();

    // CV = (std / mean) * 100
    // Mean = 30, Std ≈ 15.81, CV ≈ 52.7 (using sample std)
    match result {
        Value::Number(n) => {
            assert!(n > 50.0 && n < 55.0, "Expected CV around 52.7, got {}", n);
        }
        _ => panic!("Expected number result"),
    }
}

#[test]
fn test_import_from_multiple_user_modules() {
    let mut evaluator = Evaluator::new();

    // Import from multiple user modules
    evaluator.eval_str("import { double } from \"tests/test_modules/math_utils\"").unwrap();
    evaluator.eval_str("import { goldenRatio } from \"tests/test_modules/constants\"").unwrap();

    // Use functions from both modules
    let result = evaluator.eval_str("double(goldenRatio)").unwrap();

    match result {
        Value::Number(n) => {
            assert!((n - 3.23606797749979).abs() < 1e-10);
        }
        _ => panic!("Expected number result"),
    }
}

// ============================================================================
// Module Caching Tests
// ============================================================================

#[test]
fn test_module_caching() {
    let mut evaluator = Evaluator::new();

    // Import from the same module twice
    evaluator.eval_str("import { double } from \"tests/test_modules/math_utils\"").unwrap();
    evaluator.eval_str("import { triple } from \"tests/test_modules/math_utils\"").unwrap();

    // Both should work (module should be cached after first import)
    let result1 = evaluator.eval_str("double(4)").unwrap();
    let result2 = evaluator.eval_str("triple(4)").unwrap();

    match (result1, result2) {
        (Value::Number(n1), Value::Number(n2)) => {
            assert_eq!(n1, 8.0);
            assert_eq!(n2, 12.0);
        }
        _ => panic!("Expected number results"),
    }
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_import_nonexistent_module() {
    let mut evaluator = Evaluator::new();

    // Try to import from a module that doesn't exist
    let result = evaluator.eval_str("import { foo } from \"tests/test_modules/nonexistent\"");

    assert!(result.is_err());
}

#[test]
fn test_import_nonexistent_function() {
    let mut evaluator = Evaluator::new();

    // Try to import a function that doesn't exist in the module
    let result = evaluator.eval_str("import { nonExistent } from \"tests/test_modules/math_utils\"");

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not exported"));
}

#[test]
fn test_export_undefined_value() {
    let mut evaluator = Evaluator::new();

    // Try to export a value that doesn't exist
    let result = evaluator.eval_str("export { undefinedValue }");

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found in current scope"));
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_use_imported_in_expression() {
    let mut evaluator = Evaluator::new();

    evaluator.eval_str("import { double, triple } from \"tests/test_modules/math_utils\"").unwrap();

    // Use imported functions in complex expressions
    let result = evaluator.eval_str("double(5) + triple(3)").unwrap();

    match result {
        Value::Number(n) => assert_eq!(n, 19.0), // 10 + 9
        _ => panic!("Expected number result"),
    }
}

#[test]
fn test_use_imported_with_vectors() {
    let mut evaluator = Evaluator::new();

    evaluator.eval_str("import { double } from \"tests/test_modules/math_utils\"").unwrap();

    // Use imported function with map over a vector
    let result = evaluator.eval_str("map(double, [1, 2, 3, 4, 5])").unwrap();

    match result {
        Value::Vector(vec) => {
            assert_eq!(vec.len(), 5);
            assert_eq!(vec[0], Value::Number(2.0));
            assert_eq!(vec[1], Value::Number(4.0));
            assert_eq!(vec[2], Value::Number(6.0));
            assert_eq!(vec[3], Value::Number(8.0));
            assert_eq!(vec[4], Value::Number(10.0));
        }
        _ => panic!("Expected vector result"),
    }
}

#[test]
fn test_use_imported_with_filter() {
    let mut evaluator = Evaluator::new();

    evaluator.eval_str("import { square } from \"tests/test_modules/math_utils\"").unwrap();

    // Use imported function in filter predicate
    let result = evaluator.eval_str("filter(x => square(x) < 20, [1, 2, 3, 4, 5])").unwrap();

    match result {
        Value::Vector(vec) => {
            assert_eq!(vec.len(), 4); // 1, 2, 3, 4 (squares: 1, 4, 9, 16)
            assert_eq!(vec[0], Value::Number(1.0));
            assert_eq!(vec[1], Value::Number(2.0));
            assert_eq!(vec[2], Value::Number(3.0));
            assert_eq!(vec[3], Value::Number(4.0));
        }
        _ => panic!("Expected vector result"),
    }
}

#[test]
fn test_mixed_built_in_and_user_imports() {
    let mut evaluator = Evaluator::new();

    // Import from both built-in and user modules
    evaluator.eval_str("import { mean } from \"stats\"").unwrap();
    evaluator.eval_str("import { double } from \"tests/test_modules/math_utils\"").unwrap();

    // Use both in a single expression
    let result = evaluator.eval_str("double(mean([10, 20, 30]))").unwrap();

    match result {
        Value::Number(n) => assert_eq!(n, 40.0), // mean = 20, double(20) = 40
        _ => panic!("Expected number result"),
    }
}

// ============================================================================
// Advanced Tests
// ============================================================================

#[test]
fn test_import_in_lambda() {
    let mut evaluator = Evaluator::new();

    evaluator.eval_str("import { double } from \"tests/test_modules/math_utils\"").unwrap();

    // Define a lambda that uses imported function
    evaluator.eval_str("let f = x => double(x) + 1").unwrap();

    let result = evaluator.eval_str("f(10)").unwrap();

    match result {
        Value::Number(n) => assert_eq!(n, 21.0),
        _ => panic!("Expected number result"),
    }
}

#[test]
fn test_import_and_local_definition() {
    let mut evaluator = Evaluator::new();

    evaluator.eval_str("import { double } from \"tests/test_modules/math_utils\"").unwrap();

    // Define a local function with the same name as an imported one
    // The local should shadow the imported
    evaluator.eval_str("let double = x => x * 4").unwrap();

    let result = evaluator.eval_str("double(5)").unwrap();

    match result {
        Value::Number(n) => assert_eq!(n, 20.0), // Uses local definition (x * 4)
        _ => panic!("Expected number result"),
    }
}
