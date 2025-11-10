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
    ComplexLiteral {
        re: f64,
        im: f64,
    },
    ArrayLiteral(Vec<AstNode>), // Unified array syntax: [1,2,3], [[1,2],[3,4]], [[[...]]], etc.
    RecordLiteral(Vec<(String, AstNode)>), // Vec of (key, value) pairs
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
}
