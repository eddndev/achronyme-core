mod test_common;
use test_common::eval;
use achronyme_types::value::Value;

// ============================================================================
// Topological Sort Tests
// ============================================================================

#[test]
fn test_topological_sort_simple_dag() {
    let result = eval(r#"
        let g = network([
            A -> B,
            B -> C,
            A -> C
        ])
        topological_sort(g)
    "#).unwrap();

    match result {
        Value::Vector(nodes) => {
            assert_eq!(nodes.len(), 3);
            // A must come before B and C
            // B must come before C
            let pos_a = nodes.iter().position(|n| matches!(n, Value::String(s) if s == "A")).unwrap();
            let pos_b = nodes.iter().position(|n| matches!(n, Value::String(s) if s == "B")).unwrap();
            let pos_c = nodes.iter().position(|n| matches!(n, Value::String(s) if s == "C")).unwrap();

            assert!(pos_a < pos_b);
            assert!(pos_a < pos_c);
            assert!(pos_b < pos_c);
        }
        _ => panic!("Expected vector"),
    }
}

#[test]
fn test_topological_sort_with_cycle() {
    let result = eval(r#"
        let g = network([
            A -> B,
            B -> C,
            C -> A
        ])
        topological_sort(g)
    "#);

    match result {
        Err(msg) => {
            assert!(msg.contains("Directed Acyclic Graph"));
            assert!(msg.contains("cycles"));
        }
        Ok(_) => panic!("Expected error for graph with cycle"),
    }
}

#[test]
fn test_topological_sort_complex_dag() {
    let result = eval(r#"
        let g = network([
            Task1 -> Task3,
            Task2 -> Task3,
            Task3 -> Task4,
            Task1 -> Task4
        ])
        topological_sort(g)
    "#).unwrap();

    match result {
        Value::Vector(nodes) => {
            assert_eq!(nodes.len(), 4);

            // Task3 must come before Task4
            let pos_3 = nodes.iter().position(|n| matches!(n, Value::String(s) if s == "Task3")).unwrap();
            let pos_4 = nodes.iter().position(|n| matches!(n, Value::String(s) if s == "Task4")).unwrap();
            assert!(pos_3 < pos_4);
        }
        _ => panic!("Expected vector"),
    }
}

#[test]
fn test_topological_sort_dependencies() {
    // Real-world example: build system dependencies
    let result = eval(r#"
        let g = network([
            Compile -> Link,
            Link -> Deploy,
            Test -> Deploy
        ])
        topological_sort(g)
    "#).unwrap();

    match result {
        Value::Vector(nodes) => {
            assert_eq!(nodes.len(), 4);

            // Deploy must be last
            match nodes.last() {
                Some(Value::String(s)) => assert_eq!(s, "Deploy"),
                _ => panic!("Expected Deploy to be last"),
            }
        }
        _ => panic!("Expected vector"),
    }
}
