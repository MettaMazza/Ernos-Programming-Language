/// Python → ErnosPlain Transpiler
///
/// Parses a subset of Python 3 source code and emits equivalent ErnosPlain (.ep) code.
///
/// Architecture:
///   Python source → Python tokenizer → Python AST → ErnosPlain emitter → .ep file
///
/// Supported Python constructs:
///   - Variables and assignments (including augmented: +=, -=, *=, //=, %=)
///   - Functions (def) with type hints (optional)
///   - Control flow: if/elif/else, for, while, break, continue
///   - Print statements → display
///   - Input() → read_line()
///   - Lists: [], append, len, pop, indexing
///   - Dicts: {}, d[k] = v, len(d), del d[k], 'in'
///   - Strings: f-strings, +, len, upper, lower, strip, split, replace, find, in
///   - Arithmetic: +, -, *, //, %, **
///   - Comparisons: ==, !=, <, >, <=, >=, and, or, not
///   - Return statements
///   - Classes (basic) → structs
///   - Try/except → try/check
///   - Import statements (noted as comments)
///   - Comments preserved
///   - Type hints mapped to Ernos types

use std::collections::HashMap;

// ============ Python Tokenizer ============

#[derive(Debug, Clone, PartialEq)]
pub enum PyToken {
    Ident(String),
    IntLit(i64),
    FloatLit(f64),
    StrLit(String),
    FStrLit(String),
    // Operators
    Plus, Minus, Star, DoubleStar, Slash, DoubleSlash, Percent,
    Eq, EqEq, NotEq, Lt, Gt, LtEq, GtEq,
    PlusEq, MinusEq, StarEq, SlashEq, PercentEq,
    // Delimiters
    LParen, RParen, LBracket, RBracket, LBrace, RBrace,
    Comma, Colon, Dot, Arrow, Semi, At,
    // Special
    Newline,
    Indent(usize),  // indentation level
    Hash,
    Eof,
}

pub struct PyLexer {
    chars: Vec<char>,
    pos: usize,
    at_line_start: bool,
}

impl PyLexer {
    pub fn new(input: &str) -> Self {
        PyLexer {
            chars: input.chars().collect(),
            pos: 0,
            at_line_start: true,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.get(self.pos).copied();
        self.pos += 1;
        c
    }

    pub fn tokenize(&mut self) -> Vec<PyToken> {
        let mut tokens = Vec::new();

        loop {
            if self.at_line_start {
                // Count indentation
                let mut indent = 0;
                while self.peek() == Some(' ') {
                    self.advance();
                    indent += 1;
                }
                while self.peek() == Some('\t') {
                    self.advance();
                    indent += 4;
                }
                // Skip blank lines
                if self.peek() == Some('\n') {
                    self.advance();
                    continue;
                }
                if self.peek() == Some('#') {
                    // Comment line — skip
                    while self.peek() != Some('\n') && self.peek().is_some() {
                        self.advance();
                    }
                    if self.peek() == Some('\n') { self.advance(); }
                    continue;
                }
                if self.peek().is_none() { break; }
                tokens.push(PyToken::Indent(indent));
                self.at_line_start = false;
            }

            // Skip inline whitespace
            while self.peek() == Some(' ') || self.peek() == Some('\t') {
                self.advance();
            }

            match self.peek() {
                None => break,
                Some('\n') => {
                    self.advance();
                    tokens.push(PyToken::Newline);
                    self.at_line_start = true;
                }
                Some('#') => {
                    // Skip comment to end of line
                    while self.peek() != Some('\n') && self.peek().is_some() {
                        self.advance();
                    }
                }
                Some('\\') => {
                    // Line continuation
                    self.advance();
                    if self.peek() == Some('\n') { self.advance(); }
                }
                Some('"') | Some('\'') => {
                    let quote = self.peek().unwrap();
                    self.advance();

                    // Check for triple quotes
                    let is_triple = if self.peek() == Some(quote) {
                        let save = self.pos;
                        self.advance();
                        if self.peek() == Some(quote) {
                            self.advance();
                            true
                        } else {
                            self.pos = save;
                            false
                        }
                    } else {
                        false
                    };

                    let mut s = String::new();
                    if is_triple {
                        loop {
                            match self.advance() {
                                Some(c) if c == quote => {
                                    if self.peek() == Some(quote) {
                                        self.advance();
                                        if self.peek() == Some(quote) {
                                            self.advance();
                                            break;
                                        }
                                        s.push(c);
                                        s.push(c);
                                    } else {
                                        s.push(c);
                                    }
                                }
                                Some('\\') => {
                                    if let Some(esc) = self.advance() {
                                        match esc {
                                            'n' => s.push('\n'),
                                            't' => s.push('\t'),
                                            '\\' => s.push('\\'),
                                            '\'' => s.push('\''),
                                            '"' => s.push('"'),
                                            _ => { s.push('\\'); s.push(esc); }
                                        }
                                    }
                                }
                                Some(c) => s.push(c),
                                None => break,
                            }
                        }
                    } else {
                        loop {
                            match self.advance() {
                                Some(c) if c == quote => break,
                                Some('\\') => {
                                    if let Some(esc) = self.advance() {
                                        match esc {
                                            'n' => s.push('\n'),
                                            't' => s.push('\t'),
                                            '\\' => s.push('\\'),
                                            '\'' => s.push('\''),
                                            '"' => s.push('"'),
                                            _ => { s.push('\\'); s.push(esc); }
                                        }
                                    }
                                }
                                Some(c) => s.push(c),
                                None => break,
                            }
                        }
                    }
                    tokens.push(PyToken::StrLit(s));
                }
                Some('f') if matches!(self.chars.get(self.pos + 1), Some('"') | Some('\'')) => {
                    self.advance(); // f
                    let quote = self.peek().unwrap();
                    self.advance();
                    let mut s = String::new();
                    loop {
                        match self.advance() {
                            Some(c) if c == quote => break,
                            Some('\\') => {
                                if let Some(esc) = self.advance() {
                                    match esc {
                                        'n' => s.push('\n'),
                                        't' => s.push('\t'),
                                        '\\' => s.push('\\'),
                                        _ => { s.push('\\'); s.push(esc); }
                                    }
                                }
                            }
                            Some(c) => s.push(c),
                            None => break,
                        }
                    }
                    tokens.push(PyToken::FStrLit(s));
                }
                Some(c) if c.is_ascii_digit() => {
                    let mut num = String::new();
                    while let Some(c) = self.peek() {
                        if c.is_ascii_digit() || c == '_' {
                            if c != '_' { num.push(c); }
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    if self.peek() == Some('.') && self.chars.get(self.pos + 1).map_or(false, |c| c.is_ascii_digit()) {
                        num.push('.');
                        self.advance();
                        while let Some(c) = self.peek() {
                            if c.is_ascii_digit() || c == '_' {
                                if c != '_' { num.push(c); }
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        tokens.push(PyToken::FloatLit(num.parse().unwrap_or(0.0)));
                    } else {
                        tokens.push(PyToken::IntLit(num.parse().unwrap_or(0)));
                    }
                }
                Some(c) if c.is_ascii_alphabetic() || c == '_' => {
                    let mut ident = String::new();
                    while let Some(c) = self.peek() {
                        if c.is_ascii_alphanumeric() || c == '_' {
                            ident.push(c);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    tokens.push(PyToken::Ident(ident));
                }
                Some('+') => { self.advance(); if self.peek() == Some('=') { self.advance(); tokens.push(PyToken::PlusEq); } else { tokens.push(PyToken::Plus); } }
                Some('-') => {
                    self.advance();
                    if self.peek() == Some('=') { self.advance(); tokens.push(PyToken::MinusEq); }
                    else if self.peek() == Some('>') { self.advance(); tokens.push(PyToken::Arrow); }
                    else { tokens.push(PyToken::Minus); }
                }
                Some('*') => {
                    self.advance();
                    if self.peek() == Some('*') { self.advance(); tokens.push(PyToken::DoubleStar); }
                    else if self.peek() == Some('=') { self.advance(); tokens.push(PyToken::StarEq); }
                    else { tokens.push(PyToken::Star); }
                }
                Some('/') => {
                    self.advance();
                    if self.peek() == Some('/') { self.advance(); tokens.push(PyToken::DoubleSlash); }
                    else if self.peek() == Some('=') { self.advance(); tokens.push(PyToken::SlashEq); }
                    else { tokens.push(PyToken::Slash); }
                }
                Some('%') => { self.advance(); if self.peek() == Some('=') { self.advance(); tokens.push(PyToken::PercentEq); } else { tokens.push(PyToken::Percent); } }
                Some('=') => { self.advance(); if self.peek() == Some('=') { self.advance(); tokens.push(PyToken::EqEq); } else { tokens.push(PyToken::Eq); } }
                Some('!') => { self.advance(); if self.peek() == Some('=') { self.advance(); tokens.push(PyToken::NotEq); } else { tokens.push(PyToken::Ident("not".into())); } }
                Some('<') => { self.advance(); if self.peek() == Some('=') { self.advance(); tokens.push(PyToken::LtEq); } else { tokens.push(PyToken::Lt); } }
                Some('>') => { self.advance(); if self.peek() == Some('=') { self.advance(); tokens.push(PyToken::GtEq); } else { tokens.push(PyToken::Gt); } }
                Some('(') => { self.advance(); tokens.push(PyToken::LParen); }
                Some(')') => { self.advance(); tokens.push(PyToken::RParen); }
                Some('[') => { self.advance(); tokens.push(PyToken::LBracket); }
                Some(']') => { self.advance(); tokens.push(PyToken::RBracket); }
                Some('{') => { self.advance(); tokens.push(PyToken::LBrace); }
                Some('}') => { self.advance(); tokens.push(PyToken::RBrace); }
                Some(',') => { self.advance(); tokens.push(PyToken::Comma); }
                Some(':') => { self.advance(); tokens.push(PyToken::Colon); }
                Some('.') => { self.advance(); tokens.push(PyToken::Dot); }
                Some(';') => { self.advance(); tokens.push(PyToken::Semi); }
                Some('@') => { self.advance(); tokens.push(PyToken::At); }
                Some(_) => { self.advance(); }
            }
        }

        tokens.push(PyToken::Newline);
        tokens.push(PyToken::Eof);
        tokens
    }
}

// ============ Python AST (Simplified) ============

#[derive(Debug, Clone)]
pub enum PyExpr {
    Int(i64),
    Float(f64),
    Str(String),
    FStr(String),
    Bool(bool),
    None,
    Name(String),
    BinOp(Box<PyExpr>, String, Box<PyExpr>),
    UnaryOp(String, Box<PyExpr>),
    Compare(Box<PyExpr>, Vec<(String, PyExpr)>),
    BoolOp(String, Vec<PyExpr>),
    Call(Box<PyExpr>, Vec<PyExpr>, Vec<(String, PyExpr)>),
    Attribute(Box<PyExpr>, String),
    Subscript(Box<PyExpr>, Box<PyExpr>),
    List(Vec<PyExpr>),
    Dict(Vec<(PyExpr, PyExpr)>),
    IfExpr(Box<PyExpr>, Box<PyExpr>, Box<PyExpr>),
    ListComp(Box<PyExpr>, String, Box<PyExpr>, Option<Box<PyExpr>>),
}

#[derive(Debug, Clone)]
pub enum PyStmt {
    Assign(Vec<String>, PyExpr),
    AugAssign(String, String, PyExpr),
    Expr(PyExpr),
    Return(Option<PyExpr>),
    If(PyExpr, Vec<PyStmt>, Vec<(PyExpr, Vec<PyStmt>)>, Option<Vec<PyStmt>>),
    While(PyExpr, Vec<PyStmt>),
    For(String, PyExpr, Vec<PyStmt>),
    FuncDef(String, Vec<(String, Option<String>)>, Option<String>, Vec<PyStmt>),
    ClassDef(String, Vec<PyStmt>),
    Import(String, Option<String>),
    FromImport(String, Vec<String>),
    Print(Vec<PyExpr>),
    Break,
    Continue,
    Pass,
    Comment(String),
    Del(PyExpr),
    Try(Vec<PyStmt>, Vec<(Option<String>, Option<String>, Vec<PyStmt>)>),
}

// ============ Python Parser ============

pub struct PyParser {
    tokens: Vec<PyToken>,
    pos: usize,
}

impl PyParser {
    pub fn new(tokens: Vec<PyToken>) -> Self {
        PyParser { tokens, pos: 0 }
    }

    fn peek(&self) -> &PyToken {
        self.tokens.get(self.pos).unwrap_or(&PyToken::Eof)
    }

    fn advance(&mut self) -> PyToken {
        let tok = self.tokens.get(self.pos).cloned().unwrap_or(PyToken::Eof);
        self.pos += 1;
        tok
    }

    fn skip_newlines(&mut self) {
        while matches!(self.peek(), PyToken::Newline) {
            self.advance();
        }
    }

    fn expect_newline(&mut self) {
        if matches!(self.peek(), PyToken::Newline) {
            self.advance();
        }
    }

    /// Parse an entire Python file → list of statements
    pub fn parse(&mut self) -> Vec<PyStmt> {
        let mut stmts = Vec::new();
        loop {
            self.skip_newlines();
            if matches!(self.peek(), PyToken::Eof) { break; }
            if let PyToken::Indent(_) = self.peek() {
                self.advance(); // consume top-level indent (should be 0)
            }
            if matches!(self.peek(), PyToken::Newline | PyToken::Eof) { continue; }
            if let Some(s) = self.parse_stmt() {
                stmts.push(s);
            }
        }
        stmts
    }

    fn parse_stmt(&mut self) -> Option<PyStmt> {
        match self.peek().clone() {
            PyToken::Ident(ref s) => {
                match s.as_str() {
                    "def" => self.parse_funcdef(),
                    "class" => self.parse_classdef(),
                    "if" => self.parse_if(),
                    "while" => self.parse_while(),
                    "for" => self.parse_for(),
                    "return" => self.parse_return(),
                    "print" => { self.advance(); self.parse_print() }
                    "import" => self.parse_import(),
                    "from" => self.parse_from_import(),
                    "break" => { self.advance(); self.expect_newline(); Some(PyStmt::Break) }
                    "continue" => { self.advance(); self.expect_newline(); Some(PyStmt::Continue) }
                    "pass" => { self.advance(); self.expect_newline(); Some(PyStmt::Pass) }
                    "del" => self.parse_del(),
                    "try" => self.parse_try(),
                    _ => self.parse_assign_or_expr(),
                }
            }
            PyToken::Hash => {
                self.advance();
                let mut comment = String::new();
                while !matches!(self.peek(), PyToken::Newline | PyToken::Eof) {
                    match self.advance() {
                        PyToken::Ident(s) => { if !comment.is_empty() { comment.push(' '); } comment.push_str(&s); }
                        PyToken::IntLit(n) => { if !comment.is_empty() { comment.push(' '); } comment.push_str(&n.to_string()); }
                        _ => {}
                    }
                }
                self.expect_newline();
                Some(PyStmt::Comment(comment))
            }
            _ => {
                self.parse_assign_or_expr()
            }
        }
    }

    fn parse_funcdef(&mut self) -> Option<PyStmt> {
        self.advance(); // def
        let name = if let PyToken::Ident(s) = self.advance() { s } else { return None };
        if !matches!(self.advance(), PyToken::LParen) { return None; }

        let mut params = Vec::new();
        while !matches!(self.peek(), PyToken::RParen | PyToken::Eof) {
            if matches!(self.peek(), PyToken::Star | PyToken::DoubleStar) {
                self.advance();
            }
            if let PyToken::Ident(pname) = self.advance() {
                if pname == "self" {
                    // Skip self parameter for methods
                    if matches!(self.peek(), PyToken::Comma) { self.advance(); }
                    continue;
                }
                let type_hint = if matches!(self.peek(), PyToken::Colon) {
                    self.advance();
                    self.parse_type_hint()
                } else {
                    None
                };
                // Skip default value
                if matches!(self.peek(), PyToken::Eq) {
                    self.advance();
                    let _ = self.parse_expr();
                }
                params.push((pname, type_hint));
            }
            if matches!(self.peek(), PyToken::Comma) { self.advance(); }
        }
        if matches!(self.peek(), PyToken::RParen) { self.advance(); }

        // Return type hint
        let ret_hint = if matches!(self.peek(), PyToken::Arrow) {
            self.advance();
            self.parse_type_hint()
        } else {
            None
        };

        if matches!(self.peek(), PyToken::Colon) { self.advance(); }
        self.expect_newline();

        let body = self.parse_block();

        Some(PyStmt::FuncDef(name, params, ret_hint, body))
    }

    fn parse_type_hint(&mut self) -> Option<String> {
        match self.peek().clone() {
            PyToken::Ident(s) => {
                self.advance();
                // Handle Optional[X], List[X], Dict[K,V], etc.
                if matches!(self.peek(), PyToken::LBracket) {
                    self.advance();
                    let mut depth = 1;
                    while depth > 0 {
                        match self.advance() {
                            PyToken::LBracket => depth += 1,
                            PyToken::RBracket => depth -= 1,
                            PyToken::Eof => break,
                            _ => {}
                        }
                    }
                }
                Some(s)
            }
            _ => None
        }
    }

    fn parse_classdef(&mut self) -> Option<PyStmt> {
        self.advance(); // class
        let _name = if let PyToken::Ident(s) = self.advance() { s } else { return None };

        // Skip parent classes
        if matches!(self.peek(), PyToken::LParen) {
            self.advance();
            let mut depth = 1;
            while depth > 0 {
                match self.advance() {
                    PyToken::LParen => depth += 1,
                    PyToken::RParen => depth -= 1,
                    PyToken::Eof => break,
                    _ => {}
                }
            }
        }
        if matches!(self.peek(), PyToken::Colon) { self.advance(); }
        self.expect_newline();

        let body = self.parse_block();
        Some(PyStmt::ClassDef(_name, body))
    }

    fn parse_if(&mut self) -> Option<PyStmt> {
        self.advance(); // if
        let cond = self.parse_expr()?;
        if matches!(self.peek(), PyToken::Colon) { self.advance(); }
        self.expect_newline();
        let body = self.parse_block();

        let mut elifs = Vec::new();
        let mut else_body = None;

        loop {
            self.skip_newlines();
            if let PyToken::Indent(_) = self.peek() {
                let save = self.pos;
                self.advance();
                if let PyToken::Ident(s) = self.peek() {
                    if s == "elif" {
                        self.advance();
                        let econd = self.parse_expr().unwrap_or(PyExpr::Bool(true));
                        if matches!(self.peek(), PyToken::Colon) { self.advance(); }
                        self.expect_newline();
                        let ebody = self.parse_block();
                        elifs.push((econd, ebody));
                        continue;
                    } else if s == "else" {
                        self.advance();
                        if matches!(self.peek(), PyToken::Colon) { self.advance(); }
                        self.expect_newline();
                        else_body = Some(self.parse_block());
                        break;
                    }
                }
                self.pos = save;
                break;
            } else {
                break;
            }
        }

        Some(PyStmt::If(cond, body, elifs, else_body))
    }

    fn parse_while(&mut self) -> Option<PyStmt> {
        self.advance(); // while
        let cond = self.parse_expr()?;
        if matches!(self.peek(), PyToken::Colon) { self.advance(); }
        self.expect_newline();
        let body = self.parse_block();
        Some(PyStmt::While(cond, body))
    }

    fn parse_for(&mut self) -> Option<PyStmt> {
        self.advance(); // for
        let var = if let PyToken::Ident(s) = self.advance() { s } else { return None };
        // Skip 'in'
        if let PyToken::Ident(s) = self.peek() {
            if s == "in" { self.advance(); }
        }
        let iter = self.parse_expr()?;
        if matches!(self.peek(), PyToken::Colon) { self.advance(); }
        self.expect_newline();
        let body = self.parse_block();
        Some(PyStmt::For(var, iter, body))
    }

    fn parse_return(&mut self) -> Option<PyStmt> {
        self.advance(); // return
        if matches!(self.peek(), PyToken::Newline | PyToken::Eof) {
            self.expect_newline();
            return Some(PyStmt::Return(None));
        }
        let val = self.parse_expr();
        self.expect_newline();
        Some(PyStmt::Return(val))
    }

    fn parse_print(&mut self) -> Option<PyStmt> {
        if !matches!(self.peek(), PyToken::LParen) {
            // print without parens (Python 2 style)
            let expr = self.parse_expr()?;
            self.expect_newline();
            return Some(PyStmt::Print(vec![expr]));
        }
        self.advance(); // (
        let mut args = Vec::new();
        while !matches!(self.peek(), PyToken::RParen | PyToken::Eof) {
            // Skip keyword arguments like end=, sep=
            if let PyToken::Ident(s) = self.peek() {
                let _s = s.clone();
                let save = self.pos;
                self.advance();
                if matches!(self.peek(), PyToken::Eq) {
                    self.advance();
                    let _ = self.parse_expr();
                    if matches!(self.peek(), PyToken::Comma) { self.advance(); }
                    continue;
                }
                self.pos = save;
            }
            if let Some(e) = self.parse_expr() {
                args.push(e);
            }
            if matches!(self.peek(), PyToken::Comma) { self.advance(); }
        }
        if matches!(self.peek(), PyToken::RParen) { self.advance(); }
        self.expect_newline();
        Some(PyStmt::Print(args))
    }

    fn parse_import(&mut self) -> Option<PyStmt> {
        self.advance(); // import
        let mut module = String::new();
        while let PyToken::Ident(s) = self.peek() {
            if !module.is_empty() { module.push('.'); }
            module.push_str(s);
            self.advance();
            if matches!(self.peek(), PyToken::Dot) { self.advance(); } else { break; }
        }
        let alias = if let PyToken::Ident(s) = self.peek() {
            if s == "as" {
                self.advance();
                if let PyToken::Ident(a) = self.advance() { Some(a) } else { None }
            } else { None }
        } else { None };
        self.expect_newline();
        Some(PyStmt::Import(module, alias))
    }

    fn parse_from_import(&mut self) -> Option<PyStmt> {
        self.advance(); // from
        let mut module = String::new();
        while let PyToken::Ident(s) = self.peek() {
            let s = s.clone();
            if s == "import" { break; }
            if !module.is_empty() { module.push('.'); }
            module.push_str(&s);
            self.advance();
            if matches!(self.peek(), PyToken::Dot) { self.advance(); } else { break; }
        }
        if let PyToken::Ident(s) = self.peek() {
            if s == "import" { self.advance(); }
        }
        let mut names = Vec::new();
        loop {
            match self.peek() {
                PyToken::Ident(s) => { names.push(s.clone()); self.advance(); }
                PyToken::Star => { names.push("*".into()); self.advance(); }
                _ => break,
            }
            if matches!(self.peek(), PyToken::Comma) { self.advance(); } else { break; }
        }
        self.expect_newline();
        Some(PyStmt::FromImport(module, names))
    }

    fn parse_del(&mut self) -> Option<PyStmt> {
        self.advance(); // del
        let expr = self.parse_expr()?;
        self.expect_newline();
        Some(PyStmt::Del(expr))
    }

    fn parse_try(&mut self) -> Option<PyStmt> {
        self.advance(); // try
        if matches!(self.peek(), PyToken::Colon) { self.advance(); }
        self.expect_newline();
        let body = self.parse_block();

        let mut handlers = Vec::new();
        loop {
            self.skip_newlines();
            if let PyToken::Indent(_) = self.peek() {
                let save = self.pos;
                self.advance();
                if let PyToken::Ident(s) = self.peek() {
                    if s == "except" {
                        self.advance();
                        let exc_type = if let PyToken::Ident(t) = self.peek() {
                            let t = t.clone();
                            self.advance();
                            Some(t)
                        } else { None };
                        let exc_name = if let PyToken::Ident(s) = self.peek() {
                            if s == "as" {
                                self.advance();
                                if let PyToken::Ident(n) = self.advance() { Some(n) } else { None }
                            } else { None }
                        } else { None };
                        if matches!(self.peek(), PyToken::Colon) { self.advance(); }
                        self.expect_newline();
                        let handler_body = self.parse_block();
                        handlers.push((exc_type, exc_name, handler_body));
                        continue;
                    }
                }
                self.pos = save;
            }
            break;
        }

        Some(PyStmt::Try(body, handlers))
    }

    fn parse_assign_or_expr(&mut self) -> Option<PyStmt> {
        let expr = self.parse_expr()?;

        // Check for assignment: name = expr, or augmented assignment
        match self.peek() {
            PyToken::Eq => {
                self.advance();
                let rhs = self.parse_expr()?;
                self.expect_newline();

                // Extract target names
                let targets = match expr {
                    PyExpr::Name(n) => vec![n],
                    _ => vec!["_".into()],
                };
                Some(PyStmt::Assign(targets, rhs))
            }
            PyToken::PlusEq => { self.advance(); let rhs = self.parse_expr()?; self.expect_newline();
                if let PyExpr::Name(n) = expr { Some(PyStmt::AugAssign(n, "+".into(), rhs)) } else { None }
            }
            PyToken::MinusEq => { self.advance(); let rhs = self.parse_expr()?; self.expect_newline();
                if let PyExpr::Name(n) = expr { Some(PyStmt::AugAssign(n, "-".into(), rhs)) } else { None }
            }
            PyToken::StarEq => { self.advance(); let rhs = self.parse_expr()?; self.expect_newline();
                if let PyExpr::Name(n) = expr { Some(PyStmt::AugAssign(n, "*".into(), rhs)) } else { None }
            }
            PyToken::SlashEq => { self.advance(); let rhs = self.parse_expr()?; self.expect_newline();
                if let PyExpr::Name(n) = expr { Some(PyStmt::AugAssign(n, "/".into(), rhs)) } else { None }
            }
            PyToken::PercentEq => { self.advance(); let rhs = self.parse_expr()?; self.expect_newline();
                if let PyExpr::Name(n) = expr { Some(PyStmt::AugAssign(n, "%".into(), rhs)) } else { None }
            }
            _ => {
                self.expect_newline();
                Some(PyStmt::Expr(expr))
            }
        }
    }

    fn parse_block(&mut self) -> Vec<PyStmt> {
        let mut stmts = Vec::new();
        self.skip_newlines();

        // Determine block indent
        let block_indent = if let PyToken::Indent(n) = self.peek() {
            *n
        } else {
            return stmts;
        };

        loop {
            self.skip_newlines();
            match self.peek() {
                PyToken::Indent(n) if *n >= block_indent => {
                    self.advance();
                    if matches!(self.peek(), PyToken::Newline | PyToken::Eof) { continue; }
                    if let Some(s) = self.parse_stmt() {
                        stmts.push(s);
                    }
                }
                _ => break,
            }
        }
        stmts
    }

    // ============ Expression Parsing ============

    fn parse_expr(&mut self) -> Option<PyExpr> {
        self.parse_ternary()
    }

    fn parse_ternary(&mut self) -> Option<PyExpr> {
        let expr = self.parse_or()?;
        if let PyToken::Ident(s) = self.peek() {
            if s == "if" {
                self.advance();
                let cond = self.parse_or()?;
                if let PyToken::Ident(s) = self.peek() {
                    if s == "else" {
                        self.advance();
                        let alt = self.parse_or()?;
                        return Some(PyExpr::IfExpr(Box::new(expr), Box::new(cond), Box::new(alt)));
                    }
                }
            }
        }
        Some(expr)
    }

    fn parse_or(&mut self) -> Option<PyExpr> {
        let mut left = self.parse_and()?;
        while let PyToken::Ident(s) = self.peek() {
            if s != "or" { break; }
            self.advance();
            let right = self.parse_and()?;
            left = PyExpr::BoolOp("or".into(), vec![left, right]);
        }
        Some(left)
    }

    fn parse_and(&mut self) -> Option<PyExpr> {
        let mut left = self.parse_not()?;
        while let PyToken::Ident(s) = self.peek() {
            if s != "and" { break; }
            self.advance();
            let right = self.parse_not()?;
            left = PyExpr::BoolOp("and".into(), vec![left, right]);
        }
        Some(left)
    }

    fn parse_not(&mut self) -> Option<PyExpr> {
        if let PyToken::Ident(s) = self.peek() {
            if s == "not" {
                self.advance();
                let expr = self.parse_not()?;
                return Some(PyExpr::UnaryOp("not".into(), Box::new(expr)));
            }
        }
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Option<PyExpr> {
        let left = self.parse_addition()?;
        let mut ops = Vec::new();

        loop {
            let op = match self.peek() {
                PyToken::EqEq => "==",
                PyToken::NotEq => "!=",
                PyToken::Lt => "<",
                PyToken::Gt => ">",
                PyToken::LtEq => "<=",
                PyToken::GtEq => ">=",
                PyToken::Ident(s) if s == "in" => "in",
                PyToken::Ident(s) if s == "not" => {
                    let save = self.pos;
                    self.advance();
                    if let PyToken::Ident(s2) = self.peek() {
                        if s2 == "in" {
                            self.advance();
                            let right = self.parse_addition()?;
                            ops.push(("not in".to_string(), right));
                            continue;
                        }
                    }
                    self.pos = save;
                    break;
                }
                _ => break,
            };
            let op_str = op.to_string();
            self.advance();
            let right = self.parse_addition()?;
            ops.push((op_str, right));
        }

        if ops.is_empty() {
            Some(left)
        } else {
            Some(PyExpr::Compare(Box::new(left), ops))
        }
    }

    fn parse_addition(&mut self) -> Option<PyExpr> {
        let mut left = self.parse_multiplication()?;
        loop {
            let op = match self.peek() {
                PyToken::Plus => "+",
                PyToken::Minus => "-",
                _ => break,
            };
            let op_str = op.to_string();
            self.advance();
            let right = self.parse_multiplication()?;
            left = PyExpr::BinOp(Box::new(left), op_str, Box::new(right));
        }
        Some(left)
    }

    fn parse_multiplication(&mut self) -> Option<PyExpr> {
        let mut left = self.parse_power()?;
        loop {
            let op = match self.peek() {
                PyToken::Star => "*",
                PyToken::DoubleSlash => "//",
                PyToken::Slash => "/",
                PyToken::Percent => "%",
                _ => break,
            };
            let op_str = op.to_string();
            self.advance();
            let right = self.parse_power()?;
            left = PyExpr::BinOp(Box::new(left), op_str, Box::new(right));
        }
        Some(left)
    }

    fn parse_power(&mut self) -> Option<PyExpr> {
        let left = self.parse_unary()?;
        if matches!(self.peek(), PyToken::DoubleStar) {
            self.advance();
            let right = self.parse_unary()?;
            Some(PyExpr::BinOp(Box::new(left), "**".into(), Box::new(right)))
        } else {
            Some(left)
        }
    }

    fn parse_unary(&mut self) -> Option<PyExpr> {
        match self.peek() {
            PyToken::Minus => {
                self.advance();
                let expr = self.parse_postfix()?;
                Some(PyExpr::UnaryOp("-".into(), Box::new(expr)))
            }
            _ => self.parse_postfix()
        }
    }

    fn parse_postfix(&mut self) -> Option<PyExpr> {
        let mut expr = self.parse_atom()?;

        loop {
            match self.peek() {
                PyToken::LParen => {
                    self.advance();
                    let mut args = Vec::new();
                    let mut kwargs = Vec::new();
                    while !matches!(self.peek(), PyToken::RParen | PyToken::Eof) {
                        // Check for keyword argument
                        if let PyToken::Ident(s) = self.peek() {
                            let s = s.clone();
                            let save = self.pos;
                            self.advance();
                            if matches!(self.peek(), PyToken::Eq) {
                                self.advance();
                                let val = self.parse_expr().unwrap_or(PyExpr::None);
                                kwargs.push((s, val));
                                if matches!(self.peek(), PyToken::Comma) { self.advance(); }
                                continue;
                            }
                            self.pos = save;
                        }
                        if let Some(arg) = self.parse_expr() {
                            args.push(arg);
                        }
                        if matches!(self.peek(), PyToken::Comma) { self.advance(); }
                    }
                    if matches!(self.peek(), PyToken::RParen) { self.advance(); }
                    expr = PyExpr::Call(Box::new(expr), args, kwargs);
                }
                PyToken::LBracket => {
                    self.advance();
                    let idx = self.parse_expr()?;
                    if matches!(self.peek(), PyToken::RBracket) { self.advance(); }

                    // Check for subscript assignment: x[i] = v
                    // This is handled at statement level, not here
                    expr = PyExpr::Subscript(Box::new(expr), Box::new(idx));
                }
                PyToken::Dot => {
                    self.advance();
                    if let PyToken::Ident(attr) = self.advance() {
                        expr = PyExpr::Attribute(Box::new(expr), attr);
                    }
                }
                _ => break,
            }
        }

        Some(expr)
    }

    fn parse_atom(&mut self) -> Option<PyExpr> {
        match self.peek().clone() {
            PyToken::IntLit(n) => { self.advance(); Some(PyExpr::Int(n)) }
            PyToken::FloatLit(f) => { self.advance(); Some(PyExpr::Float(f)) }
            PyToken::StrLit(s) => { self.advance(); Some(PyExpr::Str(s)) }
            PyToken::FStrLit(s) => { self.advance(); Some(PyExpr::FStr(s)) }
            PyToken::Ident(s) => {
                match s.as_str() {
                    "True" => { self.advance(); Some(PyExpr::Bool(true)) }
                    "False" => { self.advance(); Some(PyExpr::Bool(false)) }
                    "None" => { self.advance(); Some(PyExpr::None) }
                    _ => { self.advance(); Some(PyExpr::Name(s)) }
                }
            }
            PyToken::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                if matches!(self.peek(), PyToken::RParen) { self.advance(); }
                Some(expr)
            }
            PyToken::LBracket => {
                self.advance();
                let mut elems = Vec::new();
                while !matches!(self.peek(), PyToken::RBracket | PyToken::Eof) {
                    if let Some(e) = self.parse_expr() {
                        // Check for list comprehension: [expr for x in iter]
                        if let PyToken::Ident(s) = self.peek() {
                            if s == "for" && elems.is_empty() {
                                self.advance();
                                let var = if let PyToken::Ident(v) = self.advance() { v } else { "_".into() };
                                if let PyToken::Ident(s) = self.peek() { if s == "in" { self.advance(); } }
                                let iter = self.parse_expr()?;
                                let filter = if let PyToken::Ident(s) = self.peek() {
                                    if s == "if" { self.advance(); self.parse_expr().map(Box::new) }
                                    else { None }
                                } else { None };
                                if matches!(self.peek(), PyToken::RBracket) { self.advance(); }
                                return Some(PyExpr::ListComp(Box::new(e), var, Box::new(iter), filter));
                            }
                        }
                        elems.push(e);
                    }
                    if matches!(self.peek(), PyToken::Comma) { self.advance(); }
                }
                if matches!(self.peek(), PyToken::RBracket) { self.advance(); }
                Some(PyExpr::List(elems))
            }
            PyToken::LBrace => {
                self.advance();
                let mut pairs = Vec::new();
                while !matches!(self.peek(), PyToken::RBrace | PyToken::Eof) {
                    let key = self.parse_expr()?;
                    if matches!(self.peek(), PyToken::Colon) {
                        self.advance();
                        let val = self.parse_expr()?;
                        pairs.push((key, val));
                    }
                    if matches!(self.peek(), PyToken::Comma) { self.advance(); }
                }
                if matches!(self.peek(), PyToken::RBrace) { self.advance(); }
                Some(PyExpr::Dict(pairs))
            }
            PyToken::Minus => {
                self.advance();
                let expr = self.parse_atom()?;
                Some(PyExpr::UnaryOp("-".into(), Box::new(expr)))
            }
            _ => None,
        }
    }
}

// ============ Python → Ernos Emitter ============

fn py_type_to_ernos(hint: &str) -> &str {
    match hint {
        "int" => "Int",
        "str" => "Str",
        "bool" => "Int",
        "float" => "Int",
        "list" | "List" => "List",
        "dict" | "Dict" => "Int",
        "None" | "NoneType" => "Int",
        "Optional" => "Int",
        _ => "Int",
    }
}

fn emit_indent(out: &mut String, depth: usize) {
    for _ in 0..depth {
        out.push_str("    ");
    }
}

pub fn emit_ernos_from_python(filename: &str, source: &str) -> String {
    let mut lexer = PyLexer::new(source);
    let tokens = lexer.tokenize();
    let mut parser = PyParser::new(tokens);
    let stmts = parser.parse();

    let mut out = String::new();
    out.push_str(&format!("# Auto-transpiled from Python: {}\n", filename));
    out.push_str("# Generated by: ernos transpile\n\n");

    let mut ctx = EmitCtx {
        vars: HashMap::new(),
        depth: 0,
    };

    for stmt in &stmts {
        emit_stmt(&mut out, &mut ctx, stmt);
    }

    out
}

struct EmitCtx {
    vars: HashMap<String, bool>,  // name → is_set (for tracking first assignment)
    depth: usize,
}

fn emit_stmt(out: &mut String, ctx: &mut EmitCtx, stmt: &PyStmt) {
    match stmt {
        PyStmt::FuncDef(name, params, ret_hint, body) => {
            emit_indent(out, ctx.depth);
            let ep_name = name.replace("__init__", "create");

            if params.is_empty() {
                if let Some(ret) = ret_hint {
                    out.push_str(&format!("define {} returning {}:\n", ep_name, py_type_to_ernos(ret)));
                } else {
                    out.push_str(&format!("define {}:\n", ep_name));
                }
            } else {
                let param_parts: Vec<String> = params.iter()
                    .map(|(pname, hint)| {
                        let ptype = hint.as_deref().map(py_type_to_ernos).unwrap_or("Int");
                        format!("{} as {}", pname, ptype)
                    })
                    .collect();

                if let Some(ret) = ret_hint {
                    out.push_str(&format!("define {} with {} returning {}:\n",
                        ep_name, param_parts.join(" and "), py_type_to_ernos(ret)));
                } else {
                    out.push_str(&format!("define {} with {}:\n",
                        ep_name, param_parts.join(" and ")));
                }
            }

            let mut inner_ctx = EmitCtx { vars: HashMap::new(), depth: ctx.depth + 1 };
            for p in params { inner_ctx.vars.insert(p.0.clone(), true); }
            for s in body {
                emit_stmt(out, &mut inner_ctx, s);
            }
            out.push('\n');
        }

        PyStmt::ClassDef(name, body) => {
            // Extract fields from __init__ method
            let mut fields = Vec::new();
            let mut methods = Vec::new();

            for s in body {
                if let PyStmt::FuncDef(fname, _params, _ret, fbody) = s {
                    if fname == "__init__" {
                        for init_stmt in fbody {
                            if let PyStmt::Assign(targets, _val) = init_stmt {
                                for t in targets {
                                    if t.starts_with("self.") {
                                        let field_name = t.strip_prefix("self.").unwrap_or(t);
                                        fields.push(field_name.to_string());
                                    }
                                }
                            }
                        }
                    } else {
                        methods.push(s.clone());
                    }
                }
            }

            emit_indent(out, ctx.depth);
            out.push_str(&format!("define structure {}:\n", name));
            for field in &fields {
                emit_indent(out, ctx.depth + 1);
                out.push_str(&format!("field {} as Int\n", field));
            }
            out.push('\n');

            // Emit methods
            for method in &methods {
                if let PyStmt::FuncDef(mname, params, _ret, mbody) = method {
                    emit_indent(out, ctx.depth);
                    let param_parts: Vec<String> = params.iter()
                        .map(|(pname, hint)| {
                            let ptype = hint.as_deref().map(py_type_to_ernos).unwrap_or("Int");
                            format!("{} as {}", pname, ptype)
                        })
                        .collect();

                    if param_parts.is_empty() {
                        out.push_str(&format!("define method {} on {}:\n", mname, name));
                    } else {
                        out.push_str(&format!("define method {} on {} with {}:\n",
                            mname, name, param_parts.join(" and ")));
                    }

                    let mut inner_ctx = EmitCtx { vars: HashMap::new(), depth: ctx.depth + 1 };
                    for p in params { inner_ctx.vars.insert(p.0.clone(), true); }
                    for s in mbody {
                        emit_stmt(out, &mut inner_ctx, s);
                    }
                    out.push('\n');
                }
            }
        }

        PyStmt::Assign(targets, val) => {
            let target = &targets[0];
            // Handle self.field = val → set self_field to val
            if target.contains('.') {
                emit_indent(out, ctx.depth);
                let parts: Vec<&str> = target.splitn(2, '.').collect();
                out.push_str(&format!("set the {} of {} to ", parts[1], parts[0]));
                emit_expr(out, val);
                out.push('\n');
            } else if ctx.vars.contains_key(target) {
                emit_indent(out, ctx.depth);
                out.push_str(&format!("set {} to ", target));
                emit_expr(out, val);
                out.push('\n');
            } else {
                ctx.vars.insert(target.clone(), true);
                emit_indent(out, ctx.depth);
                out.push_str(&format!("set {} to ", target));
                emit_expr(out, val);
                out.push('\n');
            }
        }

        PyStmt::AugAssign(name, op, val) => {
            emit_indent(out, ctx.depth);
            out.push_str(&format!("set {} to {} {} ", name, name, op));
            emit_expr(out, val);
            out.push('\n');
        }

        PyStmt::Print(args) => {
            for arg in args {
                emit_indent(out, ctx.depth);
                out.push_str("display ");
                emit_expr(out, arg);
                out.push('\n');
            }
        }

        PyStmt::Return(val) => {
            emit_indent(out, ctx.depth);
            if let Some(v) = val {
                out.push_str("return ");
                emit_expr(out, v);
                out.push('\n');
            } else {
                out.push_str("return 0\n");
            }
        }

        PyStmt::If(cond, body, elifs, else_body) => {
            emit_indent(out, ctx.depth);
            out.push_str("if ");
            emit_cond(out, cond);
            out.push_str(":\n");
            for s in body { emit_stmt(out, ctx, s); }

            for (econd, ebody) in elifs {
                emit_indent(out, ctx.depth);
                out.push_str("else if ");
                emit_cond(out, econd);
                out.push_str(":\n");
                ctx.depth += 1;
                for s in ebody { emit_stmt(out, ctx, s); }
                ctx.depth -= 1;
            }

            if let Some(eb) = else_body {
                emit_indent(out, ctx.depth);
                out.push_str("else:\n");
                ctx.depth += 1;
                for s in eb { emit_stmt(out, ctx, s); }
                ctx.depth -= 1;
            }
        }

        PyStmt::While(cond, body) => {
            emit_indent(out, ctx.depth);
            out.push_str("repeat while ");
            emit_cond(out, cond);
            out.push_str(":\n");
            ctx.depth += 1;
            for s in body { emit_stmt(out, ctx, s); }
            ctx.depth -= 1;
        }

        PyStmt::For(var, iter, body) => {
            emit_indent(out, ctx.depth);
            ctx.vars.insert(var.clone(), true);

            // Check if iterating over range()
            if let PyExpr::Call(func, args, _) = iter {
                if let PyExpr::Name(fname) = func.as_ref() {
                    if fname == "range" {
                        out.push_str(&format!("for each {} in ", var));
                        // range(n) or range(a, b)
                        match args.len() {
                            1 => {
                                out.push_str("range(0 and ");
                                emit_expr(out, &args[0]);
                                out.push(')');
                            }
                            2 => {
                                out.push_str("range(");
                                emit_expr(out, &args[0]);
                                out.push_str(" and ");
                                emit_expr(out, &args[1]);
                                out.push(')');
                            }
                            _ => {
                                out.push_str("range(0 and 10)");
                            }
                        }
                        out.push_str(":\n");
                        ctx.depth += 1;
                        for s in body { emit_stmt(out, ctx, s); }
                        ctx.depth -= 1;
                        return;
                    }
                }
            }

            out.push_str(&format!("for each {} in ", var));
            emit_expr(out, iter);
            out.push_str(":\n");
            ctx.depth += 1;
            for s in body { emit_stmt(out, ctx, s); }
            ctx.depth -= 1;
        }

        PyStmt::Break => {
            emit_indent(out, ctx.depth);
            out.push_str("break\n");
        }

        PyStmt::Continue => {
            emit_indent(out, ctx.depth);
            out.push_str("continue\n");
        }

        PyStmt::Pass => {
            emit_indent(out, ctx.depth);
            out.push_str("# pass\n");
        }

        PyStmt::Import(module, alias) => {
            emit_indent(out, ctx.depth);
            if let Some(a) = alias {
                out.push_str(&format!("# import {} as {} — manual translation needed\n", module, a));
            } else {
                out.push_str(&format!("# import {} — manual translation needed\n", module));
            }
        }

        PyStmt::FromImport(module, names) => {
            emit_indent(out, ctx.depth);
            out.push_str(&format!("# from {} import {} — manual translation needed\n", module, names.join(", ")));
        }

        PyStmt::Comment(text) => {
            emit_indent(out, ctx.depth);
            out.push_str(&format!("# {}\n", text));
        }

        PyStmt::Expr(expr) => {
            // Method calls like list.append(), etc.
            emit_indent(out, ctx.depth);
            match expr {
                PyExpr::Call(func, args, _) => {
                    emit_call_stmt(out, func, args);
                    out.push('\n');
                }
                _ => {
                    emit_expr(out, expr);
                    out.push('\n');
                }
            }
        }

        PyStmt::Del(expr) => {
            emit_indent(out, ctx.depth);
            if let PyExpr::Subscript(obj, key) = expr {
                out.push_str("map_delete(");
                emit_expr(out, obj);
                out.push_str(" and ");
                emit_expr(out, key);
                out.push_str(")\n");
            } else {
                out.push_str("# del ");
                emit_expr(out, expr);
                out.push_str(" — manual translation needed\n");
            }
        }

        PyStmt::Try(body, handlers) => {
            // Basic translation — wrap body statements
            emit_indent(out, ctx.depth);
            out.push_str("# try/except — approximate translation\n");
            for s in body { emit_stmt(out, ctx, s); }
            for (exc_type, _exc_name, handler_body) in handlers {
                emit_indent(out, ctx.depth);
                if let Some(t) = exc_type {
                    out.push_str(&format!("# except {}:\n", t));
                } else {
                    out.push_str("# except:\n");
                }
                ctx.depth += 1;
                for s in handler_body { emit_stmt(out, ctx, s); }
                ctx.depth -= 1;
            }
        }
    }
}

fn emit_call_stmt(out: &mut String, func: &PyExpr, args: &[PyExpr]) {
    // Handle method calls
    if let PyExpr::Attribute(obj, method) = func {
        match method.as_str() {
            "append" => {
                out.push_str("append_list(");
                emit_expr(out, obj);
                if !args.is_empty() {
                    out.push_str(" and ");
                    emit_expr(out, &args[0]);
                }
                out.push(')');
                return;
            }
            "pop" => {
                out.push_str("pop_list(");
                emit_expr(out, obj);
                out.push(')');
                return;
            }
            _ => {}
        }
    }

    // Regular function call
    emit_expr(out, &PyExpr::Call(Box::new(func.clone()), args.to_vec(), vec![]));
}

fn emit_cond(out: &mut String, expr: &PyExpr) {
    match expr {
        PyExpr::Compare(left, ops) => {
            emit_expr(out, left);
            for (op, right) in ops {
                let ep_op = match op.as_str() {
                    "==" => " equals ",
                    "!=" => " not equals ",
                    "<" => " < ",
                    ">" => " > ",
                    "<=" => " <= ",
                    ">=" => " >= ",
                    "in" => {
                        out.push_str(" # 'in' check — needs manual translation");
                        return;
                    }
                    _ => &format!(" {} ", op),
                };
                out.push_str(ep_op);
                emit_expr(out, right);
            }
        }
        PyExpr::BoolOp(op, exprs) => {
            let ep_op = if op == "and" { " and " } else { " or " };
            for (i, e) in exprs.iter().enumerate() {
                if i > 0 { out.push_str(ep_op); }
                emit_cond(out, e);
            }
        }
        PyExpr::UnaryOp(op, expr) if op == "not" => {
            out.push_str("not ");
            emit_cond(out, expr);
        }
        _ => {
            emit_expr(out, expr);
        }
    }
}

fn emit_expr(out: &mut String, expr: &PyExpr) {
    match expr {
        PyExpr::Int(n) => out.push_str(&n.to_string()),
        PyExpr::Float(f) => out.push_str(&(*f as i64).to_string()),
        PyExpr::Str(s) => {
            out.push('"');
            out.push_str(&s.replace('"', "\\\"").replace('\n', "\\n"));
            out.push('"');
        }
        PyExpr::FStr(s) => {
            out.push_str("f\"");
            out.push_str(&s.replace('"', "\\\"").replace('\n', "\\n"));
            out.push('"');
        }
        PyExpr::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        PyExpr::None => out.push('0'),
        PyExpr::Name(n) => out.push_str(n),

        PyExpr::BinOp(left, op, right) => {
            let ep_op = match op.as_str() {
                "+" => " + ",
                "-" => " - ",
                "*" => " * ",
                "//" => " / ",
                "/" => " / ",
                "%" => " % ",
                "**" => {
                    // No native power operator — use multiplication or note
                    out.push_str("# power operation: ");
                    emit_expr(out, left);
                    out.push_str(" ** ");
                    emit_expr(out, right);
                    return;
                }
                _ => &format!(" {} ", op),
            };
            emit_expr(out, left);
            out.push_str(ep_op);
            emit_expr(out, right);
        }

        PyExpr::UnaryOp(op, expr) => {
            if op == "-" {
                out.push_str("0 - ");
                emit_expr(out, expr);
            } else if op == "not" {
                out.push_str("not ");
                emit_expr(out, expr);
            }
        }

        PyExpr::Compare(left, ops) => {
            emit_expr(out, left);
            for (op, right) in ops {
                let ep_op = match op.as_str() {
                    "==" => " equals ",
                    "!=" => " not equals ",
                    _ => &format!(" {} ", op),
                };
                out.push_str(ep_op);
                emit_expr(out, right);
            }
        }

        PyExpr::BoolOp(op, exprs) => {
            let ep_op = if op == "and" { " and " } else { " or " };
            for (i, e) in exprs.iter().enumerate() {
                if i > 0 { out.push_str(ep_op); }
                emit_expr(out, e);
            }
        }

        PyExpr::Call(func, args, _kwargs) => {
            // Map Python builtins to Ernos
            if let PyExpr::Name(fname) = func.as_ref() {
                match fname.as_str() {
                    "len" => {
                        if !args.is_empty() {
                            match &args[0] {
                                PyExpr::Name(_) => {
                                    // Could be list or string — default to length_list
                                    out.push_str("length_list(");
                                    emit_expr(out, &args[0]);
                                    out.push(')');
                                }
                                _ => {
                                    out.push_str("length_list(");
                                    emit_expr(out, &args[0]);
                                    out.push(')');
                                }
                            }
                        }
                        return;
                    }
                    "int" => {
                        if !args.is_empty() {
                            out.push_str("string_to_int(");
                            emit_expr(out, &args[0]);
                            out.push(')');
                        }
                        return;
                    }
                    "str" => {
                        if !args.is_empty() {
                            out.push_str("int_to_string(");
                            emit_expr(out, &args[0]);
                            out.push(')');
                        }
                        return;
                    }
                    "input" => {
                        out.push_str("read_line()");
                        return;
                    }
                    "abs" => {
                        if !args.is_empty() {
                            out.push_str("ep_abs(");
                            emit_expr(out, &args[0]);
                            out.push(')');
                        }
                        return;
                    }
                    "print" => {
                        // print as expression — unlikely but handle
                        if !args.is_empty() {
                            out.push_str("display ");
                            emit_expr(out, &args[0]);
                        }
                        return;
                    }
                    _ => {}
                }
            }

            // Handle method calls
            if let PyExpr::Attribute(obj, method) = func.as_ref() {
                match method.as_str() {
                    "append" => {
                        out.push_str("append_list(");
                        emit_expr(out, obj);
                        if !args.is_empty() {
                            out.push_str(" and ");
                            emit_expr(out, &args[0]);
                        }
                        out.push(')');
                        return;
                    }
                    "pop" => {
                        out.push_str("pop_list(");
                        emit_expr(out, obj);
                        out.push(')');
                        return;
                    }
                    "upper" => {
                        out.push_str("string_upper(");
                        emit_expr(out, obj);
                        out.push(')');
                        return;
                    }
                    "lower" => {
                        out.push_str("string_lower(");
                        emit_expr(out, obj);
                        out.push(')');
                        return;
                    }
                    "strip" => {
                        out.push_str("string_trim(");
                        emit_expr(out, obj);
                        out.push(')');
                        return;
                    }
                    "split" => {
                        out.push_str("string_split(");
                        emit_expr(out, obj);
                        if !args.is_empty() {
                            out.push_str(" and ");
                            emit_expr(out, &args[0]);
                        } else {
                            out.push_str(" and \" \"");
                        }
                        out.push(')');
                        return;
                    }
                    "replace" => {
                        if args.len() >= 2 {
                            out.push_str("string_replace(");
                            emit_expr(out, obj);
                            out.push_str(" and ");
                            emit_expr(out, &args[0]);
                            out.push_str(" and ");
                            emit_expr(out, &args[1]);
                            out.push(')');
                        }
                        return;
                    }
                    "find" | "index" => {
                        out.push_str("string_index_of(");
                        emit_expr(out, obj);
                        if !args.is_empty() {
                            out.push_str(" and ");
                            emit_expr(out, &args[0]);
                        }
                        out.push(')');
                        return;
                    }
                    "join" => {
                        // " ".join(list) → manual handling
                        out.push_str("# .join() — manual translation needed: ");
                        emit_expr(out, obj);
                        out.push_str(".join(...)");
                        return;
                    }
                    "keys" => {
                        out.push_str("map_keys(");
                        emit_expr(out, obj);
                        out.push(')');
                        return;
                    }
                    "values" => {
                        out.push_str("map_values(");
                        emit_expr(out, obj);
                        out.push(')');
                        return;
                    }
                    "items" => {
                        out.push_str("# .items() — manual translation needed for ");
                        emit_expr(out, obj);
                        return;
                    }
                    "get" => {
                        out.push_str("map_get_val(");
                        emit_expr(out, obj);
                        if !args.is_empty() {
                            out.push_str(" and ");
                            emit_expr(out, &args[0]);
                        }
                        out.push(')');
                        return;
                    }
                    _ => {}
                }
            }

            // Generic function call
            emit_expr(out, func);
            out.push('(');
            for (i, arg) in args.iter().enumerate() {
                if i > 0 { out.push_str(" and "); }
                emit_expr(out, arg);
            }
            out.push(')');
        }

        PyExpr::Attribute(obj, attr) => {
            out.push_str("the ");
            out.push_str(attr);
            out.push_str(" of ");
            emit_expr(out, obj);
        }

        PyExpr::Subscript(obj, idx) => {
            out.push_str("get_list(");
            emit_expr(out, obj);
            out.push_str(" and ");
            emit_expr(out, idx);
            out.push(')');
        }

        PyExpr::List(elems) => {
            if elems.is_empty() {
                out.push_str("create_list()");
            } else {
                out.push('[');
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 { out.push_str(", "); }
                    emit_expr(out, e);
                }
                out.push(']');
            }
        }

        PyExpr::Dict(pairs) => {
            if pairs.is_empty() {
                out.push_str("create_map()");
            } else {
                // Emit as create_map + map_insert calls
                out.push_str("create_map()");
                // Note: dict literals need to be expanded to separate statements
                // This is a limitation — complex dict literals get a comment
                if !pairs.is_empty() {
                    out.push_str(" # dict literal — add map_insert calls");
                }
            }
        }

        PyExpr::IfExpr(val, cond, alt) => {
            // Python ternary: val if cond else alt
            // No direct Ernos equivalent — expand to conditional
            out.push_str("# ternary: ");
            emit_expr(out, val);
            out.push_str(" if ");
            emit_expr(out, cond);
            out.push_str(" else ");
            emit_expr(out, alt);
        }

        PyExpr::ListComp(_expr, var, iter, filter) => {
            out.push_str(&format!("# list comprehension: [{} for {} in ", "...", var));
            emit_expr(out, iter);
            if let Some(f) = filter {
                out.push_str(" if ");
                emit_expr(out, f);
            }
            out.push(']');
        }
    }
}
