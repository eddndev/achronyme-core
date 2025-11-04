use crate::ast::{AstNode, BinaryOp, UnaryOp};
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    // Helper methods
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, expected: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(self.peek()) == std::mem::discriminant(expected)
    }

    fn match_token(&mut self, expected: &Token) -> bool {
        if self.check(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume(&mut self, expected: &Token, message: &str) -> Result<&Token, String> {
        if self.check(expected) {
            Ok(self.advance())
        } else {
            Err(message.to_string())
        }
    }

    // Main parse entry point
    pub fn parse(&mut self) -> Result<AstNode, String> {
        self.statement()
    }

    // statement → let IDENTIFIER = expression | expression
    fn statement(&mut self) -> Result<AstNode, String> {
        // Check for let declaration
        if self.match_token(&Token::Let) {
            // let IDENTIFIER = expression
            if let Token::Identifier(name) = self.peek().clone() {
                self.advance();
                self.consume(&Token::Assign, "Expected '=' after variable name")?;
                let initializer = self.expression()?;
                return Ok(AstNode::VariableDecl {
                    name,
                    initializer: Box::new(initializer),
                });
            } else {
                return Err("Expected variable name after 'let'".to_string());
            }
        }

        // Otherwise, it's just an expression
        self.expression()
    }

    // expression → comparison
    fn expression(&mut self) -> Result<AstNode, String> {
        self.comparison()
    }

    // comparison → additive (('>' | '<' | '>=' | '<=' | '==' | '!=') additive)*
    fn comparison(&mut self) -> Result<AstNode, String> {
        let mut node = self.additive()?;

        while self.match_token(&Token::Gt)
            || self.match_token(&Token::Lt)
            || self.match_token(&Token::Gte)
            || self.match_token(&Token::Lte)
            || self.match_token(&Token::Eq)
            || self.match_token(&Token::Neq)
        {
            let op = match self.previous() {
                Token::Gt => BinaryOp::Gt,
                Token::Lt => BinaryOp::Lt,
                Token::Gte => BinaryOp::Gte,
                Token::Lte => BinaryOp::Lte,
                Token::Eq => BinaryOp::Eq,
                Token::Neq => BinaryOp::Neq,
                _ => return Err("Unknown comparison operator".to_string()),
            };

            let right = self.additive()?;
            node = AstNode::BinaryOp {
                op,
                left: Box::new(node),
                right: Box::new(right),
            };
        }

        Ok(node)
    }

    // additive → term (('+' | '-') term)*
    fn additive(&mut self) -> Result<AstNode, String> {
        let mut node = self.term()?;

        while self.match_token(&Token::Plus) || self.match_token(&Token::Minus) {
            let op = match self.previous() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Subtract,
                _ => unreachable!(),
            };

            let right = self.term()?;
            node = AstNode::BinaryOp {
                op,
                left: Box::new(node),
                right: Box::new(right),
            };
        }

        Ok(node)
    }

    // term → factor (('*' | '/' | '%') factor)*
    fn term(&mut self) -> Result<AstNode, String> {
        let mut node = self.factor()?;

        while self.match_token(&Token::Star)
            || self.match_token(&Token::Slash)
            || self.match_token(&Token::Modulo)
        {
            let op = match self.previous() {
                Token::Star => BinaryOp::Multiply,
                Token::Slash => BinaryOp::Divide,
                Token::Modulo => BinaryOp::Modulo,
                _ => unreachable!(),
            };

            let right = self.factor()?;
            node = AstNode::BinaryOp {
                op,
                left: Box::new(node),
                right: Box::new(right),
            };
        }

        Ok(node)
    }

    // factor → exponent ('^' exponent)*
    // Note: Right-associative! We need to handle this carefully
    fn factor(&mut self) -> Result<AstNode, String> {
        let mut node = self.exponent()?;

        if self.match_token(&Token::Caret) {
            // Right-associative: parse the rest as a factor
            let right = self.factor()?; // Recursive call for right-associativity
            node = AstNode::BinaryOp {
                op: BinaryOp::Power,
                left: Box::new(node),
                right: Box::new(right),
            };
        }

        Ok(node)
    }

    // exponent → '-' exponent | primary
    fn exponent(&mut self) -> Result<AstNode, String> {
        if self.match_token(&Token::Minus) {
            // Recursive for multiple negations: --5
            let operand = self.exponent()?;
            return Ok(AstNode::UnaryOp {
                op: UnaryOp::Negate,
                operand: Box::new(operand),
            });
        }

        self.primary()
    }

    // primary → NUMBER 'i'? | IDENTIFIER ('(' args ')')? | '(' expression ')' 'i'? | '[' vector_or_matrix ']'
    fn primary(&mut self) -> Result<AstNode, String> {
        // NUMBER with optional 'i' suffix
        if let Token::Number(value) = self.peek() {
            let value = *value;
            self.advance();

            // Check for imaginary unit: 3i
            if let Token::Identifier(name) = self.peek() {
                if name == "i" {
                    self.advance();
                    return Ok(AstNode::ComplexLiteral {
                        re: 0.0,
                        im: value,
                    });
                }
            }

            return Ok(AstNode::Number(value));
        }

        // Handle identifiers (constants, function calls, lambdas, or 'i')
        if let Token::Identifier(name) = self.peek().clone() {
            self.advance();

            // Special case: standalone 'i' (imaginary unit)
            if name == "i" {
                return Ok(AstNode::ComplexLiteral { re: 0.0, im: 1.0 });
            }

            // Check for single-param lambda: param => expr
            if self.check(&Token::Arrow) {
                self.advance(); // consume =>
                let body = self.expression()?;
                return Ok(AstNode::Lambda {
                    params: vec![name],
                    body: Box::new(body),
                });
            }

            // Check if it's a function call
            if self.check(&Token::LParen) {
                return self.parse_function_call(name);
            }

            // Otherwise, it's a constant or variable reference
            return Ok(AstNode::VariableRef(name));
        }

        // Parenthesized expression or multi-param lambda
        if self.match_token(&Token::LParen) {
            // Lookahead to determine if this is a lambda or expression
            // Lambda: (x, y) => expr
            // Expression: (2 + 3)

            let saved_pos = self.current;

            // Try to parse as lambda parameters
            let mut params = Vec::new();
            let mut is_lambda = false;

            if let Token::Identifier(name) = self.peek().clone() {
                params.push(name);
                self.advance();

                // Check for more parameters
                while self.match_token(&Token::Comma) {
                    if let Token::Identifier(name) = self.peek().clone() {
                        params.push(name);
                        self.advance();
                    } else {
                        // Not a lambda, restore position
                        self.current = saved_pos;
                        params.clear();
                        break;
                    }
                }

                // Must have RPAREN followed by ARROW
                if self.match_token(&Token::RParen) && self.check(&Token::Arrow) {
                    is_lambda = true;
                    self.advance(); // consume =>
                    let body = self.expression()?;
                    return Ok(AstNode::Lambda {
                        params,
                        body: Box::new(body),
                    });
                }
            }

            // Restore position and parse as expression
            if !is_lambda {
                self.current = saved_pos;
            }

            let node = self.expression()?;
            self.consume(&Token::RParen, "Expected ')' after expression")?;

            // Check for imaginary unit: (2+3)i
            // For simplicity, we'll throw an error for now (as C++ does)
            if let Token::Identifier(name) = self.peek() {
                if name == "i" {
                    return Err(
                        "Complex syntax (expr)i not yet fully supported. Use expr * i instead."
                            .to_string(),
                    );
                }
            }

            return Ok(node);
        }

        // Vector or Matrix literal
        if self.match_token(&Token::LBracket) {
            return self.parse_vector_or_matrix();
        }

        Err("Expected expression".to_string())
    }

    // Parse function call: name '(' args ')'
    fn parse_function_call(&mut self, name: String) -> Result<AstNode, String> {
        self.consume(&Token::LParen, "Expected '(' after function name")?;

        let mut args = Vec::new();

        // Parse arguments (if any)
        if !self.check(&Token::RParen) {
            loop {
                args.push(self.expression()?);
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }

        self.consume(&Token::RParen, "Expected ')' after arguments")?;

        Ok(AstNode::FunctionCall { name, args })
    }

    // Parse vector or matrix starting with '['
    // After consuming '[', we need to determine if this is:
    //   - Vector: [expr, expr, expr]
    //   - Matrix: [[expr, expr], [expr, expr]]
    fn parse_vector_or_matrix(&mut self) -> Result<AstNode, String> {
        // We've already consumed the first '['

        // Check if this is a matrix (next token is '[')
        if self.check(&Token::LBracket) {
            // This is a matrix: [[...], [...], ...]
            let mut rows = Vec::new();

            loop {
                self.consume(&Token::LBracket, "Expected '[' for matrix row")?;

                let mut row = Vec::new();
                if !self.check(&Token::RBracket) {
                    loop {
                        row.push(self.expression()?);
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                }

                self.consume(&Token::RBracket, "Expected ']' after matrix row")?;
                rows.push(row);

                if !self.match_token(&Token::Comma) {
                    break;
                }
            }

            self.consume(&Token::RBracket, "Expected ']' after matrix")?;

            // Validate that all rows have the same length
            if !rows.is_empty() {
                let expected_cols = rows[0].len();
                for (i, row) in rows.iter().enumerate() {
                    if row.len() != expected_cols {
                        return Err(format!(
                            "Matrix rows must have the same number of elements. \
                             Row 0 has {} elements, but row {} has {} elements.",
                            expected_cols,
                            i,
                            row.len()
                        ));
                    }
                }
            }

            return Ok(AstNode::MatrixLiteral(rows));
        }

        // This is a vector: [expr, expr, expr]
        let mut elements = Vec::new();

        if !self.check(&Token::RBracket) {
            loop {
                elements.push(self.expression()?);
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }

        self.consume(&Token::RBracket, "Expected ']' after vector")?;

        Ok(AstNode::VectorLiteral(elements))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse(source: &str) -> Result<AstNode, String> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    #[test]
    fn test_simple_number() {
        let ast = parse("42").unwrap();
        assert_eq!(ast, AstNode::Number(42.0));
    }

    #[test]
    fn test_addition() {
        let ast = parse("2 + 3").unwrap();
        match ast {
            AstNode::BinaryOp { op, .. } => assert_eq!(op, BinaryOp::Add),
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_power_right_associative() {
        // 2^3^4 should be 2^(3^4), not (2^3)^4
        let ast = parse("2 ^ 3 ^ 4").unwrap();
        match ast {
            AstNode::BinaryOp {
                op: BinaryOp::Power,
                left,
                right,
            } => {
                assert_eq!(*left, AstNode::Number(2.0));
                match *right {
                    AstNode::BinaryOp {
                        op: BinaryOp::Power,
                        ..
                    } => {} // Correct: right side is another power operation
                    _ => panic!("Power should be right-associative"),
                }
            }
            _ => panic!("Expected power operation"),
        }
    }

    #[test]
    fn test_precedence() {
        // 2 + 3 * 4 should be 2 + (3 * 4)
        let ast = parse("2 + 3 * 4").unwrap();
        match ast {
            AstNode::BinaryOp {
                op: BinaryOp::Add,
                left,
                right,
            } => {
                assert_eq!(*left, AstNode::Number(2.0));
                match *right {
                    AstNode::BinaryOp {
                        op: BinaryOp::Multiply,
                        ..
                    } => {} // Correct
                    _ => panic!("Multiplication should have higher precedence"),
                }
            }
            _ => panic!("Expected addition at top level"),
        }
    }

    #[test]
    fn test_vector() {
        let ast = parse("[1, 2, 3]").unwrap();
        match ast {
            AstNode::VectorLiteral(elements) => assert_eq!(elements.len(), 3),
            _ => panic!("Expected vector literal"),
        }
    }

    #[test]
    fn test_function_call() {
        let ast = parse("sin(PI)").unwrap();
        match ast {
            AstNode::FunctionCall { name, args } => {
                assert_eq!(name, "sin");
                assert_eq!(args.len(), 1);
            }
            _ => panic!("Expected function call"),
        }
    }

    #[test]
    fn test_let_statement() {
        let ast = parse("let x = 5").unwrap();
        match ast {
            AstNode::VariableDecl { name, .. } => {
                assert_eq!(name, "x");
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_lambda() {
        let ast = parse("x => x * 2").unwrap();
        match ast {
            AstNode::Lambda { params, .. } => {
                assert_eq!(params.len(), 1);
                assert_eq!(params[0], "x");
            }
            _ => panic!("Expected lambda"),
        }
    }

    #[test]
    fn test_comparison() {
        let ast = parse("x > 5").unwrap();
        match ast {
            AstNode::BinaryOp { op, .. } => assert_eq!(op, BinaryOp::Gt),
            _ => panic!("Expected comparison"),
        }
    }
}
