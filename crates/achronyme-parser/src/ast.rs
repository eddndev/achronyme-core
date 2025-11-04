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
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Number(f64),
    BinaryOp {
        op: BinaryOp,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    UnaryOp {
        op: UnaryOp,
        operand: Box<AstNode>,
    },
    FunctionCall {
        name: String,
        args: Vec<AstNode>,
    },
    ComplexLiteral {
        re: f64,
        im: f64,
    },
    VectorLiteral(Vec<AstNode>),
    MatrixLiteral(Vec<Vec<AstNode>>),
    VariableDecl {
        name: String,
        initializer: Box<AstNode>,
    },
    VariableRef(String),
    Lambda {
        params: Vec<String>,
        body: Box<AstNode>,
    },
}
