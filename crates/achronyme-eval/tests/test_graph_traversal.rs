mod test_common;
use test_common::eval;
use achronyme_types::value::Value;

// ============================================================================
// Graph Traversal Algorithm Tests (BFS/DFS)
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
