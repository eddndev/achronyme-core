use achronyme_eval::Evaluator;
use achronyme_types::value::Value;

fn eval(source: &str) -> Result<Value, String> {
    let mut evaluator = Evaluator::new();
    evaluator.eval_str(source)
}

#[test]
fn test_array_spread_basic() {
    let result = eval("[...[1,2], ...[3,4]]").unwrap();
    if let Value::Tensor(t) = result {
        assert_eq!(t.data(), &[1.0, 2.0, 3.0, 4.0]);
    } else {
        panic!("Expected Tensor, got {:?}", result);
    }
}

#[test]
fn test_record_spread_basic() {
    let result = eval("{ ...{a:1}, ...{b:2} }").unwrap();
    if let Value::Record(map) = result {
        assert_eq!(map.len(), 2);
        assert!(matches!(map.get("a"), Some(Value::Number(1.0))));
        assert!(matches!(map.get("b"), Some(Value::Number(2.0))));
    } else {
        panic!("Expected Record, got {:?}", result);
    }
}

#[test]
fn test_record_spread_override() {
    let result = eval("{ a:1, ...{a:2} }").unwrap();
    if let Value::Record(map) = result {
        assert_eq!(map.len(), 1);
        assert!(matches!(map.get("a"), Some(Value::Number(2.0))));
    } else {
        panic!("Expected Record, got {:?}", result);
    }
}

#[test]
fn test_array_spread_multiple() {
    let result = eval("[...[1], ...[2], ...[3]]").unwrap();
    if let Value::Tensor(t) = result {
        assert_eq!(t.data(), &[1.0, 2.0, 3.0]);
    } else {
        panic!("Expected Tensor, got {:?}", result);
    }
}

#[test]
fn test_array_spread_mixed() {
    let result = eval("[0, ...[1,2], 3]").unwrap();
    if let Value::Tensor(t) = result {
        assert_eq!(t.data(), &[0.0, 1.0, 2.0, 3.0]);
    } else {
        panic!("Expected Tensor, got {:?}", result);
    }
}

#[test]
fn test_spread_with_variables() {
    let result = eval(r#"
        let nums1 = [1, 2, 3];
        let nums2 = [4, 5, 6];
        [...nums1, ...nums2]
    "#).unwrap();
    if let Value::Tensor(t) = result {
        assert_eq!(t.data(), &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    } else {
        panic!("Expected Tensor, got {:?}", result);
    }
}

#[test]
fn test_record_spread_precedence() {
    let result = eval(r#"
        let defaults = { a: 1, b: 2, c: 3 };
        let overrides = { b: 10, c: 20 };
        { ...defaults, ...overrides }
    "#).unwrap();
    if let Value::Record(map) = result {
        assert_eq!(map.len(), 3);
        assert!(matches!(map.get("a"), Some(Value::Number(1.0))));
        assert!(matches!(map.get("b"), Some(Value::Number(10.0))));
        assert!(matches!(map.get("c"), Some(Value::Number(20.0))));
    } else {
        panic!("Expected Record, got {:?}", result);
    }
}

#[test]
fn test_record_spread_reversed() {
    let result = eval(r#"
        let defaults = { a: 1, b: 2, c: 3 };
        let overrides = { b: 10, c: 20 };
        { ...overrides, ...defaults }
    "#).unwrap();
    if let Value::Record(map) = result {
        assert_eq!(map.len(), 3);
        assert!(matches!(map.get("a"), Some(Value::Number(1.0))));
        assert!(matches!(map.get("b"), Some(Value::Number(2.0))));
        assert!(matches!(map.get("c"), Some(Value::Number(3.0))));
    } else {
        panic!("Expected Record, got {:?}", result);
    }
}

#[test]
fn test_record_spread_triple_override() {
    let result = eval("{ ...{a:1}, ...{a:2}, ...{a:3} }").unwrap();
    if let Value::Record(map) = result {
        assert_eq!(map.len(), 1);
        assert!(matches!(map.get("a"), Some(Value::Number(3.0))));
    } else {
        panic!("Expected Record, got {:?}", result);
    }
}

#[test]
fn test_record_spread_with_literal() {
    let result = eval("{ ...{a:1, b:2}, a: 3 }").unwrap();
    if let Value::Record(map) = result {
        assert_eq!(map.len(), 2);
        assert!(matches!(map.get("a"), Some(Value::Number(3.0))));
        assert!(matches!(map.get("b"), Some(Value::Number(2.0))));
    } else {
        panic!("Expected Record, got {:?}", result);
    }
}

#[test]
fn test_record_spread_literal_first() {
    let result = eval("{ a: 1, ...{a:2, b:3} }").unwrap();
    if let Value::Record(map) = result {
        assert_eq!(map.len(), 2);
        assert!(matches!(map.get("a"), Some(Value::Number(2.0))));
        assert!(matches!(map.get("b"), Some(Value::Number(3.0))));
    } else {
        panic!("Expected Record, got {:?}", result);
    }
}

#[test]
fn test_array_spread_strings() {
    let result = eval(r#"
        let strs1 = ["a", "b"];
        let strs2 = ["c", "d"];
        [...strs1, ...strs2]
    "#).unwrap();
    if let Value::Vector(vec) = result {
        assert_eq!(vec.len(), 4);
        assert!(matches!(&vec[0], Value::String(s) if s == "a"));
        assert!(matches!(&vec[1], Value::String(s) if s == "b"));
        assert!(matches!(&vec[2], Value::String(s) if s == "c"));
        assert!(matches!(&vec[3], Value::String(s) if s == "d"));
    } else {
        panic!("Expected Vector, got {:?}", result);
    }
}

#[test]
fn test_spread_oop_pattern() {
    let result = eval(r#"
        let object_class = {
            value: 10,
            getValue: () => self.value
        };
        let obj = { ...object_class, value: 20 };
        obj.getValue()
    "#).unwrap();
    assert!(matches!(result, Value::Number(20.0)));
}

#[test]
fn test_spread_oop_method_override() {
    let result = eval(r#"
        let Base = { method: () => "base" };
        let Derived = { ...Base, method: () => "derived" };
        Derived.method()
    "#).unwrap();
    if let Value::String(s) = result {
        assert_eq!(s, "derived");
    } else {
        panic!("Expected String, got {:?}", result);
    }
}

#[test]
fn test_spread_composition() {
    let result = eval(r#"
        let Walkable = { walk: () => "Walking..." };
        let Swimmable = { swim: () => "Swimming..." };
        let Duck = { ...Walkable, ...Swimmable, name: "Duck" };
        Duck.walk()
    "#).unwrap();
    if let Value::String(s) = result {
        assert_eq!(s, "Walking...");
    } else {
        panic!("Expected String, got {:?}", result);
    }
}

#[test]
fn test_spread_error_non_vector() {
    let result = eval("[...5]");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Cannot spread non-iterable"));
}

#[test]
fn test_spread_error_record_in_array() {
    let result = eval("[...{a:1}]");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Cannot spread non-iterable"));
}

#[test]
fn test_spread_error_array_in_record() {
    let result = eval("{ ...[1,2] }");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Cannot spread non-Record"));
}

#[test]
fn test_spread_error_number_in_record() {
    let result = eval("{ ...42 }");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Cannot spread non-Record"));
}

#[test]
fn test_spread_error_tensor() {
    let result = eval(r#"
        let tensor = [[1, 2], [3, 4]];
        [...tensor]
    "#);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Cannot spread multi-dimensional Tensor"));
}

#[test]
fn test_spread_empty_arrays() {
    let result = eval("[...[]]").unwrap();
    if let Value::Vector(vec) = result {
        assert_eq!(vec.len(), 0);
    } else {
        panic!("Expected Vector, got {:?}", result);
    }
}

#[test]
fn test_spread_empty_records() {
    let result = eval("{ ...{} }").unwrap();
    if let Value::Record(map) = result {
        assert_eq!(map.len(), 0);
    } else {
        panic!("Expected Record, got {:?}", result);
    }
}
