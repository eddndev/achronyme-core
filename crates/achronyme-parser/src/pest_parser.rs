// ============================================================================
// Pest-based Parser for Achronyme SOC Language
// ============================================================================
// This module uses Pest (PEG parser generator) to parse SOC expressions
// and generate the AST (Abstract Syntax Tree).
//
// The grammar is defined in grammar.pest.
// ============================================================================

use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

use crate::ast::{AstNode, BinaryOp, UnaryOp};

// ============================================================================
// Parser Definition
// ============================================================================

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct SOCParser;

// ============================================================================
// Public API
// ============================================================================

/// Parse a SOC expression string into an AST
pub fn parse(input: &str) -> Result<Vec<AstNode>, String> {
    let pairs = SOCParser::parse(Rule::program, input)
        .map_err(|e| format!("Parse error: {}", e))?;

    let mut statements = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::program => {
                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::statement => {
                            statements.push(build_ast_from_statement(inner_pair)?);
                        }
                        Rule::EOI => {} // End of input, ignore
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Ok(statements)
}

// ============================================================================
// AST Building Functions
// ============================================================================

fn build_ast_from_statement(pair: Pair<Rule>) -> Result<AstNode, String> {
    let inner = pair.into_inner().next()
        .ok_or("Empty statement")?;

    match inner.as_rule() {
        Rule::let_statement => build_let_statement(inner),
        Rule::expr => build_ast_from_expr(inner),
        _ => Err(format!("Unexpected statement rule: {:?}", inner.as_rule()))
    }
}

fn build_let_statement(pair: Pair<Rule>) -> Result<AstNode, String> {
    let mut inner = pair.into_inner();

    let identifier = inner.next()
        .ok_or("Missing identifier in let statement")?
        .as_str()
        .to_string();

    let initializer = inner.next()
        .ok_or("Missing initializer in let statement")?;

    Ok(AstNode::VariableDecl {
        name: identifier,
        initializer: Box::new(build_ast_from_expr(initializer)?),
    })
}

fn build_ast_from_expr(pair: Pair<Rule>) -> Result<AstNode, String> {
    let rule = pair.as_rule();
    match rule {
        Rule::expr => {
            let inner = pair.into_inner().next()
                .ok_or("Empty expression")?;
            build_ast_from_expr(inner)
        }
        Rule::comparison => build_comparison(pair),
        Rule::additive => build_binary_op(pair),
        Rule::multiplicative => build_binary_op(pair),
        Rule::unary => build_unary(pair),
        Rule::power => build_power(pair),
        Rule::primary => build_primary(pair),
        _ => Err(format!("Unexpected expression rule: {:?}", rule))
    }
}

fn build_comparison(pair: Pair<Rule>) -> Result<AstNode, String> {
    let pairs: Vec<_> = pair.into_inner().collect();

    if pairs.len() == 1 {
        // No comparison, just the additive expression
        return build_ast_from_expr(pairs[0].clone());
    }

    // Should have exactly 3 pairs: left, operator, right
    if pairs.len() != 3 {
        return Err(format!("Expected 3 pairs for comparison, got {}", pairs.len()));
    }

    let left = build_ast_from_expr(pairs[0].clone())?;

    // The operator should be a cmp_op rule
    let op = match pairs[1].as_rule() {
        Rule::cmp_op => {
            match pairs[1].as_str() {
                ">" => BinaryOp::Gt,
                "<" => BinaryOp::Lt,
                ">=" => BinaryOp::Gte,
                "<=" => BinaryOp::Lte,
                "==" => BinaryOp::Eq,
                "!=" => BinaryOp::Neq,
                _ => return Err(format!("Unknown comparison operator: {}", pairs[1].as_str()))
            }
        }
        _ => return Err(format!("Expected cmp_op, got: {:?}", pairs[1].as_rule()))
    };

    let right = build_ast_from_expr(pairs[2].clone())?;

    Ok(AstNode::BinaryOp {
        op,
        left: Box::new(left),
        right: Box::new(right),
    })
}

fn build_binary_op(pair: Pair<Rule>) -> Result<AstNode, String> {
    let pairs: Vec<_> = pair.into_inner().collect();

    if pairs.len() == 1 {
        // Single operand, no operation
        return build_ast_from_expr(pairs[0].clone());
    }

    // Pest gives us: [operand, operator, operand, operator, operand, ...]
    // We need to separate them into operands and operators

    let mut operands = Vec::new();
    let mut operators = Vec::new();

    for p in pairs.iter() {
        // Check if this is an operator rule
        match p.as_rule() {
            Rule::add_op | Rule::mult_op => {
                let s = p.as_str();
                let op = match s {
                    "+" => BinaryOp::Add,
                    "-" => BinaryOp::Subtract,
                    "*" => BinaryOp::Multiply,
                    "/" => BinaryOp::Divide,
                    "%" => BinaryOp::Modulo,
                    _ => unreachable!()
                };
                operators.push(op);
            }
            _ => {
                // It's an operand (a sub-rule)
                operands.push(p.clone());
            }
        }
    }

    if operands.is_empty() {
        return Err("No operands found".to_string());
    }

    // Build left-associative tree
    let mut left = build_ast_from_expr(operands[0].clone())?;

    for (i, op) in operators.iter().enumerate() {
        let right = build_ast_from_expr(operands[i + 1].clone())?;
        left = AstNode::BinaryOp {
            op: op.clone(),
            left: Box::new(left),
            right: Box::new(right),
        };
    }

    Ok(left)
}

fn build_unary(pair: Pair<Rule>) -> Result<AstNode, String> {
    let mut inner = pair.into_inner();
    let first = inner.next().ok_or("Empty unary expression")?;

    match first.as_rule() {
        Rule::unary => {
            // This is a negation: -<expr>
            Ok(AstNode::UnaryOp {
                op: UnaryOp::Negate,
                operand: Box::new(build_unary(first)?),
            })
        }
        Rule::power => build_power(first),
        _ => Err(format!("Unexpected unary rule: {:?}", first.as_rule()))
    }
}

fn build_power(pair: Pair<Rule>) -> Result<AstNode, String> {
    let mut inner = pair.into_inner();
    let base = build_ast_from_expr(inner.next().ok_or("Missing base in power")?)?;

    if let Some(exponent_pair) = inner.next() {
        // Right-associative: 2^3^4 = 2^(3^4)
        let exponent = build_power(exponent_pair)?;
        Ok(AstNode::BinaryOp {
            op: BinaryOp::Power,
            left: Box::new(base),
            right: Box::new(exponent),
        })
    } else {
        Ok(base)
    }
}

fn build_primary(pair: Pair<Rule>) -> Result<AstNode, String> {
    let inner = pair.into_inner().next()
        .ok_or("Empty primary expression")?;

    match inner.as_rule() {
        Rule::number => {
            let num = inner.as_str().parse::<f64>()
                .map_err(|e| format!("Failed to parse number: {}", e))?;
            Ok(AstNode::Number(num))
        }
        Rule::complex => {
            // Complex number: "3i" or "-2i"
            let s = inner.as_str();
            let num_part = &s[..s.len()-1]; // Remove 'i'
            let im = num_part.parse::<f64>()
                .map_err(|e| format!("Failed to parse complex number: {}", e))?;
            Ok(AstNode::ComplexLiteral { re: 0.0, im })
        }
        Rule::identifier => {
            Ok(AstNode::VariableRef(inner.as_str().to_string()))
        }
        Rule::vector => build_vector(inner),
        Rule::matrix => build_matrix(inner),
        Rule::lambda => build_lambda(inner),
        Rule::function_call => build_function_call(inner),
        Rule::expr => build_ast_from_expr(inner),
        _ => Err(format!("Unexpected primary rule: {:?}", inner.as_rule()))
    }
}

fn build_vector(pair: Pair<Rule>) -> Result<AstNode, String> {
    let elements: Result<Vec<AstNode>, String> = pair
        .into_inner()
        .map(|p| build_ast_from_expr(p))
        .collect();

    Ok(AstNode::VectorLiteral(elements?))
}

fn build_matrix(pair: Pair<Rule>) -> Result<AstNode, String> {
    let rows: Result<Vec<Vec<AstNode>>, String> = pair
        .into_inner()
        .map(|row_pair| {
            row_pair.into_inner()
                .map(|elem| build_ast_from_expr(elem))
                .collect()
        })
        .collect();

    Ok(AstNode::MatrixLiteral(rows?))
}

fn build_lambda(pair: Pair<Rule>) -> Result<AstNode, String> {
    let mut inner = pair.into_inner();

    let params_pair = inner.next().ok_or("Missing lambda parameters")?;
    let params = extract_lambda_params(params_pair)?;

    let body_pair = inner.next().ok_or("Missing lambda body")?;
    let body = build_ast_from_expr(body_pair)?;

    Ok(AstNode::Lambda {
        params,
        body: Box::new(body),
    })
}

fn extract_lambda_params(pair: Pair<Rule>) -> Result<Vec<String>, String> {
    let inner = pair.into_inner();
    let params: Vec<String> = inner
        .map(|p| p.as_str().to_string())
        .collect();

    if params.is_empty() {
        Err("Lambda must have at least one parameter".to_string())
    } else {
        Ok(params)
    }
}

fn build_function_call(pair: Pair<Rule>) -> Result<AstNode, String> {
    let mut inner = pair.into_inner();

    let name = inner.next()
        .ok_or("Missing function name")?
        .as_str()
        .to_string();

    let args: Result<Vec<AstNode>, String> = inner
        .map(|p| build_ast_from_expr(p))
        .collect();

    Ok(AstNode::FunctionCall {
        name,
        args: args?,
    })
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        let result = parse("42").unwrap();
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], AstNode::Number(42.0)));
    }

    #[test]
    fn test_parse_arithmetic() {
        let result = parse("2 + 3 * 4").unwrap();
        assert_eq!(result.len(), 1);
        // Should parse as: 2 + (3 * 4)

        // Verify the structure
        match &result[0] {
            AstNode::BinaryOp { op, left, right } => {
                assert!(matches!(op, BinaryOp::Add));
                assert!(matches!(**left, AstNode::Number(2.0)));
                // Right should be 3 * 4
                match &**right {
                    AstNode::BinaryOp { op: mult_op, .. } => {
                        assert!(matches!(mult_op, BinaryOp::Multiply));
                    }
                    _ => panic!("Expected multiplication on right side"),
                }
            }
            _ => panic!("Expected binary op at top level"),
        }
    }

    #[test]
    fn test_parse_power() {
        let result = parse("2^3^4").unwrap();
        assert_eq!(result.len(), 1);
        // Should parse as: 2^(3^4) - right associative
    }

    #[test]
    fn test_parse_vector() {
        let result = parse("[1, 2, 3]").unwrap();
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], AstNode::VectorLiteral(_)));
    }

    #[test]
    fn test_parse_lambda() {
        let result = parse("x => x^2").unwrap();
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], AstNode::Lambda { .. }));
    }

    #[test]
    fn test_parse_function_call() {
        let result = parse("sin(PI)").unwrap();
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], AstNode::FunctionCall { .. }));
    }

    #[test]
    fn test_parse_let_statement() {
        let result = parse("let x = 42").unwrap();
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], AstNode::VariableDecl { .. }));
    }

    #[test]
    fn test_parse_complex() {
        let result = parse("3i").unwrap();
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], AstNode::ComplexLiteral { re: 0.0, im: 3.0 }));
    }
}
