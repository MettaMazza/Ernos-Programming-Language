#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompOp {
    LessThan,
    GreaterThan,
    Equals,
    NotEquals,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Integer(i64),
    StringLiteral(String),
    Identifier(String),
    Binary(Box<Expr>, Op, Box<Expr>),
    Comparison(Box<Expr>, CompOp, Box<Expr>),
    Logical(Box<Expr>, LogicalOp, Box<Expr>),
    Call(String, Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Set(String, Expr),
    If(Expr, Vec<Stmt>, Option<Vec<Stmt>>),
    RepeatWhile(Expr, Vec<Stmt>),
    Return(Expr),
    Display(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub imports: Vec<String>,
    pub functions: Vec<Function>,
}
