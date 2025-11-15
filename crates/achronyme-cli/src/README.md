# Achronyme CLI - Documentación de implementación

**Arquitectura interna y guía para contribuidores del CLI de Achronyme.**

## 🏛️ Arquitectura interna

### Estructura de archivos

```
src/
├── main.rs           # Entry point, argument parsing, mode dispatch
├── lib.rs            # Public re-exports (para uso como library)
├── repl_helper.rs    # Integración con rustyline (Helper trait)
└── highlighter.rs    # Syntax highlighting con ANSI colors
```

### Diagrama de flujo completo

```
┌─────────────────────────────────────────────────────────────┐
│                        main()                                │
│  • Parse command-line arguments                             │
│  • Determine execution mode                                 │
└─────────────────────────────────────────────────────────────┘
                            │
            ┌───────────────┼───────────────┐
            ↓               ↓               ↓
    ┌──────────────┐ ┌──────────────┐ ┌──────────────┐
    │  run_repl()  │ │ run_file()   │ │run_expression│
    │              │ │              │ │              │
    │ Mode 1:      │ │ Mode 2:      │ │ Mode 3:      │
    │ Interactive  │ │ File exec    │ │ One-shot     │
    └──────────────┘ └──────────────┘ └──────────────┘
            │               │               │
            │               └───────┬───────┘
            │                       │
            │         ┌─────────────────────────┐
            │         │ evaluate_expression()   │
            │         │  • evaluator.eval_str() │
            │         │  • format_value()       │
            │         └─────────────────────────┘
            │                       │
            ↓                       ↓
    ┌─────────────────────────────────────┐
    │         REPL Loop                   │
    │  ┌──────────────────────────────┐   │
    │  │ 1. Read line (rustyline)     │   │
    │  │    └─> ReplHelper:           │   │
    │  │        • Highlighter         │   │
    │  │        • Completer           │   │
    │  │        • Hinter              │   │
    │  └──────────────────────────────┘   │
    │  ┌──────────────────────────────┐   │
    │  │ 2. Check special commands    │   │
    │  │    • help, clear, cls, exit  │   │
    │  └──────────────────────────────┘   │
    │  ┌──────────────────────────────┐   │
    │  │ 3. Multi-line detection      │   │
    │  │    └─> should_continue?      │   │
    │  │        • Balance delimiters  │   │
    │  │        • Parser validation   │   │
    │  └──────────────────────────────┘   │
    │  ┌──────────────────────────────┐   │
    │  │ 4. Evaluate expression       │   │
    │  │    └─> evaluate_expression() │   │
    │  └──────────────────────────────┘   │
    │  ┌──────────────────────────────┐   │
    │  │ 5. Format and print result   │   │
    │  │    └─> format_value()        │   │
    │  └──────────────────────────────┘   │
    │  ┌──────────────────────────────┐   │
    │  │ 6. Loop back to step 1       │   │
    │  └──────────────────────────────┘   │
    └─────────────────────────────────────┘
```

## 📁 Responsabilidades de cada módulo

### 1. `main.rs` - Entry point y modos de ejecución

**Responsabilidades:**
- Parsear argumentos de línea de comandos
- Determinar modo de ejecución (REPL / File / Expression)
- Dispatcher a la función apropiada
- Manejo de errores de alto nivel

**Funciones principales:**

#### `main()`
Entry point del programa. Parsea argumentos y delega:

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_repl(),           // Sin argumentos → REPL
        2 => {
            if ends_with_script_ext(&args[1]) {
                run_file(&args[1])  // *.soc o *.ach → File mode
            } else {
                run_expression(&args[1])  // Cualquier otra cosa → Expression
            }
        }
        _ => print_usage(&args[0]),
    }
}
```

**Detección de archivos:**
- `*.soc` → Script SOC (Scientific Operations Calculator)
- `*.ach` → Script Achronyme legacy
- Cualquier otro input → Expresión

#### `run_repl()`
Implementa el **loop REPL** (Read-Eval-Print-Loop):

**Componentes:**
1. **Configuración inicial:**
   ```rust
   let config = Config::builder()
       .auto_add_history(true)  // Historial automático
       .build();

   let helper = ReplHelper::new();
   let mut rl = Editor::with_config(config)?;
   rl.set_helper(Some(helper));  // Syntax highlighting + autocomplete
   ```

2. **Carga de historial:**
   ```rust
   let history_path = dirs::home_dir()
       .map(|mut p| {
           p.push(".achronyme_history");
           p
       });

   if let Some(ref path) = history_path {
       let _ = rl.load_history(path);
   }
   ```

3. **Loop principal:**
   ```rust
   let mut evaluator = Evaluator::new();
   let mut line_number = 1;
   let mut input_buffer = String::new();  // Para multi-line

   loop {
       // 1. Read line
       let prompt = if input_buffer.is_empty() {
           format!("ach[{}]> ", line_number)
       } else {
           "     ...> ".to_string()  // Continuation prompt
       };

       match rl.readline(&prompt) {
           Ok(line) => {
               input_buffer.push_str(&line);

               // 2. Check special commands
               if input_buffer.trim() == "exit" { break; }
               // ... más comandos ...

               // 3. Check if complete
               if should_continue_reading(&input_buffer) {
                   continue;  // Esperar más input
               }

               // 4. Evaluate
               match evaluate_expression(&mut evaluator, &input_buffer) {
                   Ok(result) => println!("{}", result),
                   Err(err) => eprintln!("Error: {}", err),
               }

               input_buffer.clear();
               line_number += 1;
           }
           Err(ReadlineError::Interrupted) => {
               println!("^C");
               input_buffer.clear();
           }
           Err(ReadlineError::Eof) => break,
           Err(err) => {
               eprintln!("Error: {}", err);
               break;
           }
       }
   }

   // 5. Save history
   if let Some(path) = history_path {
       let _ = rl.save_history(&path);
   }
   ```

**Características del REPL:**
- ✅ Multi-line input con prompts de continuación
- ✅ Historial persistente en disco
- ✅ Comandos especiales (`help`, `clear`, `exit`)
- ✅ Manejo de Ctrl+C (no sale, solo cancela línea actual)
- ✅ Manejo de Ctrl+D/EOF (sale gracefully)
- ✅ Numeración de líneas para contexto

#### `should_continue_reading()` - Multi-line detection

**Estrategia híbrida de 2 fases:**

```rust
fn should_continue_reading(input: &str) -> bool {
    // Fase 1: Fast check - balance de delimitadores
    if !has_balanced_delimiters(input) {
        return true;  // Definitivamente incompleto
    }

    // Fase 2: Parser check - confirmar completitud
    match achronyme_parser::parse(input) {
        Ok(_) => false,  // ✅ Completo y válido
        Err(e) => {
            let error_msg = e.to_string();
            // Pest reporta "expected X, found EOI" cuando termina prematuramente
            error_msg.contains("expected") && error_msg.contains("EOI")
        }
    }
}
```

**¿Por qué híbrido?**

1. **Fast check primero** - O(n), rápido:
   - Detecta casos obvios (`[1, 2,` sin cerrar)
   - Evita parsear si obviamente está incompleto
   - Maneja strings correctamente (ignora delimitadores dentro de `"..."`)

2. **Parser check después** - Más lento pero preciso:
   - Distingue "incompleto" de "completo pero inválido"
   - Permite mostrar errores de sintaxis inmediatamente
   - Usa el parser real (no duplica lógica)

**Ejemplos:**

```javascript
// Incompleto (Fase 1 detecta):
"let x = [1, 2, 3"  // ← '[' sin cerrar
// should_continue → true

// Incompleto (Fase 2 detecta):
"let x ="  // ← Delimiters balanceados pero incompleto
// should_continue → true (parser dice "expected expression, found EOI")

// Completo pero inválido:
"let x = )"  // ← Error de sintaxis
// should_continue → false (mostrar error)

// Completo y válido:
"let x = 5"
// should_continue → false (evaluar)
```

#### `has_balanced_delimiters()` - Fast delimiter check

```rust
fn has_balanced_delimiters(input: &str) -> bool {
    let mut paren_count = 0;   // ()
    let mut brace_count = 0;   // {}
    let mut bracket_count = 0; // []
    let mut in_string = false;
    let mut escape_next = false;

    for ch in input.chars() {
        if escape_next {
            escape_next = false;
            continue;
        }

        match ch {
            '\\' if in_string => escape_next = true,
            '"' => in_string = !in_string,
            '(' if !in_string => paren_count += 1,
            ')' if !in_string => {
                paren_count -= 1;
                if paren_count < 0 { return false; }  // Más closing que opening
            }
            // ... similar para {} y []
            _ => {}
        }
    }

    // Balanceado si todos los contadores = 0 y no estamos en un string
    paren_count == 0 && brace_count == 0 && bracket_count == 0 && !in_string
}
```

**Features:**
- ✅ Cuenta paréntesis, llaves y corchetes
- ✅ Ignora delimitadores dentro de strings (`"[123]"` no cuenta como bracket)
- ✅ Maneja escape sequences (`"\""` no termina el string)
- ✅ Detecta más closing que opening (error inmediato)

#### `run_file()` - File execution mode

```rust
fn run_file(filename: &str) {
    // 1. Read file contents
    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|err| {
            eprintln!("Error reading file '{}': {}", filename, err);
            std::process::exit(1);
        });

    // 2. Create fresh evaluator
    let mut evaluator = Evaluator::new();

    // 3. Set current file directory (para imports relativos)
    evaluator.set_current_file_dir(filename);

    // 4. Evaluate entire file
    match evaluate_expression(&mut evaluator, &contents) {
        Ok(result) => println!("{}", result),
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}
```

**Características:**
- ✅ Evalúa el archivo completo como una unidad
- ✅ Soporta imports relativos (se resuelven desde el directorio del archivo)
- ✅ Exit code 1 en caso de error
- ✅ Imprime solo el resultado de la última expresión

#### `run_expression()` - Single expression mode

```rust
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
```

**Uso típico:**
```bash
# Calculadora de línea de comandos
achronyme "2 + 2"

# En scripts de shell
result=$(achronyme "mean([1, 2, 3, 4, 5])")
echo "Average: $result"
```

#### `evaluate_expression()` - Evaluation wrapper

```rust
fn evaluate_expression(evaluator: &mut Evaluator, input: &str)
    -> Result<String, String>
{
    // 1. Parse and evaluate
    let result = evaluator.eval_str(input)?;

    // 2. Format for display
    Ok(format_value(&result))
}
```

Simple wrapper que:
1. Llama al evaluator
2. Formatea el resultado
3. Retorna como String

#### `format_value()` - Result formatting

Formatea valores de forma legible para el usuario:

```rust
fn format_value(value: &Value) -> String {
    match value {
        Value::Number(n) => format!("{}", n),
        Value::Boolean(b) => format!("{}", b),
        Value::String(s) => format!("\"{}\"", s),

        Value::Complex(c) => {
            if c.im >= 0.0 {
                format!("{}+{}i", c.re, c.im)
            } else {
                format!("{}{}i", c.re, c.im)  // im ya tiene el signo -
            }
        }

        Value::Vector(v) => {
            let elements: Vec<String> = v.iter()
                .map(|val| format_value(val))
                .collect();
            format!("[{}]", elements.join(", "))
        }

        Value::Tensor(t) => {
            match t.rank() {
                0 => format!("{}", t.data()[0]),  // Scalar
                1 => /* Vector formatting */,
                2 => /* Matrix formatting con saltos de línea */,
                _ => format!("{}", t),  // 3D+ usa Display trait
            }
        }

        Value::Record(map) => {
            let mut fields: Vec<String> = map.iter()
                .map(|(k, v)| format!("{}: {}", k, format_value(v)))
                .collect();
            fields.sort();  // Orden alfabético para consistencia
            format!("{{ {} }}", fields.join(", "))
        }

        Value::Edge { from, to, directed, properties } => {
            let arrow = if *directed { "->" } else { "<>" };
            if properties.is_empty() {
                format!("{} {} {}", from, arrow, to)
            } else {
                let props: Vec<String> = properties.iter()
                    .map(|(k, v)| format!("{}: {}", k, format_value(v)))
                    .collect();
                format!("{} {} {}: {{ {} }}", from, arrow, to, props.join(", "))
            }
        }

        Value::Function(_) => "<function>".to_string(),
        Value::MutableRef(rc) => format_value(&rc.borrow()),

        // Valores internos que nunca deberían aparecer:
        Value::TailCall(_) => "<internal:tail-call>".to_string(),
        Value::EarlyReturn(_) => "<internal:early-return>".to_string(),
    }
}
```

**Características especiales:**

**Matrices con formato multi-línea:**
```javascript
ach[1]> [[1, 2], [3, 4]]
[[1, 2],
 [3, 4]]  // ← Más legible
```

**Records ordenados alfabéticamente:**
```javascript
ach[1]> { z: 3, a: 1, m: 2 }
{ a: 1, m: 2, z: 3 }  // ← Consistente
```

**Complejos con signo correcto:**
```javascript
ach[1]> 3 + 4i
3+4i

ach[2]> 3 - 4i
3-4i  // ← No "3+-4i"
```

#### `clear_screen()` - Cross-platform screen clearing

```rust
fn clear_screen() {
    // Platform-specific command
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

    // Fallback ANSI escape
    print!("\x1B[2J\x1B[1;1H");
}
```

**Estrategia de 3 capas:**
1. Comando nativo del OS (Windows: `cls`, Unix: `clear`)
2. Fallback ANSI escape sequence
3. Ignora errores (best-effort)

#### `print_help()` - Help message

```rust
fn print_help() {
    use nu_ansi_term::Color;

    println!("{}", Color::Green.bold().paint("Achronyme REPL Commands:"));
    println!("  {}        - Show this help", Color::Cyan.paint("help"));
    println!("  {}       - Clear screen and reset", Color::Cyan.paint("clear"));
    println!("  {}        - Clear screen only", Color::Cyan.paint("cls"));
    println!("  {}  - Exit the REPL", Color::Cyan.paint("exit, quit"));

    println!();
    println!("{}", Color::Green.bold().paint("Features:"));
    println!("  - Syntax highlighting (automatic)");
    println!("  - Command history (use ↑/↓ arrows)");
    println!("  - Tab completion for built-in functions");
    // ...

    println!();
    println!("{}", Color::Green.bold().paint("Examples:"));
    println!("  {}         - Basic arithmetic", Color::Yellow.paint("2 + 2"));
    println!("  {}  - Variable", Color::Yellow.paint("let x = 5"));
    // ...
}
```

Usa `nu-ansi-term` para colorear la ayuda.

### 2. `repl_helper.rs` - Integración con rustyline

**Responsabilidades:**
- Implementar traits de `rustyline` para features del REPL
- Autocompletado de funciones built-in
- Hints en línea (sugerencias)
- Syntax highlighting (delegado a `highlighter.rs`)

**Estructura principal:**

```rust
pub struct ReplHelper {
    pub functions: Vec<String>,  // Lista de funciones para autocomplete
}

impl Helper for ReplHelper {}  // Marker trait
```

#### `ReplHelper::new()` - Constructor

```rust
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
```

**Lista completa:**
- 7 funciones trigonométricas + inversas + hiperbólicas
- 5 funciones exponenciales/logarítmicas
- 8 funciones de redondeo y potencias
- 4 higher-order functions
- 4 funciones de cálculo numérico
- 9 funciones de álgebra lineal
- 9 funciones estadísticas
- 5 funciones de procesamiento de señales
- 5 funciones de utilidad
- 3 keywords

**Total: ~40 entradas**

#### `Completer` trait - Tab completion

```rust
impl Completer for ReplHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        // 1. Find start of current word
        let start = line[..pos]
            .rfind(|c: char| !c.is_alphanumeric() && c != '_')
            .map(|i| i + 1)
            .unwrap_or(0);

        let word = &line[start..pos];

        if word.is_empty() {
            return Ok((pos, Vec::new()));
        }

        // 2. Find all functions that start with this prefix
        let matches: Vec<String> = self.functions
            .iter()
            .filter(|f| f.starts_with(word))
            .cloned()
            .collect();

        Ok((start, matches))
    }
}
```

**Algoritmo:**
1. Encuentra el inicio de la palabra actual (retrocede hasta encontrar no-alfanumérico)
2. Extrae la palabra parcial
3. Filtra funciones que empiecen con esa palabra
4. Retorna lista de candidatos

**Ejemplos:**

```
Input: "si<TAB>"
→ start=0, word="si"
→ matches=["sin", "sinh"]

Input: "map(x => x^2, lin<TAB>"
→ start=15, word="lin"
→ matches=["linspace"]

Input: "di<TAB>"
→ start=0, word="di"
→ matches=["diff"]
```

#### `Hinter` trait - Inline hints

```rust
impl Hinter for ReplHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<Self::Hint> {
        // Solo sugerir al final de la línea
        if pos < line.len() {
            return None;
        }

        // Find start of current word
        let start_char = line[..pos]
            .char_indices()
            .rev()
            .find(|(_, c)| !c.is_alphanumeric() && *c != '_')
            .map(|(i, c)| i + c.len_utf8())
            .unwrap_or(0);

        let word = &line[start_char..];

        if word.is_empty() {
            return None;
        }

        // Find first match longer than current word
        self.functions
            .iter()
            .find(|f| f.starts_with(word) && f.len() > word.len())
            .map(|f| f[word.len()..].to_string())  // Solo la parte faltante
    }
}
```

**Diferencia con Completer:**
- **Completer** (TAB): Muestra todas las opciones
- **Hinter**: Muestra solo la primera opción, en gris, mientras escribes

**Ejemplo visual:**
```
ach[1]> si█
        ^^n      ← Hint en gris (sin presionar TAB)

ach[1]> si<TAB>
sin  sinh        ← Completer muestra todas las opciones
```

#### `Highlighter` trait - Syntax highlighting

```rust
impl Highlighter for ReplHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        Cow::Owned(highlight_code(line))
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _forced: bool) -> bool {
        // Solo highlight cuando forced (después de Enter o triggers)
        // Evita highlighting excesivo en cada carácter
        _forced
    }
}
```

Delega el highlighting real a `highlighter::highlight_code()`.

**Optimización:** `highlight_char()` retorna `_forced` para evitar re-highlighting en cada tecla presionada. Solo destaca cuando:
- Se presiona Enter
- Se completa una palabra
- Se dispara manualmente

#### `Validator` trait - Input validation

```rust
impl Validator for ReplHelper {}  // Default implementation
```

Usa la implementación por defecto (no valida, acepta todo). La validación real ocurre en `should_continue_reading()`.

### 3. `highlighter.rs` - Syntax highlighting

**Responsabilidades:**
- Colorear código SOC con colores ANSI
- Usar un lexer simple para identificar tokens
- Ser rápido (se ejecuta en cada línea del REPL)

#### `highlight_code()` - Main entry point

```rust
pub fn highlight_code(input: &str) -> String {
    highlight_partial(input)
}
```

Simple wrapper a `highlight_partial()`.

#### `highlight_partial()` - Lexer-based highlighter

**Algoritmo:**

```rust
fn highlight_partial(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            // Numbers
            '0'..='9' => {
                let mut num = String::from(ch);

                // Consume full number (including . e E i)
                while let Some(&next) = chars.peek() {
                    if next.is_ascii_digit() || next == '.' ||
                       next == 'e' || next == 'E' ||
                       next == '-' || next == '+' {
                        num.push(chars.next().unwrap());
                    } else if next == 'i' {
                        // Complex number
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
            '+' | '*' | '/' | '%' | '^' | '-' => {
                result.push_str(&Color::Red.bold().paint(ch.to_string()).to_string());
            }

            // Comparison/logical operators
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
                    _ if is_builtin_function(&ident) => {
                        Color::Green.bold().paint(&ident).to_string()
                    }
                    _ => Color::White.paint(&ident).to_string(),
                };
                result.push_str(&colored);
            }

            _ => result.push(ch),  // Other chars (whitespace, etc.)
        }
    }

    result
}
```

**Estados del lexer:**

1. **Números** - Detecta:
   - Enteros: `123`
   - Decimales: `3.14`
   - Científicos: `1e-5`, `2.5E+10`
   - Complejos: `3+4i`, `2i`

2. **Operadores** - Detecta:
   - Aritméticos: `+`, `-`, `*`, `/`, `%`, `^`
   - Comparación: `==`, `!=`, `<`, `>`, `<=`, `>=`
   - Lógicos: `&&`, `||`
   - Asignación: `=`, `=>`

3. **Brackets** - `()`, `[]`, `{}`

4. **Identificadores** - Palabras alfanuméricas + `_`

5. **Keywords** - `let`, `true`, `false`

6. **Built-in functions** - Ver `is_builtin_function()`

**Esquema de colores:**

| Token | Color | Bold | Ejemplo |
|-------|-------|------|---------|
| Keywords (`let`) | Purple | ✅ | `let` |
| Booleans (`true`, `false`) | Yellow | ❌ | `true` |
| Built-in functions | Green | ✅ | `sin` |
| Numbers | Cyan | ❌ | `3.14` |
| Complex numbers | Light Cyan | ❌ | `3+4i` |
| Operators | Red | ✅ | `+` |
| Brackets | Light Blue | ✅ | `[` |
| Variables | White | ❌ | `x` |

#### `is_builtin_function()` - Function detection

```rust
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
```

Usa `matches!` macro para matching eficiente. Debe mantenerse sincronizado con `ReplHelper::functions`.

### 4. `lib.rs` - Public API

```rust
#[cfg(feature = "cli")]
pub mod highlighter;
#[cfg(feature = "cli")]
pub mod repl_helper;
```

**Propósito:**
- Re-exportar módulos públicos
- Permitir usar el CLI como library (si es necesario)
- Usar feature flags para dependencias opcionales

**Feature flags:**
```toml
[features]
default = ["cli"]
cli = ["dep:rustyline", "dep:nu-ansi-term", "dep:pest", "dep:dirs"]
```

Si compilas sin `--features cli`, los módulos no se incluyen (útil para builds mínimos).

## 🔄 Flujos de ejecución

### Flujo 1: REPL - Expresión simple

```
Usuario: "2 + 2"
    ↓
run_repl() → rl.readline("ach[1]> ")
    ↓
ReplHelper::highlight() → "2 + 2" coloreado
    ↓
should_continue_reading("2 + 2")
    ├─> has_balanced_delimiters() → true (balanceado)
    └─> parse("2 + 2") → Ok(_) → false (completo)
    ↓
evaluate_expression()
    ├─> evaluator.eval_str("2 + 2")
    │       ↓ (en achronyme-eval)
    │   parse("2 + 2") → BinaryOp { Add, Number(2), Number(2) }
    │       ↓
    │   evaluate(BinaryOp) → Value::Number(4.0)
    └─> format_value(Number(4.0)) → "4"
    ↓
println!("4")
```

### Flujo 2: REPL - Multi-line input

```
Usuario: "let f = x => do {"
    ↓
run_repl() → rl.readline("ach[1]> ")
    ↓
input_buffer = "let f = x => do {"
    ↓
should_continue_reading("let f = x => do {")
    ├─> has_balanced_delimiters() → false ('{' sin cerrar)
    └─> return true (incompleto)
    ↓
continue (no evaluar, esperar más input)
    ↓
rl.readline("     ...> ")  ← Prompt de continuación
    ↓
Usuario: "    x * x"
    ↓
input_buffer = "let f = x => do {\n    x * x"
    ↓
should_continue_reading(...)
    ├─> has_balanced_delimiters() → false ('{' sin cerrar)
    └─> return true (incompleto)
    ↓
continue
    ↓
Usuario: "}"
    ↓
input_buffer = "let f = x => do {\n    x * x\n}"
    ↓
should_continue_reading(...)
    ├─> has_balanced_delimiters() → true (balanceado)
    └─> parse(...) → Ok(_) → false (completo)
    ↓
evaluate_expression(input_buffer)
    ↓
println!("<function>")
```

### Flujo 3: File execution

```
$ achronyme script.soc
    ↓
main() → args.len() == 2 && args[1].ends_with(".soc")
    ↓
run_file("script.soc")
    ├─> fs::read_to_string("script.soc")
    │       ↓
    │   "let x = 5\nlet y = 10\nx + y"
    ├─> evaluator = Evaluator::new()
    ├─> evaluator.set_current_file_dir("script.soc")
    └─> evaluate_expression(&mut evaluator, "let x = 5\nlet y = 10\nx + y")
            ↓
        evaluator.eval_str(...)
            ↓ (en achronyme-eval)
        parse("let x = 5\nlet y = 10\nx + y")
            → [VariableDecl, VariableDecl, VariableRef]
            ↓
        evaluate(VariableDecl("x", 5))
        evaluate(VariableDecl("y", 10))
        evaluate(VariableRef("x") + VariableRef("y"))
            → Value::Number(15.0)
            ↓
        format_value(Number(15.0)) → "15"
    ↓
println!("15")
```

### Flujo 4: Tab completion

```
Usuario: "di<TAB>"
    ↓
ReplHelper::complete(line="di", pos=2)
    ↓
start = 0 (no hay no-alfanuméricos antes)
word = "di"
    ↓
matches = functions.filter(|f| f.starts_with("di"))
        → ["diff"]
    ↓
rustyline muestra: "diff"
    ↓
Usuario presiona TAB → autocompleta a "diff"
```

### Flujo 5: Syntax highlighting

```
Usuario escribe: "let x = sin(pi)"
    ↓
ReplHelper::highlight(line="let x = sin(pi)", pos=15)
    ↓
highlight_code("let x = sin(pi)")
    ↓
highlight_partial(...)
    ├─> Tokenize:
    │   1. "let" → identifier → keyword → Purple bold
    │   2. " " → whitespace → sin color
    │   3. "x" → identifier → variable → White
    │   4. " = " → operator → Red bold
    │   5. "sin" → identifier → builtin → Green bold
    │   6. "(" → bracket → Light Blue bold
    │   7. "pi" → identifier → variable → White
    │   8. ")" → bracket → Light Blue bold
    └─> result = "\x1b[35mlet\x1b[0m x \x1b[31m=\x1b[0m \x1b[32msin\x1b[0m\x1b[34m(\x1b[0mpi\x1b[34m)\x1b[0m"
    ↓
rustyline muestra la línea coloreada
```

## 🔧 Patrones de diseño utilizados

### 1. Strategy Pattern - Execution modes

Los tres modos de ejecución (REPL, File, Expression) implementan la misma interfaz:

```rust
trait ExecutionMode {
    fn execute(&mut self);
}

// Implementaciones:
struct ReplMode { /* ... */ }
struct FileMode { filename: String }
struct ExpressionMode { expr: String }
```

Aunque no está explícitamente definido como trait, el patrón está implícito en `main()`.

### 2. Facade Pattern - Evaluator wrapper

`evaluate_expression()` actúa como facade para:
- `achronyme-parser::parse()`
- `Evaluator::eval_str()`
- `format_value()`

Simplifica la interfaz para los consumidores.

### 3. Builder Pattern - rustyline Config

```rust
let config = Config::builder()
    .auto_add_history(true)
    .build();
```

rustyline usa Builder pattern para configuración.

### 4. Trait-based polymorphism - rustyline integration

```rust
impl Helper for ReplHelper {}
impl Completer for ReplHelper { /* ... */ }
impl Hinter for ReplHelper { /* ... */ }
impl Highlighter for ReplHelper { /* ... */ }
impl Validator for ReplHelper {}
```

Cada trait agrega una funcionalidad al REPL.

### 5. State Machine - Multi-line input

El REPL es una state machine implícita:

```
Estado 1: Esperando input
    ↓ (usuario escribe línea)
Estado 2: Verificando completitud
    ├─> Incompleto → Estado 1 (prompt "...>")
    └─> Completo → Estado 3
Estado 3: Evaluando
    ↓
Estado 4: Mostrando resultado
    ↓
Estado 1 (loop)
```

## 🧪 Testing strategies

### Unit testing

Cada módulo se testea independientemente:

```rust
// highlighter.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_numbers() {
        let result = highlight_code("123");
        assert!(result.contains("123"));
        assert!(result.contains("\x1b[36m"));  // Cyan color
    }

    #[test]
    fn test_highlight_keywords() {
        let result = highlight_code("let");
        assert!(result.contains("\x1b[35m"));  // Purple color
    }
}
```

### Integration testing

Probar flujos completos:

```rust
#[test]
fn test_evaluate_expression() {
    let mut evaluator = Evaluator::new();
    let result = evaluate_expression(&mut evaluator, "2 + 2").unwrap();
    assert_eq!(result, "4");
}

#[test]
fn test_multi_line_detection() {
    assert!(should_continue_reading("let x = [1, 2,"));
    assert!(!should_continue_reading("let x = [1, 2]"));
}
```

### Manual testing

```bash
# REPL testing
cargo run --package achronyme-cli

# File testing
echo "2 + 2" > test.soc
cargo run --package achronyme-cli -- test.soc

# Expression testing
cargo run --package achronyme-cli -- "2 + 2"
```

## 🔍 Debugging tips

### 1. Habilitar logging de rustyline

```rust
// En main.rs, antes de run_repl()
env::set_var("RUST_LOG", "rustyline=debug");
env_logger::init();
```

### 2. Inspeccionar AST parseado

```rust
// En evaluate_expression(), antes de evaluar:
match achronyme_parser::parse(input) {
    Ok(ast) => {
        eprintln!("DEBUG: AST = {:?}", ast);  // ← Debug output
        // ... continuar con evaluación
    }
}
```

### 3. Verificar highlighting sin ANSI codes

```rust
// En highlighter.rs
pub fn highlight_code_debug(input: &str) -> String {
    let highlighted = highlight_code(input);
    // Strip ANSI codes para ver estructura
    strip_ansi_codes(&highlighted)
}
```

### 4. Test multi-line detection manualmente

```rust
// En main.rs, agregar función de debug:
#[cfg(debug_assertions)]
fn debug_should_continue(input: &str) {
    eprintln!("Input: {:?}", input);
    eprintln!("Balanced: {}", has_balanced_delimiters(input));
    eprintln!("Should continue: {}", should_continue_reading(input));
}
```

## 📚 Referencias y recursos

### Rustyline documentation
- [Rustyline GitHub](https://github.com/kkawakam/rustyline)
- [API docs](https://docs.rs/rustyline/)
- [Examples](https://github.com/kkawakam/rustyline/tree/master/examples)

### ANSI color codes
- [nu-ansi-term](https://docs.rs/nu-ansi-term/)
- [ANSI escape codes reference](https://en.wikipedia.org/wiki/ANSI_escape_code)

### Related crates
- [achronyme-eval](../achronyme-eval/src/README.md) - Motor de evaluación
- [achronyme-parser](../achronyme-parser/src/README.md) - Parser y AST
- [achronyme-types](../achronyme-types/src/README.md) - Definiciones de tipos

## 🎯 Posibles mejoras

### 1. Mejor manejo de errores en multi-line

Actualmente, si hay un error de sintaxis en multi-line, se muestra solo al final. Podría detectarse antes.

### 2. Syntax highlighting más sofisticado

- Colorear strings
- Colorear comentarios
- Colorear tipos (Number, Boolean, etc.)

### 3. Completion context-aware

Autocompletar basado en contexto:
```javascript
let x = [1, 2, 3]
x.m<TAB>  // → sugerir métodos de array (map, etc.)
```

### 4. History search mejorado

- Filtrar historial por patrón
- Historial persistente entre versiones
- Compartir historial entre sesiones

### 5. Multi-line editing

Permitir editar líneas previas en multi-line input (como Python REPL).

### 6. Breakpoints y debugging

```javascript
debug(x => {
    let y = x * 2
    breakpoint()  // ← Pausar aquí
    y + 1
})
```

### 7. REPL commands

```
ach[1]> :help        # Comando REPL (con ':')
ach[2]> :vars        # Listar todas las variables
ach[3]> :type x      # Mostrar tipo de x
ach[4]> :clear x     # Eliminar variable x
```

### 8. Output paging

Para outputs largos (como matrices grandes), usar pager automático.

### 9. Export/Import sessions

```
ach[1]> :save session.soc     # Guardar todo el historial
ach[2]> :load session.soc     # Cargar sesión previa
```

### 10. Performance profiling

```javascript
ach[1]> :profile
ach[2]> map(x => x^2, range(1, 1000000))
Executed in 125ms
```
