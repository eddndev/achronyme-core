use pest::iterators::Pair;
use crate::ast::AstNode;
use crate::parser::AstParser;
use crate::pest_parser::Rule;

impl AstParser {
    pub(super) fn build_primary(&mut self, pair: Pair<Rule>) -> Result<AstNode, String> {
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
                let processed = self.process_escape_sequences(content);
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
            Rule::null_literal => {
                Ok(AstNode::Null)
            }
            Rule::array => self.build_array(inner),
            Rule::vector => self.build_array(inner),  // Alias for array
            Rule::matrix => self.build_array(inner),  // Alias for array
            Rule::record => self.build_record(inner),
            Rule::control_flow_expr => self.build_control_flow_expr(inner),
            Rule::do_block => self.build_do_block(inner),
            Rule::lambda => self.build_lambda(inner),

            Rule::expr => self.build_ast_from_expr(inner),
            _ => Err(format!("Unexpected primary rule: {:?}", inner.as_rule()))
        }
    }
}