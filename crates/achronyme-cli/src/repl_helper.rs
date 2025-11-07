use rustyline::completion::Completer;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Helper, Context};
use std::borrow::Cow;

use crate::highlighter::highlight_code;

pub struct ReplHelper {
    pub functions: Vec<String>,
}

impl ReplHelper {
    pub fn new() -> Self {
        let functions = vec![
            // Trigonometric
            "sin", "cos", "tan", "asin", "acos", "atan", "atan2",
            "sinh", "cosh", "tanh",
            // Exponential and logarithmic
            "exp", "ln", "log", "log10", "log2",
            // Power and roots
            "sqrt", "cbrt", "pow",
            // Rounding
            "floor", "ceil", "round", "abs",
            // Higher-order functions
            "map", "reduce", "filter", "fold",
            // Calculus
            "diff", "integral", "solve", "derivative",
            // Linear algebra
            "dot", "cross", "norm", "det", "inv", "transpose",
            "linprog", "qprog", "milprog",
            // Statistics
            "sum", "mean", "median", "std", "var", "min", "max",
            "corr", "cov",
            // Signal processing
            "fft", "ifft", "fft_mag", "fft_phase", "convolve",
            // Conditional
            "if", "piecewise",
            // Utilities
            "range", "linspace", "length", "head", "tail",
            // Keywords
            "let", "true", "false",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        Self { functions }
    }
}

impl Helper for ReplHelper {}

impl Completer for ReplHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        // Find the start of the current word
        let start = line[..pos]
            .rfind(|c: char| !c.is_alphanumeric() && c != '_')
            .map(|i| i + 1)
            .unwrap_or(0);

        let word = &line[start..pos];

        if word.is_empty() {
            return Ok((pos, Vec::new()));
        }

        let matches: Vec<String> = self
            .functions
            .iter()
            .filter(|f| f.starts_with(word))
            .cloned()
            .collect();

        Ok((start, matches))
    }
}

impl Hinter for ReplHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<Self::Hint> {
        if pos < line.len() {
            return None;
        }

        // Find the start of the current word
        let start = line[..pos]
            .rfind(|c: char| !c.is_alphanumeric() && c != '_')
            .map(|i| i + 1)
            .unwrap_or(0);

        let word = &line[start..];

        if word.is_empty() {
            return None;
        }

        // Find first matching function
        self.functions
            .iter()
            .find(|f| f.starts_with(word) && f.len() > word.len())
            .map(|f| f[word.len()..].to_string())
    }
}

impl Highlighter for ReplHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        Cow::Owned(highlight_code(line))
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _forced: bool) -> bool {
        // Only highlight when forced (e.g., after Enter or specific triggers)
        // This prevents excessive highlighting on every character
        _forced
    }
}

impl Validator for ReplHelper {}
