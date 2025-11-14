mod test_common;
use test_common::eval;
use achronyme_types::value::Value;

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
