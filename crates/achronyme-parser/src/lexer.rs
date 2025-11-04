use crate::token::Token;

pub struct Lexer<'a> {
    source: &'a str,
    pos: usize,  // Current byte position in source
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            pos: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.source.len()
    }

    fn peek(&self) -> Option<char> {
        self.source[self.pos..].chars().next()
    }

    fn peek_next(&self) -> Option<char> {
        let mut chars = self.source[self.pos..].chars();
        chars.next();
        chars.next()
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.peek() {
            self.pos += ch.len_utf8();
            Some(ch)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn scan_number(&mut self) -> Result<Token, String> {
        let start_pos = self.pos;

        // Integer part
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        // Decimal part
        if self.peek() == Some('.') && self.peek_next().map_or(false, |c| c.is_ascii_digit()) {
            self.advance(); // consume '.'
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        // Scientific notation (e.g., 1e-3, 2.5e10)
        if let Some('e') | Some('E') = self.peek() {
            self.advance(); // consume 'e' or 'E'
            if let Some('+') | Some('-') = self.peek() {
                self.advance(); // consume sign
            }
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        let number_str = &self.source[start_pos..self.pos];
        let number = number_str
            .parse::<f64>()
            .map_err(|e| format!("Invalid number '{}': {}", number_str, e))?;
        Ok(Token::Number(number))
    }

    fn scan_identifier(&mut self) -> Token {
        let start_pos = self.pos;

        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let identifier = &self.source[start_pos..self.pos];

        // Check for keywords
        match identifier {
            "let" => Token::Let,
            _ => Token::Identifier(identifier.to_string()),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.skip_whitespace();

            if self.is_at_end() {
                break;
            }

            let c = self.peek().unwrap();
            let token = match c {
                // Single-character tokens
                '+' => {
                    self.advance();
                    Token::Plus
                }
                '-' => {
                    self.advance();
                    Token::Minus
                }
                '*' => {
                    self.advance();
                    Token::Star
                }
                '/' => {
                    self.advance();
                    Token::Slash
                }
                '^' => {
                    self.advance();
                    Token::Caret
                }
                '%' => {
                    self.advance();
                    Token::Modulo
                }
                '(' => {
                    self.advance();
                    Token::LParen
                }
                ')' => {
                    self.advance();
                    Token::RParen
                }
                '[' => {
                    self.advance();
                    Token::LBracket
                }
                ']' => {
                    self.advance();
                    Token::RBracket
                }
                ',' => {
                    self.advance();
                    Token::Comma
                }
                ';' => {
                    self.advance();
                    Token::Semicolon
                }

                // Multi-character tokens
                '=' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        Token::Eq
                    } else if self.peek() == Some('>') {
                        self.advance();
                        Token::Arrow
                    } else {
                        Token::Assign
                    }
                }
                '>' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        Token::Gte
                    } else {
                        Token::Gt
                    }
                }
                '<' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        Token::Lte
                    } else {
                        Token::Lt
                    }
                }
                '!' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        Token::Neq
                    } else {
                        return Err(format!("Unexpected '!' - did you mean '!='?"));
                    }
                }

                // Numbers
                '0'..='9' | '.' => {
                    self.scan_number()?
                }

                // Identifiers and keywords
                'a'..='z' | 'A'..='Z' | '_' => {
                    self.scan_identifier()
                }

                _ => {
                    return Err(format!("Unexpected character: '{}'", c));
                }
            };

            tokens.push(token);
        }

        tokens.push(Token::Eof);
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new("42 3.14 1.5e-3");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0], Token::Number(42.0));
        assert_eq!(tokens[1], Token::Number(3.14));
        assert_eq!(tokens[2], Token::Number(1.5e-3));
    }

    #[test]
    fn test_operators() {
        let mut lexer = Lexer::new("+ - * / ^ %");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0], Token::Plus);
        assert_eq!(tokens[1], Token::Minus);
        assert_eq!(tokens[2], Token::Star);
        assert_eq!(tokens[3], Token::Slash);
        assert_eq!(tokens[4], Token::Caret);
        assert_eq!(tokens[5], Token::Modulo);
    }

    #[test]
    fn test_comparisons() {
        let mut lexer = Lexer::new("> < >= <= == !=");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0], Token::Gt);
        assert_eq!(tokens[1], Token::Lt);
        assert_eq!(tokens[2], Token::Gte);
        assert_eq!(tokens[3], Token::Lte);
        assert_eq!(tokens[4], Token::Eq);
        assert_eq!(tokens[5], Token::Neq);
    }

    #[test]
    fn test_arrow() {
        let mut lexer = Lexer::new("x => x * 2");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0], Token::Identifier("x".to_string()));
        assert_eq!(tokens[1], Token::Arrow);
        assert_eq!(tokens[2], Token::Identifier("x".to_string()));
        assert_eq!(tokens[3], Token::Star);
        assert_eq!(tokens[4], Token::Number(2.0));
    }

    #[test]
    fn test_let_statement() {
        let mut lexer = Lexer::new("let x = 5");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0], Token::Let);
        assert_eq!(tokens[1], Token::Identifier("x".to_string()));
        assert_eq!(tokens[2], Token::Assign);
        assert_eq!(tokens[3], Token::Number(5.0));
    }

    /// Test de compatibilidad completa con C++
    /// Verifica que producimos exactamente los mismos tokens que el lexer C++
    #[test]
    fn test_cpp_compatibility_all_tokens() {
        // Este test replica exactamente lo que el lexer C++ debería producir
        let test_cases = vec![
            // Números
            ("42", vec![Token::Number(42.0), Token::Eof]),
            ("3.14", vec![Token::Number(3.14), Token::Eof]),
            ("1.5e-3", vec![Token::Number(1.5e-3), Token::Eof]),

            // Operadores aritméticos
            ("+ - * / ^ %", vec![
                Token::Plus, Token::Minus, Token::Star,
                Token::Slash, Token::Caret, Token::Modulo, Token::Eof
            ]),

            // Comparaciones
            ("> < >= <= == !=", vec![
                Token::Gt, Token::Lt, Token::Gte,
                Token::Lte, Token::Eq, Token::Neq, Token::Eof
            ]),

            // Asignación y arrow
            ("= =>", vec![Token::Assign, Token::Arrow, Token::Eof]),

            // Delimitadores
            ("( ) [ ] , ;", vec![
                Token::LParen, Token::RParen, Token::LBracket,
                Token::RBracket, Token::Comma, Token::Semicolon, Token::Eof
            ]),

            // Keywords e identificadores
            ("let sin PI", vec![
                Token::Let,
                Token::Identifier("sin".to_string()),
                Token::Identifier("PI".to_string()),
                Token::Eof
            ]),

            // Expresión compleja (como en C++)
            ("let x = sin(PI / 2)", vec![
                Token::Let,
                Token::Identifier("x".to_string()),
                Token::Assign,
                Token::Identifier("sin".to_string()),
                Token::LParen,
                Token::Identifier("PI".to_string()),
                Token::Slash,
                Token::Number(2.0),
                Token::RParen,
                Token::Eof
            ]),
        ];

        for (source, expected) in test_cases {
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize().unwrap();
            assert_eq!(
                tokens, expected,
                "Failed for input: '{}'\nExpected: {:?}\nGot: {:?}",
                source, expected, tokens
            );
        }
    }
}
