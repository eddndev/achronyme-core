mod test_common;
use test_common::eval;
use achronyme_types::value::Value;

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
