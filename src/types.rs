#[derive(Debug)]
pub struct Locatable<T> {
    pub data: T,
    pub span: Span,
}

#[derive(Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize
}

impl<T> Locatable<T> {
    pub fn new(data: T, span: (usize, usize)) -> Locatable<T> {
        Locatable {
            data,
            span: Span {
                start: span.0,
                end: span.1,
            }
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Int(i32),
    Float(f32),
    Ident(String),
    Tuple(Vec<LocExpr>),
    List(Vec<LocExpr>),
    ListGenerator {
       iterators: Vec<LocExpr>,
       qualifications: Vec<Locatable<Condition>>
    },
    Iterator {
        assignable: Locatable<Assignable>,
        value: Box<LocExpr>,
    },
    Add { lhs: Box<LocExpr>, rhs: Box<LocExpr> },
    Sub { lhs: Box<LocExpr>, rhs: Box<LocExpr> },
    Mul { lhs: Box<LocExpr>, rhs: Box<LocExpr> },
    Div { lhs: Box<LocExpr>, rhs: Box<LocExpr> },
    FunctionCall { ident: String, args: Vec<LocExpr> },
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ConditionType {
    Greater, Lesser, Equal, GreaterEqual, LesserEqual, NotEqual
}

#[derive(Debug)]
pub struct Condition {
    pub cond_type: ConditionType,
    pub lhs: Box<LocExpr>,
    pub rhs: Box<LocExpr>,
}

#[derive(Debug)]
pub enum Statement {
    FunctionDefinition { 
        ident: String,
        args: Vec<String>,
        body: LocExpr,
    },
    Assignment {
        assignable: Locatable<Assignable>,
        expr: LocExpr
    }
}

#[derive(Debug)]
pub enum Assignable {
    Single(String),
    Multi(Vec<Locatable<Assignable>>)
}

pub type LocExpr = Locatable<Expression>;
