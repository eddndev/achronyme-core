//! Error types for type checking

/// Error details for type mismatches
#[derive(Debug, Clone, PartialEq)]
pub struct TypeError {
    pub expected: String,
    pub actual: String,
    pub context: Option<String>,
}

impl TypeError {
    pub fn new(expected: String, actual: String) -> Self {
        Self {
            expected,
            actual,
            context: None,
        }
    }

    pub fn with_context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.context {
            Some(ctx) => write!(
                f,
                "Type mismatch in {}: expected {}, got {}",
                ctx, self.expected, self.actual
            ),
            None => write!(
                f,
                "Type mismatch: expected {}, got {}",
                self.expected, self.actual
            ),
        }
    }
}
