mod test_common;
use test_common::eval;
use achronyme_types::value::Value;

// ============================================================================
// Edge Syntax Tests
// ============================================================================

#[test]
fn test_directed_edge() {
    let result = eval("A -> B").unwrap();
    match result {
        Value::Edge { from, to, directed, properties } => {
            assert_eq!(from, "A");
            assert_eq!(to, "B");
            assert!(directed);
            assert!(properties.is_empty());
        }
        _ => panic!("Expected directed edge"),
    }
}

#[test]
fn test_undirected_edge() {
    let result = eval("A <> B").unwrap();
    match result {
        Value::Edge { from, to, directed, properties } => {
            assert_eq!(from, "A");
            assert_eq!(to, "B");
            assert!(!directed);
            assert!(properties.is_empty());
        }
        _ => panic!("Expected undirected edge"),
    }
}

#[test]
fn test_edge_with_properties() {
    let result = eval("A -> B: {weight: 5, color: \"red\"}").unwrap();
    match result {
        Value::Edge { from, to, directed, properties } => {
            assert_eq!(from, "A");
            assert_eq!(to, "B");
            assert!(directed);
            assert_eq!(properties.len(), 2);

            match properties.get("weight") {
                Some(Value::Number(n)) => assert_eq!(*n, 5.0),
                _ => panic!("Expected weight property"),
            }

            match properties.get("color") {
                Some(Value::String(s)) => assert_eq!(s, "red"),
                _ => panic!("Expected color property"),
            }
        }
        _ => panic!("Expected edge with properties"),
    }
}

// ============================================================================
// Edge Field Access Tests
// ============================================================================

#[test]
fn test_edge_field_from() {
    let result = eval("let e = A -> B\ne.from").unwrap();
    match result {
        Value::String(s) => assert_eq!(s, "A"),
        _ => panic!("Expected string 'A'"),
    }
}

#[test]
fn test_edge_field_to() {
    let result = eval("let e = A -> B\ne.to").unwrap();
    match result {
        Value::String(s) => assert_eq!(s, "B"),
        _ => panic!("Expected string 'B'"),
    }
}

#[test]
fn test_edge_field_directed() {
    let result = eval("let e = A <> B\ne.directed").unwrap();
    match result {
        Value::Boolean(b) => assert!(!b),
        _ => panic!("Expected boolean false"),
    }
}

#[test]
fn test_edge_property_access() {
    let result = eval("let e = A -> B: {weight: 10}\ne.weight").unwrap();
    match result {
        Value::Number(n) => assert_eq!(n, 10.0),
        _ => panic!("Expected number 10"),
    }
}

// ============================================================================
// Network Constructor Tests
// ============================================================================

#[test]
fn test_network_simple() {
    let result = eval("network([A -> B, B -> C])").unwrap();
    match result {
        Value::Record(map) => {
            // Check nodes
            match map.get("nodes") {
                Some(Value::Record(nodes)) => {
                    assert_eq!(nodes.len(), 3); // A, B, C
                    // Verify nodes exist
                    assert!(nodes.contains_key("A"));
                    assert!(nodes.contains_key("B"));
                    assert!(nodes.contains_key("C"));

                    // Verify node structure (empty properties for now)
                    match nodes.get("A") {
                        Some(Value::Record(node_props)) => {
                            assert!(node_props.is_empty());
                        }
                        _ => panic!("Expected node record for 'A'"),
                    }
                }
                _ => panic!("Expected nodes record"),
            }

            // Check edges
            match map.get("edges") {
                Some(Value::Vector(edges)) => {
                    assert_eq!(edges.len(), 2);
                }
                _ => panic!("Expected edges vector"),
            }
        }
        _ => panic!("Expected network record"),
    }
}

#[test]
fn test_network_mixed_edges() {
    let result = eval("network([A -> B, B <> C, C -> A])").unwrap();
    match result {
        Value::Record(map) => {
            match map.get("nodes") {
                Some(Value::Record(nodes)) => {
                    assert_eq!(nodes.len(), 3);
                    assert!(nodes.contains_key("A"));
                    assert!(nodes.contains_key("B"));
                    assert!(nodes.contains_key("C"));
                }
                _ => panic!("Expected nodes record"),
            }

            match map.get("edges") {
                Some(Value::Vector(edges)) => {
                    assert_eq!(edges.len(), 3);

                    // Verify second edge is undirected
                    match &edges[1] {
                        Value::Edge { directed, .. } => assert!(!directed),
                        _ => panic!("Expected edge"),
                    }
                }
                _ => panic!("Expected edges vector"),
            }
        }
        _ => panic!("Expected network record"),
    }
}

// ============================================================================
// Network Constructor with Node Properties Tests
// ============================================================================

#[test]
fn test_network_with_node_properties() {
    let result = eval(r#"
        network([A -> B, B -> C], {
            A: {duration: 3, cost: 100},
            B: {duration: 5, cost: 200},
            C: {duration: 2, cost: 150}
        })
    "#).unwrap();

    match result {
        Value::Record(map) => {
            // Check nodes have properties
            match map.get("nodes") {
                Some(Value::Record(nodes)) => {
                    assert_eq!(nodes.len(), 3);

                    // Verify node A has properties
                    match nodes.get("A") {
                        Some(Value::Record(props)) => {
                            match props.get("duration") {
                                Some(Value::Number(d)) => assert_eq!(*d, 3.0),
                                _ => panic!("Expected duration property"),
                            }
                            match props.get("cost") {
                                Some(Value::Number(c)) => assert_eq!(*c, 100.0),
                                _ => panic!("Expected cost property"),
                            }
                        }
                        _ => panic!("Expected node A to be a record"),
                    }

                    // Verify node B has properties
                    match nodes.get("B") {
                        Some(Value::Record(props)) => {
                            match props.get("duration") {
                                Some(Value::Number(d)) => assert_eq!(*d, 5.0),
                                _ => panic!("Expected duration property"),
                            }
                        }
                        _ => panic!("Expected node B to be a record"),
                    }
                }
                _ => panic!("Expected nodes record"),
            }

            // Check edges
            match map.get("edges") {
                Some(Value::Vector(edges)) => {
                    assert_eq!(edges.len(), 2);
                }
                _ => panic!("Expected edges vector"),
            }
        }
        _ => panic!("Expected network record"),
    }
}

#[test]
fn test_network_with_isolated_nodes() {
    let result = eval(r#"
        network([A -> B], {
            A: {te: 3},
            B: {te: 5},
            Start: {te: 0}
        })
    "#).unwrap();

    match result {
        Value::Record(map) => {
            match map.get("nodes") {
                Some(Value::Record(nodes)) => {
                    // Should have 3 nodes: A, B, and isolated Start
                    assert_eq!(nodes.len(), 3);
                    assert!(nodes.contains_key("A"));
                    assert!(nodes.contains_key("B"));
                    assert!(nodes.contains_key("Start"));

                    // Verify Start has properties even though it's isolated
                    match nodes.get("Start") {
                        Some(Value::Record(props)) => {
                            match props.get("te") {
                                Some(Value::Number(te)) => assert_eq!(*te, 0.0),
                                _ => panic!("Expected te property for Start"),
                            }
                        }
                        _ => panic!("Expected Start to be a record"),
                    }
                }
                _ => panic!("Expected nodes record"),
            }

            match map.get("edges") {
                Some(Value::Vector(edges)) => {
                    // Only one edge
                    assert_eq!(edges.len(), 1);
                }
                _ => panic!("Expected edges vector"),
            }
        }
        _ => panic!("Expected network record"),
    }
}

#[test]
fn test_network_partial_properties() {
    let result = eval(r#"
        network([A -> B, B -> C], {
            A: {duration: 3},
            C: {duration: 5}
        })
    "#).unwrap();

    match result {
        Value::Record(map) => {
            match map.get("nodes") {
                Some(Value::Record(nodes)) => {
                    assert_eq!(nodes.len(), 3);

                    // A should have properties
                    match nodes.get("A") {
                        Some(Value::Record(props)) => {
                            assert_eq!(props.len(), 1);
                            match props.get("duration") {
                                Some(Value::Number(d)) => assert_eq!(*d, 3.0),
                                _ => panic!("Expected duration"),
                            }
                        }
                        _ => panic!("Expected A to be a record"),
                    }

                    // B should have empty properties
                    match nodes.get("B") {
                        Some(Value::Record(props)) => {
                            assert_eq!(props.len(), 0);
                        }
                        _ => panic!("Expected B to be a record"),
                    }

                    // C should have properties
                    match nodes.get("C") {
                        Some(Value::Record(props)) => {
                            assert_eq!(props.len(), 1);
                        }
                        _ => panic!("Expected C to be a record"),
                    }
                }
                _ => panic!("Expected nodes record"),
            }
        }
        _ => panic!("Expected network record"),
    }
}

#[test]
fn test_network_different_properties_per_node() {
    let result = eval(r#"
        network([A -> B, B -> C], {
            A: {op: 2, mo: 3, pe: 4},
            B: {duration: 5},
            C: {cost: 100, priority: 1}
        })
    "#).unwrap();

    match result {
        Value::Record(map) => {
            match map.get("nodes") {
                Some(Value::Record(nodes)) => {
                    // Each node has different properties - all valid
                    match nodes.get("A") {
                        Some(Value::Record(props)) => {
                            assert_eq!(props.len(), 3);
                            assert!(props.contains_key("op"));
                            assert!(props.contains_key("mo"));
                            assert!(props.contains_key("pe"));
                        }
                        _ => panic!("Expected A"),
                    }

                    match nodes.get("B") {
                        Some(Value::Record(props)) => {
                            assert_eq!(props.len(), 1);
                            assert!(props.contains_key("duration"));
                        }
                        _ => panic!("Expected B"),
                    }

                    match nodes.get("C") {
                        Some(Value::Record(props)) => {
                            assert_eq!(props.len(), 2);
                            assert!(props.contains_key("cost"));
                            assert!(props.contains_key("priority"));
                        }
                        _ => panic!("Expected C"),
                    }
                }
                _ => panic!("Expected nodes record"),
            }
        }
        _ => panic!("Expected network record"),
    }
}

#[test]
fn test_network_invalid_node_properties_type() {
    let result = eval(r#"
        network([A -> B], {
            A: 123
        })
    "#);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("node properties must be records"));
}

#[test]
fn test_network_backward_compatibility() {
    // Test that old syntax still works
    let result = eval("network([A -> B, B -> C])").unwrap();

    match result {
        Value::Record(map) => {
            match map.get("nodes") {
                Some(Value::Record(nodes)) => {
                    assert_eq!(nodes.len(), 3);

                    // All nodes should have empty properties
                    for (_id, props) in nodes {
                        match props {
                            Value::Record(p) => assert_eq!(p.len(), 0),
                            _ => panic!("Expected record"),
                        }
                    }
                }
                _ => panic!("Expected nodes record"),
            }
        }
        _ => panic!("Expected network record"),
    }
}
