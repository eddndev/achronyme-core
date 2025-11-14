mod test_common;
use test_common::eval;
use achronyme_types::value::Value;

// ============================================================================
// Dijkstra Algorithm Tests
// ============================================================================

#[test]
fn test_dijkstra_simple_path() {
    let result = eval(r#"
        let g = network([
            A -> B: {weight: 4},
            A -> C: {weight: 2},
            B -> D: {weight: 5},
            C -> D: {weight: 1},
            D -> E: {weight: 3}
        ])
        dijkstra(g, "A", "E")
    "#).unwrap();

    match result {
        Value::Record(map) => {
            // Check found
            match map.get("found") {
                Some(Value::Boolean(b)) => assert!(b),
                _ => panic!("Expected found field"),
            }

            // Check distance (A -> C -> D -> E = 2 + 1 + 3 = 6)
            match map.get("distance") {
                Some(Value::Number(d)) => assert_eq!(*d, 6.0),
                _ => panic!("Expected distance field"),
            }

            // Check path
            match map.get("path") {
                Some(Value::Vector(path)) => {
                    assert_eq!(path.len(), 4); // A, C, D, E
                    match (&path[0], &path[1], &path[2], &path[3]) {
                        (Value::String(a), Value::String(c), Value::String(d), Value::String(e)) => {
                            assert_eq!(a, "A");
                            assert_eq!(c, "C");
                            assert_eq!(d, "D");
                            assert_eq!(e, "E");
                        }
                        _ => panic!("Expected string nodes"),
                    }
                }
                _ => panic!("Expected path field"),
            }
        }
        _ => panic!("Expected record"),
    }
}

#[test]
fn test_dijkstra_no_path() {
    let result = eval(r#"
        let g = network([
            A -> B: {weight: 1},
            C -> D: {weight: 2}
        ])
        dijkstra(g, "A", "D")
    "#).unwrap();

    match result {
        Value::Record(map) => {
            match map.get("found") {
                Some(Value::Boolean(b)) => assert!(!b),
                _ => panic!("Expected found field"),
            }
            match map.get("path") {
                Some(Value::Vector(path)) => assert!(path.is_empty()),
                _ => panic!("Expected path field"),
            }
        }
        _ => panic!("Expected record"),
    }
}

#[test]
fn test_dijkstra_undirected() {
    let result = eval(r#"
        let g = network([
            A <> B: {weight: 1},
            B <> C: {weight: 2},
            C <> D: {weight: 3}
        ])
        dijkstra(g, "A", "D")
    "#).unwrap();

    match result {
        Value::Record(map) => {
            match map.get("found") {
                Some(Value::Boolean(b)) => assert!(b),
                _ => panic!("Expected found field"),
            }
            match map.get("distance") {
                Some(Value::Number(d)) => assert_eq!(*d, 6.0), // 1 + 2 + 3
                _ => panic!("Expected distance field"),
            }
        }
        _ => panic!("Expected record"),
    }
}

#[test]
fn test_dijkstra_missing_weight() {
    let result = eval(r#"
        let g = network([
            A -> B: {weight: 1},
            B -> C
        ])
        dijkstra(g, "A", "C")
    "#);

    match result {
        Err(msg) => {
            assert!(msg.contains("dijkstra() requires all edges to have a 'weight' property"));
        }
        Ok(_) => panic!("Expected error for missing weight"),
    }
}

#[test]
fn test_dijkstra_negative_weight() {
    let result = eval(r#"
        let g = network([
            A -> B: {weight: -1},
            B -> C: {weight: 2}
        ])
        dijkstra(g, "A", "C")
    "#);

    match result {
        Err(msg) => {
            assert!(msg.contains("dijkstra() requires all weights to be positive numbers"));
        }
        Ok(_) => panic!("Expected error for negative weight"),
    }
}

#[test]
fn test_dijkstra_chooses_shortest() {
    // Test that Dijkstra chooses the shortest weighted path, not shortest number of edges
    let result = eval(r#"
        let g = network([
            A -> B: {weight: 1},
            A -> C: {weight: 10},
            B -> C: {weight: 1}
        ])
        dijkstra(g, "A", "C")
    "#).unwrap();

    match result {
        Value::Record(map) => {
            // Should choose A -> B -> C (cost 2) over A -> C (cost 10)
            match map.get("distance") {
                Some(Value::Number(d)) => assert_eq!(*d, 2.0),
                _ => panic!("Expected distance field"),
            }
            match map.get("path") {
                Some(Value::Vector(path)) => {
                    assert_eq!(path.len(), 3); // A, B, C
                }
                _ => panic!("Expected path field"),
            }
        }
        _ => panic!("Expected record"),
    }
}
