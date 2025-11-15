//! Edge Type Annotation Tests

use achronyme_eval::Evaluator;

#[test]
fn test_edge_type_annotation_valid() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"
        let edge: Edge = A -> B;
        edge
    "#);
    assert!(result.is_ok());
}

#[test]
fn test_edge_type_annotation_mismatch() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"
        let edge: Edge = 42
    "#);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Type error"));
    assert!(err.contains("Edge"));
    assert!(err.contains("Number"));
}

#[test]
fn test_edge_type_undirected() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"
        let edge: Edge = A <> B;
        edge
    "#);
    assert!(result.is_ok());
}

#[test]
fn test_edge_union_type() {
    let mut eval = Evaluator::new();
    // Edge can be part of union types
    let result = eval.eval_str(r#"
        let maybeEdge: Edge | null = null
    "#);
    assert!(result.is_ok());

    let result2 = eval.eval_str(r#"
        let maybeEdge2: Edge | null = A -> B
    "#);
    assert!(result2.is_ok());
}
