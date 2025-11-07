use achronyme_eval::Evaluator;
use std::env;
use std::fs;
use std::io::{self, Write};

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
    println!("Type 'exit' or 'quit' to exit, 'help' for help");
    println!();

    let mut evaluator = Evaluator::new();
    let mut line_number = 1;

    loop {
        print!("ach[{}]> ", line_number);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
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
                        evaluator = Evaluator::new();
                        println!("Environment cleared");
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
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }
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
    println!("Achronyme REPL Commands:");
    println!("  help        - Show this help message");
    println!("  clear       - Clear the environment");
    println!("  exit, quit  - Exit the REPL");
    println!();
    println!("Examples:");
    println!("  2 + 2");
    println!("  x = 5");
    println!("  f = x => x^2");
    println!("  diff(f, 2, 1e-5)");
    println!("  integral(sin, 0, pi, 100)");
    println!("  solve(x => x^2 - 4, 0, 5, 1e-6)");
}
