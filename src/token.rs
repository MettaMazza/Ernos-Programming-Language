#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub line: usize,
    pub col: usize,
}

impl Span {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Define,
    With,
    And,
    Set,
    To,
    If,
    Else,
    Return,
    Display,
    Repeat,
    While,

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,

    // Comparisons
    LessThan,
    GreaterThan,
    Equals,
    NotEquals,
    LogicalAnd,
    LogicalOr,

    // Literals & Identifiers
    Identifier(String),
    Integer(i64),
    StringLiteral(String),

    // Punctuation
    Colon,
    LeftParen,
    RightParen,

    // Indentation & Layout
    Indent,
    Dedent,
    Newline,

    // End of file
    EOF,
}
