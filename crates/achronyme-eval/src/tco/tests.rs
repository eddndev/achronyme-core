use super::*;
use achronyme_parser::ast::{ArrayElement, BinaryOp};

#[test]
fn test_direct_rec_reference_is_tail() {
    let node = AstNode::RecReference;
    assert!(is_tail_position(&node));
    assert!(is_tail_recursive_function(&node));
}

#[test]
fn test_rec_call_expression_is_tail() {
    // rec(n-1)
    let node = AstNode::CallExpression {
        callee: Box::new(AstNode::RecReference),
        args: vec![AstNode::Number(1.0)],
    };
    assert!(is_tail_position(&node));
    assert!(is_tail_recursive_function(&node));
}

#[test]
fn test_binary_op_with_rec_is_not_tail() {
    // n * rec(n-1)
    let node = AstNode::BinaryOp {
        op: BinaryOp::Multiply,
        left: Box::new(AstNode::Number(5.0)),
        right: Box::new(AstNode::CallExpression {
            callee: Box::new(AstNode::RecReference),
            args: vec![AstNode::Number(1.0)],
        }),
    };
    assert!(!is_tail_position(&node));
    assert!(!is_tail_recursive_function(&node));
}

#[test]
fn test_if_with_tail_calls_is_tail() {
    // if(n <= 1, 1, rec(n-1, acc*n))
    let node = AstNode::If {
        condition: Box::new(AstNode::Boolean(true)),
        then_expr: Box::new(AstNode::Number(1.0)),
        else_expr: Box::new(AstNode::CallExpression {
            callee: Box::new(AstNode::RecReference),
            args: vec![AstNode::Number(1.0)],
        }),
    };
    assert!(is_tail_position(&node));
    assert!(is_tail_recursive_function(&node));
}

#[test]
fn test_if_with_non_tail_call_is_not_tail() {
    // if(n <= 1, 1, n * rec(n-1))
    let node = AstNode::If {
        condition: Box::new(AstNode::Boolean(true)),
        then_expr: Box::new(AstNode::Number(1.0)),
        else_expr: Box::new(AstNode::BinaryOp {
            op: BinaryOp::Multiply,
            left: Box::new(AstNode::Number(5.0)),
            right: Box::new(AstNode::CallExpression {
                callee: Box::new(AstNode::RecReference),
                args: vec![],
            }),
        }),
    };
    assert!(!is_tail_position(&node));
    assert!(!is_tail_recursive_function(&node));
}

#[test]
fn test_do_block_with_tail_call() {
    // do { let x = 5; rec(x) }
    let node = AstNode::DoBlock {
        statements: vec![
            AstNode::VariableDecl {
                name: "x".to_string(),
                type_annotation: None,
                initializer: Box::new(AstNode::Number(5.0)),
            },
            AstNode::CallExpression {
                callee: Box::new(AstNode::RecReference),
                args: vec![AstNode::VariableRef("x".to_string())],
            },
        ],
    };
    assert!(is_tail_position(&node));
    assert!(is_tail_recursive_function(&node));
}

#[test]
fn test_do_block_with_non_tail_call() {
    // do { let x = 5; rec(x) + 1 }
    let node = AstNode::DoBlock {
        statements: vec![
            AstNode::VariableDecl {
                name: "x".to_string(),
                type_annotation: None,
                initializer: Box::new(AstNode::Number(5.0)),
            },
            AstNode::BinaryOp {
                op: BinaryOp::Add,
                left: Box::new(AstNode::CallExpression {
                    callee: Box::new(AstNode::RecReference),
                    args: vec![AstNode::VariableRef("x".to_string())],
                }),
                right: Box::new(AstNode::Number(1.0)),
            },
        ],
    };
    assert!(!is_tail_position(&node));
    assert!(!is_tail_recursive_function(&node));
}

#[test]
fn test_literal_is_tail_but_not_recursive() {
    let node = AstNode::Number(42.0);
    assert!(is_tail_position(&node));
    assert!(!is_tail_recursive_function(&node)); // No rec
}

#[test]
fn test_no_rec_is_not_tail_recursive() {
    // if(x > 0, 1, -1) - no rec at all
    let node = AstNode::If {
        condition: Box::new(AstNode::Boolean(true)),
        then_expr: Box::new(AstNode::Number(1.0)),
        else_expr: Box::new(AstNode::Number(-1.0)),
    };
    assert!(!is_tail_recursive_function(&node)); // No rec
}

#[test]
fn test_rec_in_array_is_not_tail() {
    // [rec(n-1), n]
    let node = AstNode::ArrayLiteral(vec![
        ArrayElement::Single(AstNode::CallExpression {
            callee: Box::new(AstNode::RecReference),
            args: vec![],
        }),
        ArrayElement::Single(AstNode::Number(5.0)),
    ]);
    assert!(!is_tail_position(&node));
    assert!(!is_tail_recursive_function(&node));
}

#[test]
fn test_contains_rec() {
    let with_rec = AstNode::CallExpression {
        callee: Box::new(AstNode::RecReference),
        args: vec![],
    };
    assert!(contains_rec(&with_rec));

    let without_rec = AstNode::Number(5.0);
    assert!(!contains_rec(&without_rec));
}

#[test]
fn test_piecewise_with_tail_calls() {
    // piecewise([x < 0, -1], [x > 0, 1], 0) - no rec
    let node = AstNode::Piecewise {
        cases: vec![
            (Box::new(AstNode::Boolean(true)), Box::new(AstNode::Number(-1.0))),
            (Box::new(AstNode::Boolean(false)), Box::new(AstNode::Number(1.0))),
        ],
        default: Some(Box::new(AstNode::Number(0.0))),
    };
    assert!(!is_tail_recursive_function(&node)); // No rec

    // piecewise([x <= 1, x], rec(x-1))
    let with_rec = AstNode::Piecewise {
        cases: vec![
            (
                Box::new(AstNode::Boolean(true)),
                Box::new(AstNode::VariableRef("x".to_string())),
            )
        ],
        default: Some(Box::new(AstNode::CallExpression {
            callee: Box::new(AstNode::RecReference),
            args: vec![AstNode::Number(1.0)],
        })),
    };
    assert!(is_tail_recursive_function(&with_rec));
}
