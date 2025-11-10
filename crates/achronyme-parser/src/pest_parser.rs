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

use crate::ast::{AstNode, BinaryOp, UnaryOp, IndexArg};

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

    // Grammar: "let" ~ identifier ~ "=" ~ expr
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
        Rule::logical_or => build_binary_op(pair),
        Rule::logical_and => build_binary_op(pair),
        Rule::comparison => build_comparison(pair),
        Rule::edge => build_edge(pair),
        Rule::additive => build_binary_op(pair),
        Rule::multiplicative => build_binary_op(pair),
        Rule::unary => build_unary(pair),
        Rule::power => build_power(pair),
        Rule::primary => build_primary(pair),
        Rule::field_access => build_field_access(pair),
        Rule::access => build_access(pair),
        _ => Err(format!("Unexpected expression rule: {:?}", rule))
    }
}

fn build_edge(pair: Pair<Rule>) -> Result<AstNode, String> {
    let pairs: Vec<_> = pair.into_inner().collect();

    if pairs.len() == 1 {
        // No edge, just the additive expression
        return build_ast_from_expr(pairs[0].clone());
    }

    // Edge syntax: additive edge_op additive [":" additive]
    // pairs can be: [left, edge_op, right] or [left, edge_op, right, metadata]

    if pairs.len() < 3 {
        return Err(format!("Edge requires at least 3 pairs (from, op, to), got {}", pairs.len()));
    }

    // Extract 'from' identifier - must be a single identifier
    let from = extract_identifier(&pairs[0])?;

    // Extract edge operator
    let directed = match pairs[1].as_rule() {
        Rule::edge_op => {
            match pairs[1].as_str() {
                "->" => true,
                "<>" => false,
                _ => return Err(format!("Unknown edge operator: {}", pairs[1].as_str()))
            }
        }
        _ => return Err(format!("Expected edge_op, got: {:?}", pairs[1].as_rule()))
    };

    // Extract 'to' identifier - must be a single identifier
    let to = extract_identifier(&pairs[2])?;

    // Extract optional metadata
    let metadata = if pairs.len() >= 4 {
        Some(Box::new(build_ast_from_expr(pairs[3].clone())?))
    } else {
        None
    };

    Ok(AstNode::Edge {
        from,
        to,
        directed,
        metadata,
    })
}

/// Extract an identifier from a Pair, ensuring it's a single identifier
/// This enforces that edge nodes are pure identifiers, not expressions
fn extract_identifier(pair: &Pair<Rule>) -> Result<String, String> {
    // Navigate through the parsing tree to find the identifier
    // The structure is: additive -> multiplicative -> unary -> power -> field_access -> primary -> identifier

    fn find_identifier(p: &Pair<Rule>) -> Result<String, String> {
        match p.as_rule() {
            Rule::identifier => Ok(p.as_str().to_string()),
            Rule::additive | Rule::multiplicative | Rule::unary |
            Rule::power | Rule::field_access | Rule::primary => {
                let inner: Vec<_> = p.clone().into_inner().collect();
                if inner.len() != 1 {
                    return Err("Edge nodes must be simple identifiers, not expressions".to_string());
                }
                find_identifier(&inner[0])
            }
            _ => Err(format!("Edge nodes must be identifiers, got: {:?}", p.as_rule()))
        }
    }

    find_identifier(pair)
}

fn build_comparison(pair: Pair<Rule>) -> Result<AstNode, String> {
    let pairs: Vec<_> = pair.into_inner().collect();

    if pairs.len() == 1 {
        // No comparison, just the edge expression
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
            Rule::add_op | Rule::mult_op | Rule::logical_and_op | Rule::logical_or_op => {
                let s = p.as_str();
                let op = match s {
                    "+" => BinaryOp::Add,
                    "-" => BinaryOp::Subtract,
                    "*" => BinaryOp::Multiply,
                    "/" => BinaryOp::Divide,
                    "%" => BinaryOp::Modulo,
                    "&&" => BinaryOp::And,
                    "||" => BinaryOp::Or,
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
    let pair_str = pair.as_str();
    let mut inner = pair.into_inner();
    let first = inner.next().ok_or("Empty unary expression")?;

    match first.as_rule() {
        Rule::unary => {
            // This is either - or ! operator
            // Check what operator we have by looking at the string
            let op = if pair_str.trim_start().starts_with('-') {
                UnaryOp::Negate
            } else if pair_str.trim_start().starts_with('!') {
                UnaryOp::Not
            } else {
                return Err(format!("Unknown unary operator in: {}", pair_str));
            };

            Ok(AstNode::UnaryOp {
                op,
                operand: Box::new(build_unary(first)?),
            })
        }
        Rule::power => build_power(first),
        _ => Err(format!("Unexpected unary rule: {:?}", first.as_rule()))
    }
}

fn build_power(pair: Pair<Rule>) -> Result<AstNode, String> {
    let mut inner = pair.into_inner();
    let base = build_field_access(inner.next().ok_or("Missing base in power")?)?;

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

fn build_access(pair: Pair<Rule>) -> Result<AstNode, String> {
    let mut inner = pair.into_inner();
    let mut base = build_primary(inner.next().ok_or("Missing primary in access")?)?;

    // Process each index/slice operation (e.g., [0], [0, 1, 2], [0, .., ..])
    // Since Pest generates consecutive access_arg for comma-separated indices,
    // we need to accumulate them into a single IndexAccess node
    let mut indices = Vec::new();

    for index_pair in inner {
        match index_pair.as_rule() {
            Rule::access_arg => {
                // Accumulate all consecutive access_args
                indices.push(build_access_arg(index_pair)?);
            }
            _ => {
                // If we encounter a non-access_arg rule, apply any accumulated indices first
                if !indices.is_empty() {
                    base = AstNode::IndexAccess {
                        object: Box::new(base),
                        indices: indices.clone(),
                    };
                    indices.clear();
                }

                // This should be a group of access_args inside brackets
                // e.g., for tensor[0, 1, 2], we get multiple access_arg children
                let mut new_indices = Vec::new();

                // The index_pair might be a list of access_arg
                for arg in index_pair.into_inner() {
                    if arg.as_rule() == Rule::access_arg {
                        new_indices.push(build_access_arg(arg)?);
                    }
                }

                if !new_indices.is_empty() {
                    base = AstNode::IndexAccess {
                        object: Box::new(base),
                        indices: new_indices,
                    };
                }
            }
        }
    }

    // Apply any remaining accumulated indices
    if !indices.is_empty() {
        base = AstNode::IndexAccess {
            object: Box::new(base),
            indices,
        };
    }

    Ok(base)
}

fn build_access_arg(pair: Pair<Rule>) -> Result<IndexArg, String> {
    let inner = pair.into_inner().next()
        .ok_or("Empty access_arg")?;

    match inner.as_rule() {
        Rule::range_expr => {
            // Range: start..end, start.., ..end, or ..
            // The grammar now has explicit alternatives, so we parse accordingly
            let inner_str = inner.as_str();
            let range_parts = inner.into_inner();

            // Count how many expressions we have
            let exprs: Vec<_> = range_parts.collect();

            let (start, end) = if inner_str.starts_with("..") && !inner_str.ends_with("..") {
                // "..end" case
                if exprs.len() == 1 {
                    (None, Some(Box::new(build_ast_from_expr(exprs[0].clone())?)))
                } else {
                    return Err("Invalid range expression".to_string());
                }
            } else if inner_str.ends_with("..") && !inner_str.starts_with("..") {
                // "start.." case
                if exprs.len() == 1 {
                    (Some(Box::new(build_ast_from_expr(exprs[0].clone())?)), None)
                } else {
                    return Err("Invalid range expression".to_string());
                }
            } else if inner_str == ".." {
                // ".." case
                (None, None)
            } else {
                // "start..end" case
                if exprs.len() == 2 {
                    (
                        Some(Box::new(build_ast_from_expr(exprs[0].clone())?)),
                        Some(Box::new(build_ast_from_expr(exprs[1].clone())?))
                    )
                } else {
                    return Err("Invalid range expression".to_string());
                }
            };

            Ok(IndexArg::Range { start, end })
        }
        Rule::expr => {
            // Single index expression
            Ok(IndexArg::Single(Box::new(build_ast_from_expr(inner)?)))
        }
        _ => Err(format!("Unexpected access_arg rule: {:?}", inner.as_rule()))
    }
}

fn build_field_access(pair: Pair<Rule>) -> Result<AstNode, String> {
    let mut inner = pair.into_inner();
    // Updated to use build_access instead of build_primary
    let mut base = build_access(inner.next().ok_or("Missing access in field_access")?)?;

    // Process each field access (e.g., .field1.field2.field3)
    for field_pair in inner {
        let field_name = field_pair.as_str().to_string();
        base = AstNode::FieldAccess {
            record: Box::new(base),
            field: field_name,
        };
    }

    Ok(base)
}

/// Process escape sequences in string literals
fn process_escape_sequences(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(next_ch) = chars.next() {
                match next_ch {
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    'r' => result.push('\r'),
                    '\\' => result.push('\\'),
                    '"' => result.push('"'),
                    _ => {
                        // Unknown escape sequence, keep as is
                        result.push('\\');
                        result.push(next_ch);
                    }
                }
            } else {
                result.push('\\');
            }
        } else {
            result.push(ch);
        }
    }

    result
}

fn build_primary(pair: Pair<Rule>) -> Result<AstNode, String> {
    let inner = pair.into_inner().next()
        .ok_or("Empty primary expression")?;

    match inner.as_rule() {
        Rule::boolean => {
            let value = inner.as_str() == "true";
            Ok(AstNode::Boolean(value))
        }
        Rule::string_literal => {
            // Parse string literal: "hello" -> hello
            let s = inner.as_str();
            // Remove surrounding quotes
            let content = &s[1..s.len()-1];
            // Process escape sequences
            let processed = process_escape_sequences(content);
            Ok(AstNode::StringLiteral(processed))
        }
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
        Rule::self_ref => {
            Ok(AstNode::SelfReference)
        }
        Rule::rec_ref => {
            Ok(AstNode::RecReference)
        }
        Rule::array => build_array(inner),
        Rule::vector => build_array(inner),  // Alias for array
        Rule::matrix => build_array(inner),  // Alias for array
        Rule::record => build_record(inner),
        Rule::lambda => build_lambda(inner),
        Rule::function_call => build_function_call(inner),
        Rule::expr => build_ast_from_expr(inner),
        _ => Err(format!("Unexpected primary rule: {:?}", inner.as_rule()))
    }
}

/// Build array from pest pair - handles vectors, matrices, and N-dimensional tensors
/// Now returns unified ArrayLiteral for all dimensions
fn build_array(pair: Pair<Rule>) -> Result<AstNode, String> {
    let elements: Vec<AstNode> = pair
        .into_inner()
        .map(|p| build_ast_from_expr(p))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(AstNode::ArrayLiteral(elements))
}

fn build_record(pair: Pair<Rule>) -> Result<AstNode, String> {
    let fields: Result<Vec<(String, AstNode)>, String> = pair
        .into_inner()
        .map(|field_pair| {
            // Each field_pair is a record_field: identifier ~ ":" ~ expr
            let mut field_inner = field_pair.into_inner();
            let key = field_inner.next()
                .ok_or("Missing field key")?
                .as_str()
                .to_string();
            let value = build_ast_from_expr(
                field_inner.next().ok_or("Missing field value")?
            )?;
            Ok((key, value))
        })
        .collect();

    Ok(AstNode::RecordLiteral(fields?))
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

    // Allow empty parameter lists for lambdas like () => expr
    Ok(params)
}

fn build_function_call(pair: Pair<Rule>) -> Result<AstNode, String> {
    let mut inner = pair.into_inner();

    let first = inner.next()
        .ok_or("Missing function name or expression")?;

    // Check if this is a callable (identifier/self/rec) or an expression (IIFE)
    let is_callable = first.as_rule() == Rule::callable;

    let args: Result<Vec<AstNode>, String> = inner
        .map(|p| build_ast_from_expr(p))
        .collect();

    let args = args?;

    // If it's an expression (IIFE), return CallExpression
    if !is_callable {
        let callee = build_ast_from_expr(first)?;
        return Ok(AstNode::CallExpression {
            callee: Box::new(callee),
            args,
        });
    }

    // Otherwise, it's a normal function call
    let name = first.as_str().to_string();

    // Special handling for if() function
    if name == "if" {
        if args.len() != 3 {
            return Err(format!("if() requires exactly 3 arguments (condition, then, else), got {}", args.len()));
        }

        return Ok(AstNode::If {
            condition: Box::new(args[0].clone()),
            then_expr: Box::new(args[1].clone()),
            else_expr: Box::new(args[2].clone()),
        });
    }

    // Special handling for piecewise() function
    if name == "piecewise" {
        if args.is_empty() {
            return Err("piecewise() requires at least one argument".to_string());
        }

        let mut cases = Vec::new();
        let mut default = None;

        for (i, arg) in args.iter().enumerate() {
            match arg {
                AstNode::ArrayLiteral(elems) => {
                    // This is a [condition, value] case
                    if elems.len() != 2 {
                        return Err(format!(
                            "piecewise() case must have exactly 2 elements [condition, value], got {}",
                            elems.len()
                        ));
                    }
                    cases.push((
                        Box::new(elems[0].clone()),
                        Box::new(elems[1].clone()),
                    ));
                }
                _ => {
                    // This is the default value (must be the last argument)
                    if i != args.len() - 1 {
                        return Err(format!(
                            "piecewise() default value must be the last argument (argument {} is not a case)",
                            i + 1
                        ));
                    }
                    default = Some(Box::new(arg.clone()));
                }
            }
        }

        return Ok(AstNode::Piecewise {
            cases,
            default,
        });
    }

    Ok(AstNode::FunctionCall {
        name,
        args,
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
        assert!(matches!(result[0], AstNode::ArrayLiteral(_)));
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
