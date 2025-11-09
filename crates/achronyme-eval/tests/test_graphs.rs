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

// ============================================================================
// Network Function Tests
// ============================================================================

#[test]
fn test_nodes_function() {
    let result = eval("let g = network([A -> B, B -> C])\nnodes(g)").unwrap();
    match result {
        Value::Record(nodes) => {
            assert_eq!(nodes.len(), 3);
            assert!(nodes.contains_key("A"));
            assert!(nodes.contains_key("B"));
            assert!(nodes.contains_key("C"));
        }
        _ => panic!("Expected record of nodes"),
    }
}

#[test]
fn test_edges_function() {
    let result = eval("let g = network([A -> B, B -> C])\nedges(g)").unwrap();
    match result {
        Value::Vector(edges) => {
            assert_eq!(edges.len(), 2);
        }
        _ => panic!("Expected vector of edges"),
    }
}

#[test]
fn test_neighbors_directed() {
    let result = eval("let g = network([A -> B, A -> C, B -> D])\nneighbors(g, \"A\")").unwrap();
    match result {
        Value::Vector(neighbors) => {
            assert_eq!(neighbors.len(), 2); // B and C

            // Verify sorted order
            match &neighbors[0] {
                Value::String(s) => assert_eq!(s, "B"),
                _ => panic!("Expected string"),
            }
            match &neighbors[1] {
                Value::String(s) => assert_eq!(s, "C"),
                _ => panic!("Expected string"),
            }
        }
        _ => panic!("Expected vector of neighbors"),
    }
}

#[test]
fn test_neighbors_undirected() {
    let result = eval("let g = network([A <> B, B <> C])\nneighbors(g, \"B\")").unwrap();
    match result {
        Value::Vector(neighbors) => {
            assert_eq!(neighbors.len(), 2); // A and C (undirected, so both ways)
        }
        _ => panic!("Expected vector of neighbors"),
    }
}

#[test]
fn test_neighbors_mixed() {
    let result = eval("let g = network([A -> B, B <> C, C -> A])\nneighbors(g, \"A\")").unwrap();
    match result {
        Value::Vector(neighbors) => {
            assert_eq!(neighbors.len(), 1); // Only B (directed out). C -> A doesn't count as C being A's neighbor.
        }
        _ => panic!("Expected vector of neighbors"),
    }
}

#[test]
fn test_degree_simple() {
    let result = eval("let g = network([A -> B, B -> C])\ndegree(g, \"B\")").unwrap();
    match result {
        Value::Number(n) => assert_eq!(n, 2.0), // 1 incoming, 1 outgoing
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_degree_undirected() {
    let result = eval("let g = network([A <> B, B <> C, B <> D])\ndegree(g, \"B\")").unwrap();
    match result {
        Value::Number(n) => assert_eq!(n, 3.0), // Connected to A, C, and D
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_degree_isolated_node() {
    let result = eval("let g = network([A -> B, C -> D])\ndegree(g, \"A\")").unwrap();
    match result {
        Value::Number(n) => assert_eq!(n, 1.0), // Only one outgoing edge
        _ => panic!("Expected number"),
    }
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_weighted_graph() {
    let source = r#"
        let edges = [
            A -> B: {weight: 5},
            B -> C: {weight: 3},
            C -> A: {weight: 7}
        ]
        let g = network(edges)
        let e = edges(g)
        e
    "#;

    let result = eval(source).unwrap();
    match result {
        Value::Vector(edges) => {
            assert_eq!(edges.len(), 3);

            // Verify first edge has weight
            match &edges[0] {
                Value::Edge { properties, .. } => {
                    match properties.get("weight") {
                        Some(Value::Number(n)) => assert_eq!(*n, 5.0),
                        _ => panic!("Expected weight property"),
                    }
                }
                _ => panic!("Expected edge"),
            }
        }
        _ => panic!("Expected vector of edges"),
    }
}

#[test]
fn test_complex_network_operations() {
    let source = r#"
        let g = network([
            A -> B: {weight: 1},
            A -> C: {weight: 2},
            B -> D: {weight: 3},
            C -> D: {weight: 4},
            D <> E: {weight: 5}
        ])

        let n_neighbors = neighbors(g, "D")
        n_neighbors
    "#;

    let result = eval(source).unwrap();
    match result {
        Value::Vector(neighbors) => {
            // D has neighbors: Only E (undirected). B -> D and C -> D are incoming, not outgoing.
            assert_eq!(neighbors.len(), 1);
        }
        _ => panic!("Expected vector of neighbors"),
    }
}

// ============================================================================
// Graph Algorithm Tests
// ============================================================================

#[test]
fn test_bfs_simple() {
    let result = eval("let g = network([A -> B, A -> C, B -> D, C -> D, D -> E])\nbfs(g, \"A\")").unwrap();
    match result {
        Value::Vector(nodes) => {
            assert_eq!(nodes.len(), 5);
            // BFS order: A, B, C, D, E
            match &nodes[0] {
                Value::String(s) => assert_eq!(s, "A"),
                _ => panic!("Expected string"),
            }
            match &nodes[1] {
                Value::String(s) => assert_eq!(s, "B"),
                _ => panic!("Expected string"),
            }
        }
        _ => panic!("Expected vector"),
    }
}

#[test]
fn test_dfs_simple() {
    let result = eval("let g = network([A -> B, A -> C, B -> D, C -> D, D -> E])\ndfs(g, \"A\")").unwrap();
    match result {
        Value::Vector(nodes) => {
            assert_eq!(nodes.len(), 5);
            // DFS starts at A
            match &nodes[0] {
                Value::String(s) => assert_eq!(s, "A"),
                _ => panic!("Expected string"),
            }
        }
        _ => panic!("Expected vector"),
    }
}

#[test]
fn test_bfs_path_found() {
    let result = eval("let g = network([A -> B, A -> C, B -> D, C -> D, D -> E])\nbfs_path(g, \"A\", \"E\")").unwrap();
    match result {
        Value::Record(map) => {
            // Check found is true
            match map.get("found") {
                Some(Value::Boolean(b)) => assert!(b),
                _ => panic!("Expected found field"),
            }

            // Check path
            match map.get("path") {
                Some(Value::Vector(path)) => {
                    assert!(path.len() >= 3); // At least A -> ... -> E
                    match (&path[0], path.last()) {
                        (Value::String(start), Some(Value::String(end))) => {
                            assert_eq!(start, "A");
                            assert_eq!(end, "E");
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
fn test_bfs_path_not_found() {
    let result = eval("let g = network([A -> B, C -> D])\nbfs_path(g, \"A\", \"D\")").unwrap();
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
fn test_bfs_undirected_graph() {
    let result = eval("let g = network([A <> B, B <> C, C <> D])\nbfs(g, \"A\")").unwrap();
    match result {
        Value::Vector(nodes) => {
            assert_eq!(nodes.len(), 4);
            // Should visit all nodes in undirected graph
        }
        _ => panic!("Expected vector"),
    }
}

#[test]
fn test_dfs_undirected_graph() {
    let result = eval("let g = network([A <> B, B <> C, C <> D])\ndfs(g, \"C\")").unwrap();
    match result {
        Value::Vector(nodes) => {
            assert_eq!(nodes.len(), 4);
            // Should visit all nodes starting from C
            match &nodes[0] {
                Value::String(s) => assert_eq!(s, "C"),
                _ => panic!("Expected string"),
            }
        }
        _ => panic!("Expected vector"),
    }
}

#[test]
fn test_bfs_path_undirected() {
    let result = eval("let g = network([A <> B, B <> C, C <> D])\nbfs_path(g, \"A\", \"D\")").unwrap();
    match result {
        Value::Record(map) => {
            match map.get("found") {
                Some(Value::Boolean(b)) => assert!(b),
                _ => panic!("Expected found field"),
            }

            match map.get("path") {
                Some(Value::Vector(path)) => {
                    assert_eq!(path.len(), 4); // A, B, C, D
                }
                _ => panic!("Expected path field"),
            }
        }
        _ => panic!("Expected record"),
    }
}

#[test]
fn test_bfs_disconnected_components() {
    // BFS should only visit reachable nodes
    let result = eval("let g = network([A -> B, C -> D])\nbfs(g, \"A\")").unwrap();
    match result {
        Value::Vector(nodes) => {
            assert_eq!(nodes.len(), 2); // Only A and B
        }
        _ => panic!("Expected vector"),
    }
}

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
// Kruskal Algorithm Tests
// ============================================================================

#[test]
fn test_kruskal_simple_mst() {
    let result = eval(r#"
        let g = network([
            A <> B: {weight: 1},
            A <> C: {weight: 4},
            B <> C: {weight: 2},
            B <> D: {weight: 5},
            C <> D: {weight: 3}
        ])
        kruskal(g)
    "#).unwrap();

    match result {
        Value::Record(map) => {
            // Check total weight
            match map.get("total_weight") {
                Some(Value::Number(w)) => assert_eq!(*w, 6.0), // 1 + 2 + 3 (MST of 4 nodes)
                _ => panic!("Expected total_weight field"),
            }

            // Check edges count (MST of 4 nodes has 3 edges)
            match map.get("edges") {
                Some(Value::Vector(edges)) => assert_eq!(edges.len(), 3),
                _ => panic!("Expected edges field"),
            }
        }
        _ => panic!("Expected record"),
    }
}

#[test]
fn test_kruskal_requires_undirected() {
    let result = eval(r#"
        let g = network([
            A -> B: {weight: 1},
            B <> C: {weight: 2}
        ])
        kruskal(g)
    "#);

    match result {
        Err(msg) => {
            assert!(msg.contains("kruskal() requires an undirected graph"));
        }
        Ok(_) => panic!("Expected error for directed edge"),
    }
}

#[test]
fn test_kruskal_requires_weights() {
    let result = eval(r#"
        let g = network([
            A <> B: {weight: 1},
            B <> C
        ])
        kruskal(g)
    "#);

    match result {
        Err(msg) => {
            assert!(msg.contains("kruskal() requires all edges to have a 'weight' property"));
        }
        Ok(_) => panic!("Expected error for missing weight"),
    }
}

#[test]
fn test_kruskal_classic_example() {
    // Classic MST example from textbooks
    let result = eval(r#"
        let g = network([
            A <> B: {weight: 2},
            A <> C: {weight: 3},
            B <> C: {weight: 1},
            B <> D: {weight: 1},
            C <> D: {weight: 4}
        ])
        kruskal(g)
    "#).unwrap();

    match result {
        Value::Record(map) => {
            match map.get("total_weight") {
                Some(Value::Number(w)) => assert_eq!(*w, 4.0), // 1 + 1 + 2 (edges: B-C, B-D, A-B)
                _ => panic!("Expected total_weight field"),
            }
        }
        _ => panic!("Expected record"),
    }
}

// ============================================================================
// Prim Algorithm Tests
// ============================================================================

#[test]
fn test_prim_simple_mst() {
    let result = eval(r#"
        let g = network([
            A <> B: {weight: 1},
            A <> C: {weight: 4},
            B <> C: {weight: 2},
            B <> D: {weight: 5},
            C <> D: {weight: 3}
        ])
        prim(g, "A")
    "#).unwrap();

    match result {
        Value::Record(map) => {
            // Check total weight (same as Kruskal for connected graph)
            match map.get("total_weight") {
                Some(Value::Number(w)) => assert_eq!(*w, 6.0),
                _ => panic!("Expected total_weight field"),
            }

            // Check edges count
            match map.get("edges") {
                Some(Value::Vector(edges)) => assert_eq!(edges.len(), 3),
                _ => panic!("Expected edges field"),
            }
        }
        _ => panic!("Expected record"),
    }
}

#[test]
fn test_prim_different_start_node() {
    // MST should be same regardless of start node
    let result1 = eval(r#"
        let g = network([
            A <> B: {weight: 1},
            B <> C: {weight: 2},
            C <> D: {weight: 3}
        ])
        prim(g, "A")
    "#).unwrap();

    let result2 = eval(r#"
        let g = network([
            A <> B: {weight: 1},
            B <> C: {weight: 2},
            C <> D: {weight: 3}
        ])
        prim(g, "D")
    "#).unwrap();

    // Both should have same total weight
    match (result1, result2) {
        (Value::Record(map1), Value::Record(map2)) => {
            let w1 = match map1.get("total_weight") {
                Some(Value::Number(w)) => *w,
                _ => panic!("Expected weight"),
            };
            let w2 = match map2.get("total_weight") {
                Some(Value::Number(w)) => *w,
                _ => panic!("Expected weight"),
            };
            assert_eq!(w1, w2);
        }
        _ => panic!("Expected records"),
    }
}

#[test]
fn test_prim_requires_undirected() {
    let result = eval(r#"
        let g = network([
            A -> B: {weight: 1},
            B <> C: {weight: 2}
        ])
        prim(g, "A")
    "#);

    match result {
        Err(msg) => {
            assert!(msg.contains("prim() requires an undirected graph"));
        }
        Ok(_) => panic!("Expected error for directed edge"),
    }
}

#[test]
fn test_prim_node_not_found() {
    let result = eval(r#"
        let g = network([
            A <> B: {weight: 1}
        ])
        prim(g, "Z")
    "#);

    match result {
        Err(msg) => {
            assert!(msg.contains("Node 'Z' not found"));
        }
        Ok(_) => panic!("Expected error for missing node"),
    }
}

#[test]
fn test_kruskal_prim_same_result() {
    // Both algorithms should produce same total weight
    let kruskal_result = eval(r#"
        let g = network([
            A <> B: {weight: 2},
            A <> C: {weight: 3},
            B <> C: {weight: 1},
            B <> D: {weight: 1},
            C <> D: {weight: 4}
        ])
        kruskal(g)
    "#).unwrap();

    let prim_result = eval(r#"
        let g = network([
            A <> B: {weight: 2},
            A <> C: {weight: 3},
            B <> C: {weight: 1},
            B <> D: {weight: 1},
            C <> D: {weight: 4}
        ])
        prim(g, "A")
    "#).unwrap();

    match (kruskal_result, prim_result) {
        (Value::Record(k_map), Value::Record(p_map)) => {
            let k_weight = match k_map.get("total_weight") {
                Some(Value::Number(w)) => *w,
                _ => panic!("Expected weight"),
            };
            let p_weight = match p_map.get("total_weight") {
                Some(Value::Number(w)) => *w,
                _ => panic!("Expected weight"),
            };
            assert_eq!(k_weight, p_weight);
        }
        _ => panic!("Expected records"),
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
