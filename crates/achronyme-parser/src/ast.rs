#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Modulo,

    // Comparison
    Gt,  // >
    Lt,  // <
    Gte, // >=
    Lte, // <=
    Eq,  // ==
    Neq, // !=

    // Logical
    And, // &&
    Or,  // ||
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate, // -x
    Not,    // !x
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Number(f64),
    Boolean(bool),
    StringLiteral(String),
    BinaryOp {
        op: BinaryOp,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    UnaryOp {
        op: UnaryOp,
        operand: Box<AstNode>,
    },
    If {
        condition: Box<AstNode>,
        then_expr: Box<AstNode>,
        else_expr: Box<AstNode>,
    },
    Piecewise {
        cases: Vec<(Box<AstNode>, Box<AstNode>)>,
        default: Option<Box<AstNode>>,
    },
    FunctionCall {
        name: String,
        args: Vec<AstNode>,
    },
    CallExpression {
        callee: Box<AstNode>,  // Expression that evaluates to a function (for IIFE)
        args: Vec<AstNode>,
    },
    ComplexLiteral {
        re: f64,
        im: f64,
    },
    ArrayLiteral(Vec<ArrayElement>), // Array elements can be single values or spread expressions
    RecordLiteral(Vec<RecordFieldOrSpread>), // Record fields or spread expressions
    FieldAccess {
        record: Box<AstNode>,
        field: String,
    },
    VariableDecl {
        name: String,
        initializer: Box<AstNode>,
    },
    VariableRef(String),
    SelfReference, // 'self' keyword for use in records
    RecReference,  // 'rec' keyword for recursive function calls
    Lambda {
        params: Vec<String>,
        body: Box<AstNode>,
    },
    Edge {
        from: String,
        to: String,
        directed: bool,
        metadata: Option<Box<AstNode>>,
    },
    // Indexing and slicing
    IndexAccess {
        object: Box<AstNode>,
        indices: Vec<IndexArg>,
    },
    // Sequence: multiple statements separated by semicolons
    // Example: let a = 1; let b = 2; a + b
    // The last statement is the value of the sequence
    Sequence {
        statements: Vec<AstNode>,
    },
    // DoBlock: do { statements }
    // Similar to Sequence but explicitly wrapped in a do { } block
    // Used in lambda bodies: x => do { let a = x * 2; a + 10 }
    DoBlock {
        statements: Vec<AstNode>,
    },
}

/// Represents an array element - can be a single expression or a spread expression
#[derive(Debug, Clone, PartialEq)]
pub enum ArrayElement {
    Single(AstNode),        // Regular element: expr
    Spread(Box<AstNode>),   // Spread element: ...expr
}

/// Represents a record field or spread expression
#[derive(Debug, Clone, PartialEq)]
pub enum RecordFieldOrSpread {
    Field { name: String, value: AstNode },  // Regular field: key: value
    Spread(Box<AstNode>),                     // Spread: ...expr
}

/// Represents an indexing argument - can be a single expression or a range
#[derive(Debug, Clone, PartialEq)]
pub enum IndexArg {
    Single(Box<AstNode>),  // Single index: tensor[5]
    Range {                // Range slice: tensor[1..5], tensor[..5], tensor[1..], tensor[..]
        start: Option<Box<AstNode>>,
        end: Option<Box<AstNode>>,
    },
}
