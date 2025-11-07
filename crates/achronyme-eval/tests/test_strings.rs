use achronyme_eval::Evaluator;
use achronyme_parser::parse;
use achronyme_types::value::Value;

fn eval(source: &str) -> Result<Value, String> {
    let statements = parse(source)?;
    let mut evaluator = Evaluator::new();

    let mut result = Value::Number(0.0);
    for stmt in &statements {
        result = evaluator.evaluate(stmt)?;
    }

    Ok(result)
}

#[test]
fn test_string_literal_basic() {
    let result = eval(r#""hello""#).unwrap();
    assert_eq!(result, Value::String("hello".to_string()));
}

#[test]
fn test_string_literal_empty() {
    let result = eval(r#""""#).unwrap();
    assert_eq!(result, Value::String("".to_string()));
}

#[test]
fn test_string_literal_with_spaces() {
    let result = eval(r#""hello world""#).unwrap();
    assert_eq!(result, Value::String("hello world".to_string()));
}

#[test]
fn test_string_escape_newline() {
    let result = eval(r#""hello\nworld""#).unwrap();
    assert_eq!(result, Value::String("hello\nworld".to_string()));
}

#[test]
fn test_string_escape_tab() {
    let result = eval(r#""tab\there""#).unwrap();
    assert_eq!(result, Value::String("tab\there".to_string()));
}

#[test]
fn test_string_escape_carriage_return() {
    let result = eval(r#""line\rreturn""#).unwrap();
    assert_eq!(result, Value::String("line\rreturn".to_string()));
}

#[test]
fn test_string_escape_backslash() {
    let result = eval(r#""back\\slash""#).unwrap();
    assert_eq!(result, Value::String("back\\slash".to_string()));
}

#[test]
fn test_string_escape_quote() {
    let result = eval(r#""say \"hello\"""#).unwrap();
    assert_eq!(result, Value::String("say \"hello\"".to_string()));
}

#[test]
fn test_string_multiple_escapes() {
    let result = eval(r#""line1\nline2\tindented\\backslash""#).unwrap();
    assert_eq!(result, Value::String("line1\nline2\tindented\\backslash".to_string()));
}

#[test]
fn test_concat_function() {
    let result = eval(r#"concat("hello", " world")"#).unwrap();
    assert_eq!(result, Value::String("hello world".to_string()));
}

#[test]
fn test_concat_empty_strings() {
    let result = eval(r#"concat("", "")"#).unwrap();
    assert_eq!(result, Value::String("".to_string()));
}

#[test]
fn test_concat_one_empty() {
    let result = eval(r#"concat("hello", "")"#).unwrap();
    assert_eq!(result, Value::String("hello".to_string()));
}

#[test]
fn test_concat_with_variable() {
    let result = eval(r#"
        let greeting = "Hello"
        concat(greeting, " World")
    "#).unwrap();
    assert_eq!(result, Value::String("Hello World".to_string()));
}

#[test]
fn test_concat_error_non_string() {
    let result = eval(r#"concat("hello", 123)"#);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("concat() requires two strings"));
}

#[test]
fn test_length_function() {
    let result = eval(r#"length("hello")"#).unwrap();
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_length_empty_string() {
    let result = eval(r#"length("")"#).unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_length_with_spaces() {
    let result = eval(r#"length("hello world")"#).unwrap();
    assert_eq!(result, Value::Number(11.0));
}

#[test]
fn test_length_with_variable() {
    let result = eval(r#"
        let text = "testing"
        length(text)
    "#).unwrap();
    assert_eq!(result, Value::Number(7.0));
}

#[test]
fn test_length_error_non_string() {
    let result = eval(r#"length(123)"#);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("length() requires a string"));
}

#[test]
fn test_string_equality() {
    let result = eval(r#""hello" == "hello""#).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_string_inequality_same() {
    let result = eval(r#""hello" != "hello""#).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_string_inequality_different() {
    let result = eval(r#""hello" != "world""#).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_string_equality_different() {
    let result = eval(r#""hello" == "world""#).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_string_equality_empty() {
    let result = eval(r#"
        let empty1 = ""
        let empty2 = ""
        empty1 == empty2
    "#).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_string_comparison_with_variable() {
    let result = eval(r#"
        let name = "Alice"
        name == "Alice"
    "#).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_string_variable_assignment() {
    let result = eval(r#"
        let message = "Hello, Achronyme!"
        message
    "#).unwrap();
    assert_eq!(result, Value::String("Hello, Achronyme!".to_string()));
}

#[test]
fn test_nested_concat() {
    let result = eval(r#"concat(concat("Hello", " "), "World")"#).unwrap();
    assert_eq!(result, Value::String("Hello World".to_string()));
}

#[test]
fn test_string_in_expression() {
    let result = eval(r#"
        let first = "Alice"
        let second = "Bob"
        let result = concat(first, concat(" and ", second))
        result
    "#).unwrap();
    assert_eq!(result, Value::String("Alice and Bob".to_string()));
}

#[test]
fn test_string_length_in_comparison() {
    let result = eval(r#"length("hello") == 5"#).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_unicode_string() {
    let result = eval(r#""¡Hola Mundo! 你好 🌍""#).unwrap();
    assert_eq!(result, Value::String("¡Hola Mundo! 你好 🌍".to_string()));
}

#[test]
fn test_string_with_numbers() {
    let result = eval(r#""Number: 42""#).unwrap();
    assert_eq!(result, Value::String("Number: 42".to_string()));
}
