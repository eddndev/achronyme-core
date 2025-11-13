/// Integration tests for string utility functions
///
/// Tests all string manipulation functions including:
/// - Operator overload: + for concatenation
/// - Case conversion: upper, lower
/// - Whitespace: trim, trim_start, trim_end
/// - Search: starts_with, ends_with
/// - Manipulation: replace, split, join
/// - Padding: pad_start, pad_end

use achronyme_eval::Evaluator;
use achronyme_types::value::Value;

// ============================================================================
// Operator Overload Tests
// ============================================================================

#[test]
fn test_string_concatenation_with_plus() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#""hello" + " world""#).unwrap();
    assert_eq!(result, Value::String("hello world".to_string()));
}

#[test]
fn test_string_concatenation_multiple() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#""a" + "b" + "c""#).unwrap();
    assert_eq!(result, Value::String("abc".to_string()));
}

#[test]
fn test_string_concatenation_empty() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#""hello" + """#).unwrap();
    assert_eq!(result, Value::String("hello".to_string()));
}

// ============================================================================
// Case Conversion Tests
// ============================================================================

#[test]
fn test_upper_basic() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"upper("hello")"#).unwrap();
    assert_eq!(result, Value::String("HELLO".to_string()));
}

#[test]
fn test_upper_mixed_case() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"upper("Hello World")"#).unwrap();
    assert_eq!(result, Value::String("HELLO WORLD".to_string()));
}

#[test]
fn test_upper_already_upper() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"upper("HELLO")"#).unwrap();
    assert_eq!(result, Value::String("HELLO".to_string()));
}

#[test]
fn test_lower_basic() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"lower("HELLO")"#).unwrap();
    assert_eq!(result, Value::String("hello".to_string()));
}

#[test]
fn test_lower_mixed_case() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"lower("Hello World")"#).unwrap();
    assert_eq!(result, Value::String("hello world".to_string()));
}

#[test]
fn test_case_roundtrip() {
    let mut evaluator = Evaluator::new();

    let code = r#"
        let original = "Hello";
        let lowered = lower(original);
        upper(lowered)
    "#;
    let result = evaluator.eval_str(code).unwrap();
    assert_eq!(result, Value::String("HELLO".to_string()));
}

// ============================================================================
// Whitespace Handling Tests
// ============================================================================

#[test]
fn test_trim_both_sides() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"trim("  hello  ")"#).unwrap();
    assert_eq!(result, Value::String("hello".to_string()));
}

#[test]
fn test_trim_newlines_tabs() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("trim(\"\n\thello\t\n\")").unwrap();
    assert_eq!(result, Value::String("hello".to_string()));
}

#[test]
fn test_trim_no_whitespace() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"trim("hello")"#).unwrap();
    assert_eq!(result, Value::String("hello".to_string()));
}

#[test]
fn test_trim_start_basic() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"trim_start("  hello  ")"#).unwrap();
    assert_eq!(result, Value::String("hello  ".to_string()));
}

#[test]
fn test_trim_end_basic() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"trim_end("  hello  ")"#).unwrap();
    assert_eq!(result, Value::String("  hello".to_string()));
}

#[test]
fn test_trim_empty_string() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"trim("   ")"#).unwrap();
    assert_eq!(result, Value::String("".to_string()));
}

// ============================================================================
// Search Functions Tests
// ============================================================================

#[test]
fn test_starts_with_true() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"starts_with("hello world", "hello")"#).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_starts_with_false() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"starts_with("hello world", "world")"#).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_starts_with_exact_match() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"starts_with("hello", "hello")"#).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_starts_with_empty() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"starts_with("hello", "")"#).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_ends_with_true() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"ends_with("hello world", "world")"#).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_ends_with_false() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"ends_with("hello world", "hello")"#).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_ends_with_exact_match() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"ends_with("world", "world")"#).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

// ============================================================================
// Manipulation Functions Tests
// ============================================================================

#[test]
fn test_replace_basic() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"replace("hello world", "world", "rust")"#).unwrap();
    assert_eq!(result, Value::String("hello rust".to_string()));
}

#[test]
fn test_replace_multiple_occurrences() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"replace("aaa", "a", "b")"#).unwrap();
    assert_eq!(result, Value::String("bbb".to_string()));
}

#[test]
fn test_replace_no_match() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"replace("hello", "xyz", "abc")"#).unwrap();
    assert_eq!(result, Value::String("hello".to_string()));
}

#[test]
fn test_replace_empty_pattern() {
    let mut evaluator = Evaluator::new();

    // Replacing empty string inserts replacement at every position
    let result = evaluator.eval_str(r#"replace("abc", "", "x")"#).unwrap();
    assert_eq!(result, Value::String("xaxbxcx".to_string()));
}

#[test]
fn test_split_basic() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"split("a,b,c", ",")"#).unwrap();

    match result {
        Value::Vector(vec) => {
            assert_eq!(vec.len(), 3);
            assert_eq!(vec[0], Value::String("a".to_string()));
            assert_eq!(vec[1], Value::String("b".to_string()));
            assert_eq!(vec[2], Value::String("c".to_string()));
        }
        _ => panic!("Expected Vector"),
    }
}

#[test]
fn test_split_spaces() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"split("hello world test", " ")"#).unwrap();

    match result {
        Value::Vector(vec) => {
            assert_eq!(vec.len(), 3);
            assert_eq!(vec[0], Value::String("hello".to_string()));
            assert_eq!(vec[1], Value::String("world".to_string()));
            assert_eq!(vec[2], Value::String("test".to_string()));
        }
        _ => panic!("Expected Vector"),
    }
}

#[test]
fn test_split_no_delimiter() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"split("hello", ",")"#).unwrap();

    match result {
        Value::Vector(vec) => {
            assert_eq!(vec.len(), 1);
            assert_eq!(vec[0], Value::String("hello".to_string()));
        }
        _ => panic!("Expected Vector"),
    }
}

#[test]
fn test_join_basic() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"join(["a", "b", "c"], ",")"#).unwrap();
    assert_eq!(result, Value::String("a,b,c".to_string()));
}

#[test]
fn test_join_spaces() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"join(["hello", "world"], " ")"#).unwrap();
    assert_eq!(result, Value::String("hello world".to_string()));
}

#[test]
fn test_join_empty_delimiter() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"join(["a", "b", "c"], "")"#).unwrap();
    assert_eq!(result, Value::String("abc".to_string()));
}

#[test]
fn test_join_single_element() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"join(["hello"], ",")"#).unwrap();
    assert_eq!(result, Value::String("hello".to_string()));
}

#[test]
fn test_split_then_join_roundtrip() {
    let mut evaluator = Evaluator::new();

    let code = r#"
        let original = "a,b,c";
        let parts = split(original, ",");
        join(parts, ",")
    "#;
    let result = evaluator.eval_str(code).unwrap();
    assert_eq!(result, Value::String("a,b,c".to_string()));
}

// ============================================================================
// Padding Functions Tests
// ============================================================================

#[test]
fn test_pad_start_with_spaces() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"pad_start("5", 3)"#).unwrap();
    assert_eq!(result, Value::String("  5".to_string()));
}

#[test]
fn test_pad_start_with_zeros() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"pad_start("5", 3, "0")"#).unwrap();
    assert_eq!(result, Value::String("005".to_string()));
}

#[test]
fn test_pad_start_already_long() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"pad_start("hello", 3)"#).unwrap();
    assert_eq!(result, Value::String("hello".to_string()));
}

#[test]
fn test_pad_start_exact_length() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"pad_start("abc", 3)"#).unwrap();
    assert_eq!(result, Value::String("abc".to_string()));
}

#[test]
fn test_pad_end_with_spaces() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"pad_end("5", 3)"#).unwrap();
    assert_eq!(result, Value::String("5  ".to_string()));
}

#[test]
fn test_pad_end_with_zeros() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"pad_end("5", 3, "0")"#).unwrap();
    assert_eq!(result, Value::String("500".to_string()));
}

#[test]
fn test_pad_end_already_long() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"pad_end("hello", 3)"#).unwrap();
    assert_eq!(result, Value::String("hello".to_string()));
}

#[test]
fn test_pad_start_large_number() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"pad_start("42", 10, "0")"#).unwrap();
    assert_eq!(result, Value::String("0000000042".to_string()));
}

// ============================================================================
// Combined/Complex Tests
// ============================================================================

#[test]
fn test_case_and_trim_combined() {
    let mut evaluator = Evaluator::new();

    let code = r#"
        let messy = "  HELLO  ";
        let cleaned = trim(messy);
        lower(cleaned)
    "#;
    let result = evaluator.eval_str(code).unwrap();
    assert_eq!(result, Value::String("hello".to_string()));
}

#[test]
fn test_split_map_join() {
    let mut evaluator = Evaluator::new();

    let code = r#"
        let sentence = "hello world test";
        let words = split(sentence, " ");
        let uppercased = map(w => upper(w), words);
        join(uppercased, " ")
    "#;
    let result = evaluator.eval_str(code).unwrap();
    assert_eq!(result, Value::String("HELLO WORLD TEST".to_string()));
}

#[test]
fn test_concatenation_with_plus_and_trim() {
    let mut evaluator = Evaluator::new();

    let code = r#"
        let a = "  hello  ";
        let b = "  world  ";
        trim(a) + " " + trim(b)
    "#;
    let result = evaluator.eval_str(code).unwrap();
    assert_eq!(result, Value::String("hello world".to_string()));
}

#[test]
fn test_replace_then_split() {
    let mut evaluator = Evaluator::new();

    let code = r#"
        let text = "a;b;c";
        let normalized = replace(text, ";", ",");
        split(normalized, ",")
    "#;
    let result = evaluator.eval_str(code).unwrap();

    match result {
        Value::Vector(vec) => {
            assert_eq!(vec.len(), 3);
            assert_eq!(vec[0], Value::String("a".to_string()));
        }
        _ => panic!("Expected Vector"),
    }
}

#[test]
fn test_padding_for_table_formatting() {
    let mut evaluator = Evaluator::new();

    let code = r#"
        let name = "Alice";
        let age = "25";
        let padded_name = pad_end(name, 10);
        let padded_age = pad_start(age, 5);
        padded_name + padded_age
    "#;
    let result = evaluator.eval_str(code).unwrap();
    assert_eq!(result, Value::String("Alice        25".to_string()));
}

// ============================================================================
// Legacy Function Tests
// ============================================================================

#[test]
fn test_concat_function() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"concat("hello", " world")"#).unwrap();
    assert_eq!(result, Value::String("hello world".to_string()));
}

#[test]
fn test_length_function() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"length("hello")"#).unwrap();
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_length_empty() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"length("")"#).unwrap();
    assert_eq!(result, Value::Number(0.0));
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_pad_start_wrong_arity() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"pad_start("hello")"#);
    assert!(result.is_err());
}

#[test]
fn test_pad_start_multi_char_fill() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"pad_start("5", 3, "00")"#);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("single character"));
}

// Note: Arrays of numbers get converted to tensors automatically,
// so we can't easily test join() with numeric arrays.
// The function will correctly reject tensors as they're not Vector type.
