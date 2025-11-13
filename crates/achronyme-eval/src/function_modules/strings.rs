/// String utility functions
///
/// This module provides comprehensive string manipulation and analysis functions.
///
/// Categories:
/// - Case conversion: upper, lower
/// - Whitespace: trim, trim_start, trim_end
/// - Search: contains, starts_with, ends_with
/// - Manipulation: replace, split, join
/// - Padding: pad_start, pad_end
/// - Legacy: concat, length

use crate::functions::FunctionRegistry;
use achronyme_types::value::Value;
use achronyme_types::Environment;

pub fn register_functions(registry: &mut FunctionRegistry) {
    // Legacy functions (kept for compatibility)
    registry.register("concat", concat, 2);
    registry.register("length", length, 1);

    // Case conversion
    registry.register("upper", upper, 1);
    registry.register("lower", lower, 1);

    // Whitespace handling
    registry.register("trim", trim, 1);
    registry.register("trim_start", trim_start, 1);
    registry.register("trim_end", trim_end, 1);

    // Search functions (Note: contains is also in array module)
    registry.register("starts_with", starts_with, 2);
    registry.register("ends_with", ends_with, 2);

    // Manipulation
    registry.register("replace", replace, 3);
    registry.register("split", split, 2);
    registry.register("join", join, 2);

    // Padding
    registry.register("pad_start", pad_start, -1); // 2 or 3 args
    registry.register("pad_end", pad_end, -1);     // 2 or 3 args
}

// ============================================================================
// Legacy Functions
// ============================================================================

/// Concatenate two strings
///
/// Note: String concatenation is now also available via the + operator
///
/// Examples:
/// - concat("hello", " world") => "hello world"
/// - "hello" + " world" => "hello world"
fn concat(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::String(s1), Value::String(s2)) => {
            Ok(Value::String(format!("{}{}", s1, s2)))
        }
        _ => Err("concat() requires two strings".to_string()),
    }
}

/// Get the length of a string
///
/// Examples:
/// - length("hello") => 5
/// - length("") => 0
fn length(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match &args[0] {
        Value::String(s) => Ok(Value::Number(s.len() as f64)),
        _ => Err("length() requires a string".to_string()),
    }
}

// ============================================================================
// Case Conversion
// ============================================================================

/// Convert string to uppercase
///
/// Examples:
/// - upper("hello") => "HELLO"
/// - upper("Hello World") => "HELLO WORLD"
fn upper(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match &args[0] {
        Value::String(s) => Ok(Value::String(s.to_uppercase())),
        _ => Err("upper() requires a string".to_string()),
    }
}

/// Convert string to lowercase
///
/// Examples:
/// - lower("HELLO") => "hello"
/// - lower("Hello World") => "hello world"
fn lower(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match &args[0] {
        Value::String(s) => Ok(Value::String(s.to_lowercase())),
        _ => Err("lower() requires a string".to_string()),
    }
}

// ============================================================================
// Whitespace Handling
// ============================================================================

/// Remove whitespace from both ends of a string
///
/// Examples:
/// - trim("  hello  ") => "hello"
/// - trim("\n\thello\t\n") => "hello"
fn trim(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match &args[0] {
        Value::String(s) => Ok(Value::String(s.trim().to_string())),
        _ => Err("trim() requires a string".to_string()),
    }
}

/// Remove whitespace from the start of a string
///
/// Examples:
/// - trim_start("  hello  ") => "hello  "
/// - trim_start("\n\thello") => "hello"
fn trim_start(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match &args[0] {
        Value::String(s) => Ok(Value::String(s.trim_start().to_string())),
        _ => Err("trim_start() requires a string".to_string()),
    }
}

/// Remove whitespace from the end of a string
///
/// Examples:
/// - trim_end("  hello  ") => "  hello"
/// - trim_end("hello\n\t") => "hello"
fn trim_end(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match &args[0] {
        Value::String(s) => Ok(Value::String(s.trim_end().to_string())),
        _ => Err("trim_end() requires a string".to_string()),
    }
}

// ============================================================================
// Search Functions
// ============================================================================

/// Check if string starts with a prefix
///
/// Examples:
/// - starts_with("hello world", "hello") => true
/// - starts_with("hello world", "world") => false
fn starts_with(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::String(s), Value::String(prefix)) => {
            Ok(Value::Boolean(s.starts_with(prefix)))
        }
        _ => Err("starts_with() requires two strings".to_string()),
    }
}

/// Check if string ends with a suffix
///
/// Examples:
/// - ends_with("hello world", "world") => true
/// - ends_with("hello world", "hello") => false
fn ends_with(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::String(s), Value::String(suffix)) => {
            Ok(Value::Boolean(s.ends_with(suffix)))
        }
        _ => Err("ends_with() requires two strings".to_string()),
    }
}

// ============================================================================
// Manipulation Functions
// ============================================================================

/// Replace all occurrences of a pattern with a replacement
///
/// Examples:
/// - replace("hello world", "world", "rust") => "hello rust"
/// - replace("aaa", "a", "b") => "bbb"
fn replace(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match (&args[0], &args[1], &args[2]) {
        (Value::String(s), Value::String(pattern), Value::String(replacement)) => {
            Ok(Value::String(s.replace(pattern, replacement)))
        }
        _ => Err("replace() requires three strings".to_string()),
    }
}

/// Split a string by a delimiter into an array of strings
///
/// Examples:
/// - split("a,b,c", ",") => ["a", "b", "c"]
/// - split("hello world", " ") => ["hello", "world"]
fn split(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::String(s), Value::String(delimiter)) => {
            let parts: Vec<Value> = s
                .split(delimiter.as_str())
                .map(|part| Value::String(part.to_string()))
                .collect();
            Ok(Value::Vector(parts))
        }
        _ => Err("split() requires two strings".to_string()),
    }
}

/// Join an array of strings with a delimiter
///
/// Examples:
/// - join(["a", "b", "c"], ",") => "a,b,c"
/// - join(["hello", "world"], " ") => "hello world"
fn join(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Vector(vec), Value::String(delimiter)) => {
            let strings: Result<Vec<String>, String> = vec
                .iter()
                .map(|v| match v {
                    Value::String(s) => Ok(s.clone()),
                    _ => Err("join() requires an array of strings".to_string()),
                })
                .collect();

            match strings {
                Ok(parts) => Ok(Value::String(parts.join(delimiter))),
                Err(e) => Err(e),
            }
        }
        _ => Err("join() requires an array and a string".to_string()),
    }
}

// ============================================================================
// Padding Functions
// ============================================================================

/// Pad string at the start to a target length
///
/// Examples:
/// - pad_start("5", 3) => "  5"
/// - pad_start("5", 3, "0") => "005"
/// - pad_start("hello", 3) => "hello" (no padding if already long enough)
fn pad_start(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    if args.len() < 2 || args.len() > 3 {
        return Err("pad_start() expects 2 or 3 arguments: pad_start(string, length, fill_char?)".to_string());
    }

    let s = match &args[0] {
        Value::String(s) => s,
        _ => return Err("pad_start() first argument must be a string".to_string()),
    };

    let target_len = match &args[1] {
        Value::Number(n) => *n as usize,
        _ => return Err("pad_start() second argument must be a number".to_string()),
    };

    let fill_char = if args.len() == 3 {
        match &args[2] {
            Value::String(f) => {
                if f.len() != 1 {
                    return Err("pad_start() fill character must be a single character".to_string());
                }
                f.chars().next().unwrap()
            }
            _ => return Err("pad_start() third argument must be a string".to_string()),
        }
    } else {
        ' '
    };

    if s.len() >= target_len {
        return Ok(Value::String(s.clone()));
    }

    let padding_len = target_len - s.len();
    let padding: String = (0..padding_len).map(|_| fill_char).collect();
    Ok(Value::String(format!("{}{}", padding, s)))
}

/// Pad string at the end to a target length
///
/// Examples:
/// - pad_end("5", 3) => "5  "
/// - pad_end("5", 3, "0") => "500"
/// - pad_end("hello", 3) => "hello" (no padding if already long enough)
fn pad_end(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    if args.len() < 2 || args.len() > 3 {
        return Err("pad_end() expects 2 or 3 arguments: pad_end(string, length, fill_char?)".to_string());
    }

    let s = match &args[0] {
        Value::String(s) => s,
        _ => return Err("pad_end() first argument must be a string".to_string()),
    };

    let target_len = match &args[1] {
        Value::Number(n) => *n as usize,
        _ => return Err("pad_end() second argument must be a number".to_string()),
    };

    let fill_char = if args.len() == 3 {
        match &args[2] {
            Value::String(f) => {
                if f.len() != 1 {
                    return Err("pad_end() fill character must be a single character".to_string());
                }
                f.chars().next().unwrap()
            }
            _ => return Err("pad_end() third argument must be a string".to_string()),
        }
    } else {
        ' '
    };

    if s.len() >= target_len {
        return Ok(Value::String(s.clone()));
    }

    let padding_len = target_len - s.len();
    let padding: String = (0..padding_len).map(|_| fill_char).collect();
    Ok(Value::String(format!("{}{}", s, padding)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upper() {
        let mut env = Environment::new();
        let args = vec![Value::String("hello".to_string())];
        let result = upper(&args, &mut env).unwrap();
        assert_eq!(result, Value::String("HELLO".to_string()));
    }

    #[test]
    fn test_lower() {
        let mut env = Environment::new();
        let args = vec![Value::String("HELLO".to_string())];
        let result = lower(&args, &mut env).unwrap();
        assert_eq!(result, Value::String("hello".to_string()));
    }

    #[test]
    fn test_trim() {
        let mut env = Environment::new();
        let args = vec![Value::String("  hello  ".to_string())];
        let result = trim(&args, &mut env).unwrap();
        assert_eq!(result, Value::String("hello".to_string()));
    }

    #[test]
    fn test_starts_with() {
        let mut env = Environment::new();
        let args = vec![
            Value::String("hello world".to_string()),
            Value::String("hello".to_string()),
        ];
        let result = starts_with(&args, &mut env).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_split() {
        let mut env = Environment::new();
        let args = vec![
            Value::String("a,b,c".to_string()),
            Value::String(",".to_string()),
        ];
        let result = split(&args, &mut env).unwrap();
        match result {
            Value::Vector(vec) => {
                assert_eq!(vec.len(), 3);
                assert_eq!(vec[0], Value::String("a".to_string()));
            }
            _ => panic!("Expected Vector"),
        }
    }

    #[test]
    fn test_pad_start() {
        let mut env = Environment::new();
        let args = vec![
            Value::String("5".to_string()),
            Value::Number(3.0),
            Value::String("0".to_string()),
        ];
        let result = pad_start(&args, &mut env).unwrap();
        assert_eq!(result, Value::String("005".to_string()));
    }
}
