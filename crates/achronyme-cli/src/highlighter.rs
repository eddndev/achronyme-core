use nu_ansi_term::Color;

/// Highlights SOC syntax with ANSI colors using a simple lexer approach
pub fn highlight_code(input: &str) -> String {
    highlight_partial(input)
}

fn is_builtin_function(name: &str) -> bool {
    matches!(name,
        // Trigonometric
        "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | "atan2" |
        "sinh" | "cosh" | "tanh" |
        // Exponential and logarithmic
        "exp" | "ln" | "log" | "log10" | "log2" |
        // Power and roots
        "sqrt" | "cbrt" | "pow" |
        // Rounding
        "floor" | "ceil" | "round" | "abs" |
        // Higher-order functions
        "map" | "reduce" | "filter" | "fold" |
        // Calculus
        "diff" | "integral" | "solve" | "derivative" |
        // Linear algebra
        "dot" | "cross" | "norm" | "det" | "inv" | "transpose" |
        "linprog" | "qprog" | "milprog" |
        // Statistics
        "sum" | "mean" | "median" | "std" | "var" | "min" | "max" |
        "corr" | "cov" |
        // Signal processing
        "fft" | "ifft" | "fft_mag" | "fft_phase" | "convolve" |
        // Conditional
        "if" | "piecewise" |
        // Utilities
        "range" | "linspace" | "length" | "head" | "tail"
    )
}

fn highlight_partial(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            // Numbers
            '0'..='9' => {
                let mut num = String::from(ch);
                while let Some(&next) = chars.peek() {
                    if next.is_ascii_digit() || next == '.' || next == 'e' || next == 'E' || next == '-' || next == '+' {
                        num.push(chars.next().unwrap());
                    } else if next == 'i' {
                        num.push(chars.next().unwrap());
                        result.push_str(&Color::LightCyan.paint(&num).to_string());
                        break;
                    } else {
                        break;
                    }
                }
                if !num.ends_with('i') {
                    result.push_str(&Color::Cyan.paint(&num).to_string());
                }
            }

            // Operators
            '+' | '*' | '/' | '%' | '^' => {
                result.push_str(&Color::Red.bold().paint(ch.to_string()).to_string());
            }

            '-' => {
                result.push_str(&Color::Red.bold().paint(ch.to_string()).to_string());
            }

            // Comparison and logical operators
            '=' | '!' | '<' | '>' | '&' | '|' => {
                let mut op = String::from(ch);
                if let Some(&next) = chars.peek() {
                    if next == '=' || next == '&' || next == '|' || next == '>' {
                        op.push(chars.next().unwrap());
                    }
                }
                result.push_str(&Color::Red.bold().paint(&op).to_string());
            }

            // Brackets
            '[' | ']' | '(' | ')' => {
                result.push_str(&Color::LightBlue.bold().paint(ch.to_string()).to_string());
            }

            // Identifiers and keywords
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::from(ch);
                while let Some(&next) = chars.peek() {
                    if next.is_alphanumeric() || next == '_' {
                        ident.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                let colored = match ident.as_str() {
                    "true" | "false" => Color::Yellow.paint(&ident).to_string(),
                    "let" => Color::Purple.bold().paint(&ident).to_string(),
                    _ if is_builtin_function(&ident) => Color::Green.bold().paint(&ident).to_string(),
                    _ => Color::White.paint(&ident).to_string(),
                };
                result.push_str(&colored);
            }

            _ => result.push(ch),
        }
    }

    result
}
