use achronyme_eval::Evaluator;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Config};
use std::env;
use std::fs;

mod highlighter;
mod repl_helper;

use repl_helper::ReplHelper;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // No arguments: REPL mode
        1 => run_repl(),

        // One argument: could be file or expression
        2 => {
            let input = &args[1];

            // Check if it's a file (ends with .ach or .soc)
            if input.ends_with(".ach") || input.ends_with(".soc") {
                run_file(input);
            } else {
                // Treat as expression
                run_expression(input);
            }
        }

        // More arguments: error
        _ => {
            print_usage(&args[0]);
            std::process::exit(1);
        }
    }
}

fn print_usage(program_name: &str) {
    eprintln!("Achronyme CLI - Scientific Computing Language");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  {}                    # Start REPL (interactive mode)", program_name);
    eprintln!("  {} <file.ach>         # Execute a script file", program_name);
    eprintln!("  {} <expression>       # Evaluate a single expression", program_name);
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  {} script.ach", program_name);
    eprintln!("  {} \"2 + 2\"", program_name);
    eprintln!("  {} \"diff(x => x^2, 2, 1e-5)\"", program_name);
}

fn run_repl() {
    println!("Achronyme REPL v0.1.0");
    println!("Type 'exit' or 'quit' to exit, 'help' for help, 'clear' to clear screen");
    println!();

    let config = Config::builder()
        .auto_add_history(true)
        .build();

    let helper = ReplHelper::new();
    let mut rl = Editor::with_config(config).expect("Failed to create editor");
    rl.set_helper(Some(helper));

    // Load history from file
    let history_path = dirs::home_dir()
        .map(|mut p| {
            p.push(".achronyme_history");
            p
        });

    if let Some(ref path) = history_path {
        let _ = rl.load_history(path);
    }

    let mut evaluator = Evaluator::new();
    let mut line_number = 1;

    loop {
        let prompt = format!("ach[{}]> ", line_number);

        match rl.readline(&prompt) {
            Ok(input) => {
                let input = input.trim();

                // Handle special commands
                match input {
                    "exit" | "quit" => {
                        println!("Goodbye!");
                        break;
                    }
                    "help" => {
                        print_help();
                        continue;
                    }
                    "clear" => {
                        // Clear the screen
                        clear_screen();
                        // Also reset the evaluator environment
                        evaluator = Evaluator::new();
                        println!("Screen cleared and environment reset");
                        continue;
                    }
                    "cls" => {
                        // Just clear the screen without resetting environment
                        clear_screen();
                        continue;
                    }
                    "" => continue,
                    _ => {}
                }

                // Evaluate expression
                match evaluate_expression(&mut evaluator, input) {
                    Ok(result) => println!("{}", result),
                    Err(err) => eprintln!("Error: {}", err),
                }

                line_number += 1;
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("Goodbye!");
                break;
            }
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                break;
            }
        }
    }

    // Save history to file
    if let Some(path) = history_path {
        let _ = rl.save_history(&path);
    }
}

fn clear_screen() {
    // Cross-platform screen clearing
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(&["/C", "cls"])
            .status();
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = std::process::Command::new("clear").status();
    }

    // Also try ANSI escape sequence as fallback
    print!("\x1B[2J\x1B[1;1H");
}

fn run_file(filename: &str) {
    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", filename, err);
            std::process::exit(1);
        }
    };

    let mut evaluator = Evaluator::new();

    // Parse and evaluate the entire file using Pest
    match evaluate_expression(&mut evaluator, &contents) {
        Ok(result) => println!("{}", result),
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}

fn run_expression(expr: &str) {
    let mut evaluator = Evaluator::new();

    match evaluate_expression(&mut evaluator, expr) {
        Ok(result) => println!("{}", result),
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}

fn evaluate_expression(evaluator: &mut Evaluator, input: &str) -> Result<String, String> {
    // Parse and evaluate using Pest
    let result = evaluator.eval_str(input)?;

    // Format result
    Ok(format_value(&result))
}

fn format_value(value: &achronyme_types::value::Value) -> String {
    use achronyme_types::value::Value;

    match value {
        Value::Number(n) => format!("{}", n),
        Value::Boolean(b) => format!("{}", b),
        Value::String(s) => format!("\"{}\"", s),
        Value::Complex(c) => {
            if c.im >= 0.0 {
                format!("{}+{}i", c.re, c.im)
            } else {
                format!("{}{}i", c.re, c.im)
            }
        }
        Value::Vector(v) => {
            let elements: Vec<String> = v.data().iter()
                .map(|x| format!("{}", x))
                .collect();
            format!("[{}]", elements.join(", "))
        }
        Value::ComplexVector(cv) => {
            let elements: Vec<String> = cv.data().iter()
                .map(|c| {
                    if c.im >= 0.0 {
                        format!("{}+{}i", c.re, c.im)
                    } else {
                        format!("{}{}i", c.re, c.im)
                    }
                })
                .collect();
            format!("[{}]", elements.join(", "))
        }
        Value::Matrix(m) => {
            let mut rows = Vec::new();
            for i in 0..m.rows {
                let mut row_elements = Vec::new();
                for j in 0..m.cols {
                    if let Ok(val) = m.get(i, j) {
                        row_elements.push(format!("{}", val));
                    }
                }
                rows.push(format!("[{}]", row_elements.join(", ")));
            }
            format!("[{}]", rows.join(",\n "))
        }
        Value::Function(_) => "<function>".to_string(),
    }
}

fn print_help() {
    use nu_ansi_term::Color;

    println!("{}", Color::Green.bold().paint("Achronyme REPL Commands:"));
    println!("  {}        - Show this help message", Color::Cyan.paint("help"));
    println!("  {}       - Clear screen and reset environment", Color::Cyan.paint("clear"));
    println!("  {}        - Clear screen only (keep environment)", Color::Cyan.paint("cls"));
    println!("  {}  - Exit the REPL", Color::Cyan.paint("exit, quit"));
    println!();
    println!("{}", Color::Green.bold().paint("Features:"));
    println!("  - Syntax highlighting (automatic)");
    println!("  - Command history (use ↑/↓ arrows)");
    println!("  - Tab completion for built-in functions");
    println!("  - History saved to ~/.achronyme_history");
    println!();
    println!("{}", Color::Green.bold().paint("Examples:"));
    println!("  {}         - Basic arithmetic", Color::Yellow.paint("2 + 2"));
    println!("  {}  - Variable assignment", Color::Yellow.paint("let x = 5"));
    println!("  {}    - Lambda function", Color::Yellow.paint("let f = x => x^2"));
    println!("  {}  - Numerical derivative", Color::Yellow.paint("diff(f, 2, 1e-5)"));
    println!("  {}  - Numerical integration", Color::Yellow.paint("integral(sin, 0, 3.14159, 100)"));
    println!("  {}  - FFT magnitude", Color::Yellow.paint("fft_mag([1, 2, 3, 4])"));
}
