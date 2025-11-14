mod test_common;
use test_common::eval;
use achronyme_types::value::Value;

// ============================================================================
// Has Cycle Tests
// ============================================================================

#[test]
fn test_has_cycle_directed_with_cycle() {
    let result = eval(r#"
        let g = network([
            A -> B,
            B -> C,
            C -> A
        ])
        has_cycle(g)
    "#).unwrap();

    match result {
        Value::Boolean(b) => assert!(b),
        _ => panic!("Expected boolean"),
    }
}

#[test]
fn test_has_cycle_directed_no_cycle() {
    let result = eval(r#"
        let g = network([
            A -> B,
            B -> C,
            C -> D
        ])
        has_cycle(g)
    "#).unwrap();

    match result {
        Value::Boolean(b) => assert!(!b),
        _ => panic!("Expected boolean"),
    }
}

#[test]
fn test_has_cycle_undirected_with_cycle() {
    let result = eval(r#"
        let g = network([
            A <> B,
            B <> C,
            C <> A
        ])
        has_cycle(g)
    "#).unwrap();

    match result {
        Value::Boolean(b) => assert!(b),
        _ => panic!("Expected boolean"),
    }
}

#[test]
fn test_has_cycle_undirected_no_cycle() {
    // Tree structure has no cycles
    let result = eval(r#"
        let g = network([
            A <> B,
            A <> C,
            B <> D,
            B <> E
        ])
        has_cycle(g)
    "#).unwrap();

    match result {
        Value::Boolean(b) => assert!(!b),
        _ => panic!("Expected boolean"),
    }
}

#[test]
fn test_has_cycle_self_loop() {
    let result = eval(r#"
        let g = network([
            A -> A
        ])
        has_cycle(g)
    "#).unwrap();

    match result {
        Value::Boolean(b) => assert!(b),
        _ => panic!("Expected boolean"),
    }
}

#[test]
fn test_has_cycle_disconnected_with_cycle() {
    // One component has a cycle, another doesn't
    let result = eval(r#"
        let g = network([
            A -> B,
            B -> A,
            C -> D
        ])
        has_cycle(g)
    "#).unwrap();

    match result {
        Value::Boolean(b) => assert!(b),
        _ => panic!("Expected boolean"),
    }
}

#[test]
fn test_has_cycle_mixed_edges() {
    // Mixed directed and undirected - should be treated as directed
    let result = eval(r#"
        let g = network([
            A -> B,
            B <> C,
            C -> A
        ])
        has_cycle(g)
    "#).unwrap();

    match result {
        Value::Boolean(b) => assert!(b), // Has directed cycle A -> B -> C -> A
        _ => panic!("Expected boolean"),
    }
}

// ============================================================================
// Connected Components Tests
// ============================================================================

#[test]
fn test_connected_components_single() {
    let result = eval(r#"
        let g = network([
            A <> B,
            B <> C,
            C <> A
        ])
        connected_components(g)
    "#).unwrap();

    match result {
        Value::Vector(components) => {
            assert_eq!(components.len(), 1); // Only one component
            match &components[0] {
                Value::Vector(nodes) => assert_eq!(nodes.len(), 3),
                _ => panic!("Expected vector of nodes"),
            }
        }
        _ => panic!("Expected vector of components"),
    }
}

#[test]
fn test_connected_components_multiple() {
    let result = eval(r#"
        let g = network([
            A <> B,
            C <> D,
            E -> F
        ])
        connected_components(g)
    "#).unwrap();

    match result {
        Value::Vector(components) => {
            assert_eq!(components.len(), 3); // Three separate components
        }
        _ => panic!("Expected vector of components"),
    }
}

#[test]
fn test_connected_components_directed() {
    let result = eval(r#"
        let g = network([
            A -> B,
            B -> C,
            D -> E
        ])
        connected_components(g)
    "#).unwrap();

    match result {
        Value::Vector(components) => {
            // With directed edges, might have different connectivity
            assert!(components.len() >= 1);
        }
        _ => panic!("Expected vector of components"),
    }
}

// ============================================================================
// Is Connected Tests
// ============================================================================

#[test]
fn test_is_connected_true() {
    let result = eval(r#"
        let g = network([
            A <> B,
            B <> C,
            C <> D
        ])
        is_connected(g)
    "#).unwrap();

    match result {
        Value::Boolean(b) => assert!(b),
        _ => panic!("Expected boolean"),
    }
}

#[test]
fn test_is_connected_false() {
    let result = eval(r#"
        let g = network([
            A <> B,
            C <> D
        ])
        is_connected(g)
    "#).unwrap();

    match result {
        Value::Boolean(b) => assert!(!b),
        _ => panic!("Expected boolean"),
    }
}

#[test]
fn test_is_connected_directed_incomplete() {
    let result = eval(r#"
        let g = network([
            A -> B,
            C -> D
        ])
        is_connected(g)
    "#).unwrap();

    match result {
        Value::Boolean(b) => assert!(!b), // Disconnected
        _ => panic!("Expected boolean"),
    }
}

#[test]
fn test_is_connected_single_node() {
    let result = eval(r#"
        let g = network([
            A -> A
        ])
        is_connected(g)
    "#).unwrap();

    match result {
        Value::Boolean(b) => assert!(b), // Single node is connected
        _ => panic!("Expected boolean"),
    }
}
