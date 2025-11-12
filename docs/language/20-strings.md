# String Manipulation in Achronyme

Strings in Achronyme are a fundamental data type for text processing. This guide covers string literals, functions, operations, and advanced features like indexing and slicing.

## String Literals

### Basic String Syntax

String literals are enclosed in double quotes:

```achronyme
let greeting = "Hello"
let name = "Achronyme"
let empty = ""
```

### Escape Sequences

Achronyme supports the following escape sequences within string literals:

| Escape Sequence | Result | Description |
|---|---|---|
| \n | Newline | Line break |
| \t | Tab | Horizontal tab |
| \r | Carriage Return | Carriage return character |
| \ | Backslash | Literal backslash character |
| \" | Double Quote | Literal double quote character |

### Unicode Support

Strings fully support Unicode characters, including emoji and international characters.

## String Functions

### concat(s1, s2)

Concatenates two strings.

**Signature:** concat(String, String) -> String

**Examples:**
```achronyme
concat("Hello", " World")
concat("Achronyme", " Language")

let greeting = "Hello"
let name = "Alice"
concat(greeting, concat(" ", name))
```

**Implementation:** crates/achronyme-eval/src/function_modules/strings.rs

### length(s)

Returns the length of a string in characters.

**Signature:** length(String) -> Number

**Examples:**
```achronyme
length("hello")
length("testing")
length("") 
```

**Implementation:** crates/achronyme-eval/src/function_modules/strings.rs

## String Operations

### String Equality

Strings support equality and inequality comparison operators.

**Operators:** ==, !=

**Examples:**
```achronyme
"hello" == "hello"
"hello" != "world"
```

**Case Sensitivity:** String comparison is case-sensitive.

## String Indexing and Slicing

Strings support both single-character indexing and range-based slicing.

### Single Character Indexing

**Syntax:** string[index]

Returns a single character as a string.

**Examples:**
```achronyme
let word = "hello"
word[0]
word[-1]
```

### String Slicing

**Syntax:** string[start..end], string[start..], string[..end], string[..]

Returns a substring as a string.

**Examples:**
```achronyme
let message = "Hello, World!"
message[0..5]
message[7..]
message[..5]
message[-6..]
```

**Implementation:** index_string() in crates/achronyme-eval/src/handlers/indexing.rs (lines 127-147)

## Limitations

### No String Modification Functions

Strings are immutable. There is no built-in split(), substring(), replace(), or trim() function.

### No Case Conversion

There are no toUpperCase() or toLowerCase() functions.

### No Direct String Iteration

Higher-order functions (map, filter, reduce) do not work with strings directly.

### No Regex Support

Pattern matching and regular expressions are not supported.

### No Printf-style Formatting

There is no formatted string interpolation. Use concat() to build strings manually.

## Implementation Details

### String Storage

- Type: Value::String(String) in the value enum
- Storage: UTF-8 encoded Rust String
- Memory: Heap-allocated, supports arbitrary length

### Built-in Functions

| Function | Arity | Location |
|---|---|---|
| concat | 2 | crates/achronyme-eval/src/function_modules/strings.rs |
| length | 1 | crates/achronyme-eval/src/function_modules/strings.rs |

### String Parsing

String literals are parsed by the Pest PEG parser:

- Grammar Rule: string_literal in crates/achronyme-parser/src/grammar.pest (lines 49-56)
- Character Matching: string_char rule supports escape sequences
- Processing: process_escape_sequences() in crates/achronyme-parser/src/pest_parser.rs (lines 416-445)

### Comparison

String equality and inequality are handled in:

- Functions: apply_eq(), apply_neq() in crates/achronyme-eval/src/handlers/binary_ops.rs (lines 751-765)

## Related Documentation

- Data Types (03-data-types.md)
- Operators (04-operators.md)
- Indexing and Slicing (10-indexing-slicing.md)
- Higher-Order Functions (11-higher-order-functions.md)
- Functions (06-functions.md)

## See Also

- Example file: examples/soc/28-strings-demo.soc
- Test file: crates/achronyme-eval/tests/test_strings.rs
