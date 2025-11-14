/// Real-world examples of string broadcasting usage
/// Demonstrates the practical use cases requested by the user

use achronyme_eval::Evaluator;
use achronyme_types::value::Value;

fn eval(source: &str) -> Result<Value, String> {
    let mut evaluator = Evaluator::new();
    evaluator.eval_str(source)
}

#[test]
fn test_print_separator_line() {
    // Original request: print("-" * 40)
    let result = eval(r#"print("-" * 40)"#).unwrap();
    assert_eq!(result, Value::String("-".repeat(40)));
}

#[test]
fn test_print_result_with_value() {
    // Original request: print("Resultado: " + resultado)
    let result = eval(r#"
        let resultado = 42
        print("Resultado: " + resultado)
    "#).unwrap();
    assert_eq!(result, Value::String("Resultado: 42".to_string()));
}

#[test]
fn test_report_header() {
    // Practical example: creating a formatted report header
    let result = eval(r#"
        let width = 40
        let title = "INFORME MENSUAL"
        let border = "=" * width
        border + "\n" + title + "\n" + border
    "#).unwrap();

    let expected = format!("{}\nINFORME MENSUAL\n{}", "=".repeat(40), "=".repeat(40));
    assert_eq!(result, Value::String(expected));
}

#[test]
fn test_formatted_message() {
    // Combining string + number + string
    let result = eval(r#"
        let username = "Alice"
        let score = 95
        let max_score = 100
        username + " obtuvo " + score + " de " + max_score + " puntos"
    "#).unwrap();
    assert_eq!(result, Value::String("Alice obtuvo 95 de 100 puntos".to_string()));
}

#[test]
fn test_progress_bar() {
    // Creating a simple text progress bar
    let result = eval(r#"
        let filled = 7
        let empty = 3
        let total = filled + empty
        "[" + ("█" * filled) + ("░" * empty) + "] " + (filled * 10) + "%"
    "#).unwrap();
    assert_eq!(result, Value::String("[███████░░░] 70%".to_string()));
}

#[test]
fn test_table_row() {
    // Creating formatted table rows
    let result = eval(r#"
        let col1 = "Producto"
        let col2 = 150
        let col3 = "unidades"
        let sep = " | "
        col1 + sep + col2 + sep + col3
    "#).unwrap();
    assert_eq!(result, Value::String("Producto | 150 | unidades".to_string()));
}

#[test]
fn test_indented_text() {
    // Creating indented text with string repetition
    let result = eval(r#"
        let level = 3
        let indent = "  " * level
        let text = "Texto indentado"
        indent + text
    "#).unwrap();
    assert_eq!(result, Value::String("      Texto indentado".to_string()));
}

#[test]
fn test_debug_output() {
    // Debug-style output with labels
    let result = eval(r#"
        let x = 10
        let y = 20
        let z = x + y
        "x=" + x + ", y=" + y + ", sum=" + z
    "#).unwrap();
    assert_eq!(result, Value::String("x=10, y=20, sum=30".to_string()));
}

#[test]
fn test_multiline_report() {
    // Creating a multi-line report
    let result = eval(r#"
        let name = "Sistema A"
        let status = true
        let uptime = 99.9
        let line = "-" * 30
        line + "\n" +
        "Nombre: " + name + "\n" +
        "Estado: " + status + "\n" +
        "Uptime: " + uptime + "%\n" +
        line
    "#).unwrap();

    let expected = format!(
        "{}\nNombre: Sistema A\nEstado: true\nUptime: 99.9%\n{}",
        "-".repeat(30),
        "-".repeat(30)
    );
    assert_eq!(result, Value::String(expected));
}

#[test]
fn test_dynamic_padding() {
    // Creating dynamic padding based on calculation
    let result = eval(r#"
        let text = "Centro"
        let total_width = 20
        let text_len = length(text)
        let padding = (total_width - text_len) / 2
        let pad = " " * padding
        pad + text + pad
    "#).unwrap();
    // padding = (20 - 6) / 2 = 7
    assert_eq!(result, Value::String("       Centro       ".to_string()));
}

#[test]
fn test_function_with_formatting() {
    // Reusable formatting function
    let result = eval(r#"
        let formatPrice = (product, price) => do {
            product + ": $" + price
        }

        formatPrice("Laptop", 1299)
    "#).unwrap();
    assert_eq!(result, Value::String("Laptop: $1299".to_string()));
}

#[test]
fn test_box_drawing() {
    // Creating a simple text box
    let result = eval(r#"
        let width = 20
        let content = "Mensaje"
        let top = "┌" + ("─" * width) + "┐"
        let middle = "│ " + content + (" " * (width - length(content) - 1)) + "│"
        let bottom = "└" + ("─" * width) + "┘"
        top + "\n" + middle + "\n" + bottom
    "#).unwrap();

    let expected = format!(
        "┌{}┐\n│ Mensaje            │\n└{}┘",
        "─".repeat(20),
        "─".repeat(20)
    );
    assert_eq!(result, Value::String(expected));
}

#[test]
fn test_concatenate_without_str_function() {
    // User specifically mentioned: not using str() directly
    // This should work directly without needing a str() conversion function
    let result = eval(r#"
        let value = 42
        let active = true
        let items = [1, 2, 3]

        "Valor: " + value + ", Activo: " + active + ", Items: " + items
    "#).unwrap();
    assert_eq!(
        result,
        Value::String("Valor: 42, Activo: true, Items: [1, 2, 3]".to_string())
    );
}
