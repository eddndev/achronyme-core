use achronyme_eval::Evaluator;

#[test]
fn test_import_syntax_accepted() {
    let mut evaluator = Evaluator::new();

    // Import statement should be accepted
    let result = evaluator.eval_str("import { asin } from \"math\"");
    assert!(result.is_ok(), "Import statement should be accepted: {:?}", result);
}

#[test]
fn test_import_multiple_functions() {
    let mut evaluator = Evaluator::new();

    // Import multiple functions
    let result = evaluator.eval_str("import { asin, acos, sinh } from \"math\"");
    assert!(result.is_ok(), "Import with multiple functions should work: {:?}", result);
}

#[test]
fn test_import_with_alias() {
    let mut evaluator = Evaluator::new();

    // Import with alias
    let result = evaluator.eval_str("import { mean as average } from \"stats\"");
    assert!(result.is_ok(), "Import with alias should work: {:?}", result);
}

#[test]
fn test_import_nonexistent_module() {
    let mut evaluator = Evaluator::new();

    // Try to import from non-existent module
    let result = evaluator.eval_str("import { foo } from \"nonexistent\"");
    assert!(result.is_err(), "Import from non-existent module should fail");
    // Now it tries to load as file, so error message is different
    let err = result.unwrap_err();
    assert!(
        err.contains("Module 'nonexistent' not found") ||
        err.contains("No such file") ||
        err.contains("cannot find") ||
        err.contains("Failed to read module") ||
        err.contains("El sistema no puede encontrar"),  // Spanish Windows error
        "Expected module not found error, got: {}", err
    );
}

#[test]
fn test_import_nonexistent_function() {
    let mut evaluator = Evaluator::new();

    // Try to import non-existent function from valid module
    let result = evaluator.eval_str("import { nonexistent } from \"math\"");
    assert!(result.is_err(), "Import non-existent function should fail");
    assert!(result.unwrap_err().contains("Function 'nonexistent' not found"));
}

#[test]
fn test_multiple_import_statements() {
    let mut evaluator = Evaluator::new();

    // Multiple import statements
    let result = evaluator.eval_str(r#"
        import { asin } from "math";
        import { mean } from "stats"
    "#);
    assert!(result.is_ok(), "Multiple import statements should work: {:?}", result);
}

#[test]
fn test_import_from_different_modules() {
    let mut evaluator = Evaluator::new();

    // Import from different modules
    let result = evaluator.eval_str(r#"
        import { asin } from "math";
        import { mean } from "stats";
        import { dot } from "linalg";
        import { fft } from "dsp"
    "#);
    assert!(result.is_ok(), "Imports from multiple modules should work: {:?}", result);
}

#[test]
fn test_module_registry_has_all_modules() {
    let evaluator = Evaluator::new();
    let registry = evaluator.module_registry();

    // Check that all expected modules exist
    assert!(registry.has_module("math"));
    assert!(registry.has_module("stats"));
    assert!(registry.has_module("linalg"));
    assert!(registry.has_module("dsp"));
    assert!(registry.has_module("numerical"));
    assert!(registry.has_module("graph"));
    assert!(registry.has_module("pert"));
    assert!(registry.has_module("optimization"));
    assert!(registry.has_module("complex"));
    assert!(registry.has_module("strings"));
    assert!(registry.has_module("arrays"));
    assert!(registry.has_module("records"));
}

#[test]
fn test_imported_modules_tracked() {
    let mut evaluator = Evaluator::new();

    // Import some functions
    evaluator.eval_str("import { asin, acos as arccos } from \"math\"").unwrap();

    let imported = evaluator.imported_modules();

    // Check that imports are tracked
    assert!(imported.contains_key("asin"));
    assert_eq!(imported.get("asin"), Some(&("math".to_string(), "asin".to_string())));

    // Check alias
    assert!(imported.contains_key("arccos"));
    assert_eq!(imported.get("arccos"), Some(&("math".to_string(), "acos".to_string())));

    // Original name shouldn't be in imports
    assert!(!imported.contains_key("acos"));
}

// ============================================================================
// Phase 3 Tests: Using Imported Functions
// ============================================================================

#[test]
fn test_use_imported_function() {
    let mut evaluator = Evaluator::new();

    // Import and use a function
    evaluator.eval_str("import { mean } from \"stats\"").unwrap();
    let result = evaluator.eval_str("mean([1, 2, 3, 4, 5])").unwrap();

    // Should compute the mean
    match result {
        achronyme_types::value::Value::Number(n) => assert_eq!(n, 3.0),
        _ => panic!("Expected number result"),
    }
}

#[test]
fn test_use_aliased_function() {
    let mut evaluator = Evaluator::new();

    // Import with alias and use it
    evaluator.eval_str("import { mean as average } from \"stats\"").unwrap();
    let result = evaluator.eval_str("average([1, 2, 3, 4, 5])").unwrap();

    // Should compute the mean
    match result {
        achronyme_types::value::Value::Number(n) => assert_eq!(n, 3.0),
        _ => panic!("Expected number result"),
    }
}

#[test]
fn test_original_name_still_works_with_alias_backward_compat() {
    let mut evaluator = Evaluator::new();

    // Import with alias
    evaluator.eval_str("import { mean as average } from \"stats\"").unwrap();

    // Original name still works (backward compatibility via FunctionRegistry fallback)
    // In Phase 4 (breaking changes), this could be changed to require explicit import
    let result = evaluator.eval_str("mean([1, 2, 3])");
    assert!(result.is_ok(), "Original name should still work for backward compatibility");

    // But the alias also works
    let result2 = evaluator.eval_str("average([1, 2, 3])");
    assert!(result2.is_ok(), "Alias should work");
}

#[test]
fn test_use_multiple_imported_functions() {
    let mut evaluator = Evaluator::new();

    // Import multiple functions
    evaluator.eval_str("import { mean, std } from \"stats\"").unwrap();

    // Use both
    let mean_result = evaluator.eval_str("mean([1, 2, 3])").unwrap();
    let std_result = evaluator.eval_str("std([1, 2, 3])").unwrap();

    // Both should work
    assert!(matches!(mean_result, achronyme_types::value::Value::Number(_)));
    assert!(matches!(std_result, achronyme_types::value::Value::Number(_)));
}

#[test]
fn test_prelude_functions_still_work() {
    let mut evaluator = Evaluator::new();

    // Import something
    evaluator.eval_str("import { mean } from \"stats\"").unwrap();

    // Prelude functions should still work without import
    let result = evaluator.eval_str("sin(0)").unwrap();
    match result {
        achronyme_types::value::Value::Number(n) => assert!((n - 0.0).abs() < 1e-10),
        _ => panic!("Expected number result"),
    }
}

#[test]
fn test_import_and_use_in_expression() {
    let mut evaluator = Evaluator::new();

    // Import and use in an expression
    let result = evaluator.eval_str(r#"
        import { mean } from "stats";
        mean([1, 2, 3]) + 10
    "#).unwrap();

    // Should be 2 + 10 = 12
    match result {
        achronyme_types::value::Value::Number(n) => assert_eq!(n, 12.0),
        _ => panic!("Expected number result"),
    }
}

#[test]
fn test_import_from_math_module() {
    let mut evaluator = Evaluator::new();

    // Import inverse trig functions
    evaluator.eval_str("import { asin, acos } from \"math\"").unwrap();

    let result = evaluator.eval_str("asin(0.5)").unwrap();
    assert!(matches!(result, achronyme_types::value::Value::Number(_)));
}

#[test]
fn test_import_from_linalg_module() {
    let mut evaluator = Evaluator::new();

    // Import dot product
    evaluator.eval_str("import { dot } from \"linalg\"").unwrap();

    let result = evaluator.eval_str("dot([1, 2], [3, 4])").unwrap();
    // dot([1,2], [3,4]) = 1*3 + 2*4 = 11
    match result {
        achronyme_types::value::Value::Number(n) => assert_eq!(n, 11.0),
        _ => panic!("Expected number result"),
    }
}
