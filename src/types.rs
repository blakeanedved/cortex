#[derive(Debug)]
pub enum Expression {
    Int(i32),
    Add { lhs: Box<Expression>, rhs: Box<Expression> },
    Sub { lhs: Box<Expression>, rhs: Box<Expression> },
    Mul { lhs: Box<Expression>, rhs: Box<Expression> },
    Div { lhs: Box<Expression>, rhs: Box<Expression> },
}
