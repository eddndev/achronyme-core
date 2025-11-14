/// Integration tests for string broadcasting operations
///
/// Tests the new broadcasting features for strings:
/// - String * Number: Repetition (e.g., "-" * 40)
/// - String + Any: Concatenation with automatic toString conversion
///   (e.g., "Result: " + 42)

use achronyme_eval::Evaluator;
use achronyme_types::value::Value;

fn eval(source: &str) -> Result<Value, String> {
    let mut evaluator = Evaluator::new();
    evaluator.eval_str(source)
}

// ============================================================================
// String Repetition: String * Number
// ============================================================================

#[test]
fn test_string_repeat_basic() {
    let result = eval(r#""-" * 40"#).unwrap();
    assert_eq!(result, Value::String("-".repeat(40)));
}

#[test]
fn test_string_repeat_zero() {
    let result = eval(r#""hello" * 0"#).unwrap();
    assert_eq!(result, Value::String("".to_string()));
}

#[test]
fn test_string_repeat_one() {
    let result = eval(r#""test" * 1"#).unwrap();
    assert_eq!(result, Value::String("test".to_string()));
}

#[test]
fn test_string_repeat_multiple() {
    let result = eval(r#""abc" * 3"#).unwrap();
    assert_eq!(result, Value::String("abcabcabc".to_string()));
}

#[test]
fn test_string_repeat_commutative() {
    let result1 = eval(r#""x" * 5"#).unwrap();
    let result2 = eval(r#"5 * "x""#).unwrap();
    assert_eq!(result1, result2);
    assert_eq!(result1, Value::String("xxxxx".to_string()));
}

#[test]
fn test_string_repeat_with_variable() {
    let result = eval(r#"
        let separator = "="
        let count = 50
        separator * count
    "#).unwrap();
    assert_eq!(result, Value::String("=".repeat(50)));
}

#[test]
fn test_string_repeat_in_expression() {
    let result = eval(r#"
        let title = "Section"
        let line = "-" * 20
        line
    "#).unwrap();
    assert_eq!(result, Value::String("-".repeat(20)));
}

#[test]
fn test_string_repeat_multichar() {
    let result = eval(r#""ab" * 4"#).unwrap();
    assert_eq!(result, Value::String("abababab".to_string()));
}

#[test]
fn test_string_repeat_unicode() {
    let result = eval(r#""🌟" * 3"#).unwrap();
    assert_eq!(result, Value::String("🌟🌟🌟".to_string()));
}

#[test]
fn test_string_repeat_negative_fails() {
    let result = eval(r#""x" * -5"#);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("non-negative"));
}

#[test]
fn test_string_repeat_float_truncates() {
    let result = eval(r#""x" * 3.7"#).unwrap();
    assert_eq!(result, Value::String("xxx".to_string())); // 3.7 truncates to 3
}

// ============================================================================
// String Concatenation: String + Any (automatic toString)
// ============================================================================

#[test]
fn test_string_plus_number() {
    let result = eval(r#""Result: " + 42"#).unwrap();
    assert_eq!(result, Value::String("Result: 42".to_string()));
}

#[test]
fn test_number_plus_string() {
    let result = eval(r#"42 + " is the answer""#).unwrap();
    assert_eq!(result, Value::String("42 is the answer".to_string()));
}

#[test]
fn test_string_plus_boolean_true() {
    let result = eval(r#""Value: " + true"#).unwrap();
    assert_eq!(result, Value::String("Value: true".to_string()));
}

#[test]
fn test_string_plus_boolean_false() {
    let result = eval(r#""Active: " + false"#).unwrap();
    assert_eq!(result, Value::String("Active: false".to_string()));
}

#[test]
fn test_string_plus_float() {
    let result = eval(r#""Pi is approximately " + 3.14159"#).unwrap();
    assert_eq!(result, Value::String("Pi is approximately 3.14159".to_string()));
}

#[test]
fn test_string_plus_integer_formatted() {
    // Integers should display without decimal point
    let result = eval(r#""Count: " + 100"#).unwrap();
    assert_eq!(result, Value::String("Count: 100".to_string()));
}

#[test]
fn test_string_concatenation_chain() {
    let result = eval(r#""Value: " + 42 + " units""#).unwrap();
    assert_eq!(result, Value::String("Value: 42 units".to_string()));
}

#[test]
fn test_string_plus_with_variables() {
    let result = eval(r#"
        let name = "Alice"
        let age = 30
        name + " is " + age + " years old"
    "#).unwrap();
    assert_eq!(result, Value::String("Alice is 30 years old".to_string()));
}

#[test]
fn test_string_plus_vector() {
    let result = eval(r#""Vector: " + [1, 2, 3]"#).unwrap();
    assert_eq!(result, Value::String("Vector: [1, 2, 3]".to_string()));
}

#[test]
fn test_string_plus_complex() {
    let result = eval(r#""Complex: " + 3i"#).unwrap();
    assert_eq!(result, Value::String("Complex: 3i".to_string()));
}

#[test]
fn test_string_plus_complex_full() {
    let result = eval(r#""Value: " + (2 + 3i)"#).unwrap();
    // Note: exact format depends on complex number formatting
    assert!(matches!(result, Value::String(s) if s.contains("2") && s.contains("3")));
}

#[test]
fn test_string_plus_record() {
    let result = eval(r#""Data: " + {x: 10, y: 20}"#).unwrap();
    // Record should be formatted as string
    assert!(matches!(result, Value::String(s) if s.contains("x") && s.contains("10")));
}

// ============================================================================
// Combined Operations
// ============================================================================

#[test]
fn test_repeat_and_concatenate() {
    let result = eval(r#"
        let border = "=" * 30
        let title = "REPORT"
        border + " " + title + " " + border
    "#).unwrap();
    assert_eq!(
        result,
        Value::String(format!("{} REPORT {}", "=".repeat(30), "=".repeat(30)))
    );
}

#[test]
fn test_dynamic_separator_line() {
    let result = eval(r#"
        let width = 40
        "-" * width
    "#).unwrap();
    assert_eq!(result, Value::String("-".repeat(40)));
}

#[test]
fn test_formatted_output() {
    let result = eval(r#"
        let x = 10
        let y = 20
        let sum = x + y
        "Result: " + x + " + " + y + " = " + sum
    "#).unwrap();
    assert_eq!(result, Value::String("Result: 10 + 20 = 30".to_string()));
}

#[test]
fn test_print_with_repetition() {
    // This simulates: print("-" * 40)
    let result = eval(r#"print("-" * 40)"#).unwrap();
    assert_eq!(result, Value::String("-".repeat(40)));
}

#[test]
fn test_print_with_concatenation() {
    // This simulates: print("Result: " + resultado)
    let result = eval(r#"
        let resultado = 42
        print("Result: " + resultado)
    "#).unwrap();
    assert_eq!(result, Value::String("Result: 42".to_string()));
}

#[test]
fn test_string_multiplication_in_function() {
    let result = eval(r#"
        let createBorder = (char, len) => char * len
        createBorder("*", 25)
    "#).unwrap();
    assert_eq!(result, Value::String("*".repeat(25)));
}

#[test]
fn test_string_formatting_function() {
    let result = eval(r#"
        let format = (label, value) => label + ": " + value
        format("Temperature", 23.5)
    "#).unwrap();
    assert_eq!(result, Value::String("Temperature: 23.5".to_string()));
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_empty_string_repeat() {
    let result = eval(r#""" * 100"#).unwrap();
    assert_eq!(result, Value::String("".to_string()));
}

#[test]
fn test_empty_string_concatenate() {
    let result = eval(r#""" + 42"#).unwrap();
    assert_eq!(result, Value::String("42".to_string()));
}

#[test]
fn test_string_repeat_large_number() {
    let result = eval(r#""x" * 1000"#).unwrap();
    assert_eq!(result, Value::String("x".repeat(1000)));
}

#[test]
fn test_whitespace_repeat() {
    let result = eval(r#"" " * 10"#).unwrap();
    assert_eq!(result, Value::String(" ".repeat(10)));
}

#[test]
fn test_newline_repeat() {
    let result = eval(r#""\n" * 3"#).unwrap();
    assert_eq!(result, Value::String("\n\n\n".to_string()));
}

// ============================================================================
// Backward Compatibility
// ============================================================================

#[test]
fn test_string_string_concatenation_still_works() {
    // Original string + string should still work
    let result = eval(r#""hello" + " " + "world""#).unwrap();
    assert_eq!(result, Value::String("hello world".to_string()));
}

#[test]
fn test_concat_function_still_works() {
    // The concat function should still work
    let result = eval(r#"concat("hello", " world")"#).unwrap();
    assert_eq!(result, Value::String("hello world".to_string()));
}

#[test]
fn test_numeric_addition_not_affected() {
    // Numeric addition should not be affected
    let result = eval(r#"40 + 2"#).unwrap();
    assert_eq!(result, Value::Number(42.0));
}

#[test]
fn test_numeric_multiplication_not_affected() {
    // Numeric multiplication should not be affected
    let result = eval(r#"6 * 7"#).unwrap();
    assert_eq!(result, Value::Number(42.0));
}
