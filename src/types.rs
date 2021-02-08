#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Equals,
    Not,
    Factorial,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanEquals,
    GreaterThanEquals,
    Power,
    Abs,
    Or,
    And,
    SetAdd,
    SetSubtract,
}

#[derive(Clone)]
pub enum ASTType {
    // StringLit(String),
    // FloatLit(String),
    Number(f64),
    Boolean(bool),
    Ident(String),
    Op(Operator),
    List(Vec<ASTNode>),
    Fun {
        name: String,
        args: Vec<String>,
        body: Option<Box<ASTNode>>,
    },
}

#[derive(Clone)]
pub struct ASTNode {
    pub kind: ASTType,
    pub children: Vec<ASTNode>,
}