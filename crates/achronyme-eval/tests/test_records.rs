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

// ============================================================================
// Record Literal Tests
// ============================================================================

#[test]
fn test_empty_record() {
    let result = eval("{}").unwrap();
    match result {
        Value::Record(map) => assert!(map.is_empty()),
        _ => panic!("Expected record"),
    }
}

#[test]
fn test_record_single_field() {
    let result = eval(r#"{ name: "Alice" }"#).unwrap();
    match result {
        Value::Record(map) => {
            assert_eq!(map.len(), 1);
            assert_eq!(map.get("name"), Some(&Value::String("Alice".to_string())));
        }
        _ => panic!("Expected record"),
    }
}

#[test]
fn test_record_multiple_fields() {
    let result = eval(r#"{ name: "Alice", age: 30, city: "Madrid" }"#).unwrap();
    match result {
        Value::Record(map) => {
            assert_eq!(map.len(), 3);
            assert_eq!(map.get("name"), Some(&Value::String("Alice".to_string())));
            assert_eq!(map.get("age"), Some(&Value::Number(30.0)));
            assert_eq!(map.get("city"), Some(&Value::String("Madrid".to_string())));
        }
        _ => panic!("Expected record"),
    }
}

#[test]
fn test_record_with_numeric_values() {
    let result = eval("{ x: 10, y: 20, z: 30 }").unwrap();
    match result {
        Value::Record(map) => {
            assert_eq!(map.len(), 3);
            assert_eq!(map.get("x"), Some(&Value::Number(10.0)));
            assert_eq!(map.get("y"), Some(&Value::Number(20.0)));
            assert_eq!(map.get("z"), Some(&Value::Number(30.0)));
        }
        _ => panic!("Expected record"),
    }
}

#[test]
fn test_record_with_expressions() {
    let result = eval("{ a: 1 + 2, b: 3 * 4, c: 10 / 2 }").unwrap();
    match result {
        Value::Record(map) => {
            assert_eq!(map.get("a"), Some(&Value::Number(3.0)));
            assert_eq!(map.get("b"), Some(&Value::Number(12.0)));
            assert_eq!(map.get("c"), Some(&Value::Number(5.0)));
        }
        _ => panic!("Expected record"),
    }
}

#[test]
fn test_record_with_boolean() {
    let result = eval("{ active: true, verified: false }").unwrap();
    match result {
        Value::Record(map) => {
            assert_eq!(map.get("active"), Some(&Value::Boolean(true)));
            assert_eq!(map.get("verified"), Some(&Value::Boolean(false)));
        }
        _ => panic!("Expected record"),
    }
}

#[test]
fn test_record_in_variable() {
    let result = eval(r#"
        let person = { name: "Bob", age: 25 }
        person
    "#).unwrap();
    match result {
        Value::Record(map) => {
            assert_eq!(map.get("name"), Some(&Value::String("Bob".to_string())));
            assert_eq!(map.get("age"), Some(&Value::Number(25.0)));
        }
        _ => panic!("Expected record"),
    }
}

// ============================================================================
// Field Access Tests
// ============================================================================

#[test]
fn test_field_access_string() {
    let result = eval(r#"
        let person = { name: "Alice", age: 30 }
        person.name
    "#).unwrap();
    assert_eq!(result, Value::String("Alice".to_string()));
}

#[test]
fn test_field_access_number() {
    let result = eval(r#"
        let person = { name: "Alice", age: 30 }
        person.age
    "#).unwrap();
    assert_eq!(result, Value::Number(30.0));
}

#[test]
fn test_field_access_from_literal() {
    let result = eval(r#"{ x: 100, y: 200 }.x"#).unwrap();
    assert_eq!(result, Value::Number(100.0));
}

#[test]
fn test_nested_record() {
    let result = eval(r#"
        let node = {
            id: "A",
            position: { x: 100, y: 200 }
        }
        node
    "#).unwrap();

    match result {
        Value::Record(map) => {
            assert_eq!(map.get("id"), Some(&Value::String("A".to_string())));
            match map.get("position") {
                Some(Value::Record(pos)) => {
                    assert_eq!(pos.get("x"), Some(&Value::Number(100.0)));
                    assert_eq!(pos.get("y"), Some(&Value::Number(200.0)));
                }
                _ => panic!("Expected nested record"),
            }
        }
        _ => panic!("Expected record"),
    }
}

#[test]
fn test_nested_field_access() {
    let result = eval(r#"
        let node = {
            id: "A",
            position: { x: 100, y: 200 }
        }
        node.position.x
    "#).unwrap();
    assert_eq!(result, Value::Number(100.0));
}

#[test]
fn test_nested_field_access_deep() {
    let result = eval(r#"
        let data = {
            user: {
                profile: {
                    name: "Alice"
                }
            }
        }
        data.user.profile.name
    "#).unwrap();
    assert_eq!(result, Value::String("Alice".to_string()));
}

#[test]
fn test_field_access_error_missing_field() {
    let result = eval(r#"
        let person = { name: "Alice" }
        person.age
    "#);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Field 'age' not found"));
}

#[test]
fn test_field_access_error_non_record() {
    let result = eval(r#"
        let x = 42
        x.field
    "#);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Cannot access field"));
}

// ============================================================================
// Record Function Tests
// ============================================================================

#[test]
fn test_keys_function() {
    let result = eval(r#"
        let rec = { a: 1, b: 2, c: 3 }
        keys(rec)
    "#).unwrap();
    // For now, keys() returns the count as a number
    assert_eq!(result, Value::Number(3.0));
}

#[test]
fn test_values_function() {
    let result = eval(r#"
        let rec = { a: 10, b: 20, c: 30 }
        values(rec)
    "#).unwrap();
    match result {
        Value::Vector(v) => {
            assert_eq!(v.len(), 3);
            // Note: HashMap doesn't guarantee order, so we just check the length
        }
        _ => panic!("Expected vector"),
    }
}

#[test]
fn test_has_field_true() {
    let result = eval(r#"
        let rec = { name: "Alice", age: 30 }
        has_field(rec, "name")
    "#).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_has_field_false() {
    let result = eval(r#"
        let rec = { name: "Alice" }
        has_field(rec, "age")
    "#).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

// ============================================================================
// Complex Record Tests
// ============================================================================

#[test]
fn test_record_with_vectors() {
    let result = eval(r#"
        let data = {
            values: [1, 2, 3],
            count: 3
        }
        data.count
    "#).unwrap();
    assert_eq!(result, Value::Number(3.0));
}

#[test]
fn test_record_field_in_expression() {
    let result = eval(r#"
        let point = { x: 10, y: 20 }
        point.x + point.y
    "#).unwrap();
    assert_eq!(result, Value::Number(30.0));
}

#[test]
fn test_multiple_records() {
    let result = eval(r#"
        let p1 = { x: 10, y: 20 }
        let p2 = { x: 30, y: 40 }
        p1.x + p2.y
    "#).unwrap();
    assert_eq!(result, Value::Number(50.0));
}

#[test]
fn test_record_field_reassignment() {
    let result = eval(r#"
        let p = { x: 10, y: 20 }
        let result = p.x
        result
    "#).unwrap();
    assert_eq!(result, Value::Number(10.0));
}
