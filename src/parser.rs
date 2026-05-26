use crate::token::{Span, Token};
use crate::ast::{Program, Function, StructDef, EnumDef, MethodDef, TraitDef, TraitImpl, TypeAnnotation, Stmt, StmtNode, Expr, ExprNode, Op, CompOp, LogicalOp};

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
}

#[derive(PartialOrd, PartialEq, Clone, Copy)]
enum Precedence {
    Lowest = 0,
    LogicalOr = 1,
    LogicalAnd = 2,
    Comparison = 3,
    Sum = 4,
    Product = 5,
    Call = 6,
    FieldAccess = 7,
}

pub struct Parser {
    tokens: Vec<(Token, Span)>,
    pos: usize,
    in_condition: bool,
    call_depth: usize,
}

impl Parser {
    pub fn new(tokens: Vec<(Token, Span)>) -> Self {
        Self { tokens, pos: 0, in_condition: false, call_depth: 0 }
    }

    fn peek(&self) -> &Token {
        if self.pos < self.tokens.len() {
            &self.tokens[self.pos].0
        } else {
            &Token::EOF
        }
    }

    fn peek_span(&self) -> Span {
        if self.pos < self.tokens.len() {
            self.tokens[self.pos].1.clone()
        } else {
            Span::new(1, 1)
        }
    }

    fn advance(&mut self) -> (Token, Span) {
        if self.pos < self.tokens.len() {
            let res = self.tokens[self.pos].clone();
            self.pos += 1;
            res
        } else {
            (Token::EOF, Span::new(1, 1))
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        let (actual, span) = self.advance();
        if actual == expected {
            Ok(())
        } else {
            Err(ParseError {
                message: format!("Expected {:?}, found {:?}", expected, actual),
                span,
            })
        }
    }

    fn expect_identifier(&mut self) -> Result<(String, Span), ParseError> {
        let (tok, span) = self.advance();
        match tok {
            Token::Identifier(name) => Ok((name, span)),
            // Allow keywords to be used as variable names in expression contexts
            Token::Choice => Ok(("choice".to_string(), span)),
            Token::Field => Ok(("field".to_string(), span)),
            Token::Variant => Ok(("variant".to_string(), span)),
            Token::Range => Ok(("range".to_string(), span)),
            Token::Trait => Ok(("trait".to_string(), span)),
            Token::Implement => Ok(("implement".to_string(), span)),
            Token::Structure => Ok(("structure".to_string(), span)),
            Token::Channel => Ok(("channel".to_string(), span)),
            Token::Check => Ok(("check".to_string(), span)),
            _ => Err(ParseError {
                message: format!("Expected identifier, found {:?}", tok),
                span,
            }),
        }
    }

    fn token_precedence(&self, tok: &Token) -> Precedence {
        match tok {
            Token::LogicalOr => Precedence::LogicalOr,
            Token::LogicalAnd => Precedence::LogicalAnd,
            // Context-aware 'and': treat as logical AND in conditions when not inside a function call
            Token::And if self.in_condition && self.call_depth == 0 => Precedence::LogicalAnd,
            Token::LessThan | Token::GreaterThan | Token::LessEqual | Token::GreaterEqual | Token::Equals | Token::NotEquals => Precedence::Comparison,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Multiply | Token::Divide | Token::Modulo => Precedence::Product,
            Token::LeftParen => Precedence::Call,
            Token::Dot => Precedence::FieldAccess,
            _ => Precedence::Lowest,
        }
    }

    fn parse_type_annotation(&mut self) -> Result<TypeAnnotation, ParseError> {
        let (tok, span) = self.advance();
        match tok {
            Token::Identifier(name) => {
                match name.as_str() {
                    "Int" => Ok(TypeAnnotation::Int),
                    "Float" => Ok(TypeAnnotation::Float),
                    "Bool" => Ok(TypeAnnotation::Bool),
                    "Str" => Ok(TypeAnnotation::Str),
                    "DynStr" => Ok(TypeAnnotation::DynStr),
                    "List" => Ok(TypeAnnotation::List),
                    _ => {
                        // Check for generic: Name of T and U
                        if self.peek() == &Token::Of {
                            self.advance(); // consume "of"
                            let mut type_args = Vec::new();
                            type_args.push(self.parse_type_annotation()?);
                            while self.peek() == &Token::And {
                                self.advance(); // consume "and"
                                type_args.push(self.parse_type_annotation()?);
                            }
                            Ok(TypeAnnotation::Generic(name, type_args))
                        } else {
                            Ok(TypeAnnotation::UserDefined(name))
                        }
                    }
                }
            }
            _ => Err(ParseError {
                message: format!("Expected type name (Int, Str, DynStr, List, or a structure name), found {:?}", tok),
                span,
            }),
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut imports = Vec::new();
        let mut externals = Vec::new();
        let mut functions = Vec::new();
        let mut struct_defs = Vec::new();
        let mut enum_defs = Vec::new();
        let mut method_defs = Vec::new();
        let mut trait_defs = Vec::new();
        let mut trait_impls = Vec::new();

        while self.peek() != &Token::EOF {
            // Skip leading newlines
            if self.peek() == &Token::Newline {
                self.advance();
                continue;
            }

            if self.peek() == &Token::Import {
                self.advance(); // consume "import"
                if let (Token::StringLiteral(path), _) = self.advance() {
                    imports.push(path);
                    // Optional newline
                    if self.peek() == &Token::Newline {
                        self.advance();
                    }
                } else {
                    return Err(ParseError {
                        message: "Expected string literal after 'import'".to_string(),
                        span: self.peek_span(),
                    });
                }
            } else if self.peek() == &Token::External {
                self.advance(); // consume "external"
                self.expect(Token::Define)?;
                let (name, _) = self.expect_identifier()?;
                let mut params = Vec::new();
                if self.peek() == &Token::With {
                    self.advance(); // consume "with"
                    let mut is_borrow = false;
                    if self.peek() == &Token::Borrow {
                        self.advance();
                        is_borrow = true;
                    }
                    let (first_param, _) = self.expect_identifier()?;
                    let mut first_type = None;
                    if self.peek() == &Token::As {
                        self.advance(); // consume "as"
                        first_type = Some(self.parse_type_annotation()?);
                    }
                    params.push((first_param, is_borrow, first_type));
                    while self.peek() == &Token::And {
                        self.advance(); // consume "and"
                        let mut next_is_borrow = false;
                        if self.peek() == &Token::Borrow {
                            self.advance();
                            next_is_borrow = true;
                        }
                        let (next_param, _) = self.expect_identifier()?;
                        let mut next_type = None;
                        if self.peek() == &Token::As {
                            self.advance(); // consume "as"
                            next_type = Some(self.parse_type_annotation()?);
                        }
                        params.push((next_param, next_is_borrow, next_type));
                    }
                }
                let mut return_type = None;
                if self.peek() == &Token::Returning {
                    self.advance(); // consume "returning"
                    return_type = Some(self.parse_type_annotation()?);
                }
                // Optional newline after external definition
                if self.peek() == &Token::Newline {
                    self.advance();
                }
                externals.push(crate::ast::ExternalFunction { name, params, return_type });
            } else if self.peek() == &Token::Async {
                self.advance(); // consume "async"
                let func = self.parse_function(true)?;
                functions.push(func);
            } else if self.peek() == &Token::Define {
                // Check if this is a structure or choice definition
                if self.pos + 1 < self.tokens.len() && self.tokens[self.pos + 1].0 == Token::Structure {
                    let sd = self.parse_struct_def()?;
                    struct_defs.push(sd);
                } else if self.pos + 1 < self.tokens.len() && self.tokens[self.pos + 1].0 == Token::Choice {
                    let ed = self.parse_enum_def()?;
                    enum_defs.push(ed);
                } else if self.pos + 1 < self.tokens.len() && self.tokens[self.pos + 1].0 == Token::Trait {
                    let td = self.parse_trait_def()?;
                    trait_defs.push(td);
                } else if self.pos + 1 < self.tokens.len() {
                    if let Token::Identifier(_) = &self.tokens[self.pos + 1].0 {
                        // Look ahead: is there an "on" after the name to detect method?
                        let is_method = self.pos + 2 < self.tokens.len() && self.tokens[self.pos + 2].0 == Token::On;
                        if is_method {
                            let md = self.parse_method_def()?;
                            method_defs.push(md);
                        } else {
                            let func = self.parse_function(false)?;
                            functions.push(func);
                        }
                    } else {
                        let func = self.parse_function(false)?;
                        functions.push(func);
                    }
                } else {
                    let func = self.parse_function(false)?;
                    functions.push(func);
                }
            } else if self.peek() == &Token::Implement {
                let ti = self.parse_trait_impl()?;
                trait_impls.push(ti);
            } else {
                return Err(ParseError {
                    message: format!("Unexpected token at top level: {:?}", self.peek()),
                    span: self.peek_span(),
                });
            }
        }

        Ok(Program { imports, externals, functions, struct_defs, enum_defs, method_defs, trait_defs, trait_impls })
    }

    fn parse_struct_def(&mut self) -> Result<StructDef, ParseError> {
        self.expect(Token::Define)?;
        self.expect(Token::Structure)?;
        let (name, _) = self.expect_identifier()?;

        // Parse optional type parameters: of T and U
        let mut type_params = Vec::new();
        if self.peek() == &Token::Of {
            self.advance(); // consume "of"
            let (tp, _) = self.expect_identifier()?;
            type_params.push(tp);
            while self.peek() == &Token::And {
                self.advance();
                let (tp2, _) = self.expect_identifier()?;
                type_params.push(tp2);
            }
        }

        self.expect(Token::Colon)?;

        if self.peek() == &Token::Newline {
            self.advance();
        }

        // Parse the indented block of field declarations
        self.expect(Token::Indent)?;

        let mut fields = Vec::new();
        while self.peek() != &Token::Dedent && self.peek() != &Token::EOF {
            if self.peek() == &Token::Newline {
                self.advance();
                continue;
            }

            // Expect: field <name> as <Type>
            self.expect(Token::Field)?;
            let (field_name, _) = self.expect_identifier()?;
            self.expect(Token::As)?;
            let field_type = self.parse_type_annotation()?
;
            // Parse optional default value: is <expr>
            let default_val = if self.peek() == &Token::Is {
                self.advance(); // consume 'is'
                Some(self.parse_expr(Precedence::Lowest)?)
            } else {
                None
            };
            fields.push((field_name, field_type, default_val));

            if self.peek() == &Token::Newline {
                self.advance();
            }
        }

        if self.peek() == &Token::Dedent {
            self.advance();
        }

        Ok(StructDef { name, type_params, fields })
    }

    fn parse_enum_def(&mut self) -> Result<EnumDef, ParseError> {
        self.expect(Token::Define)?;
        self.expect(Token::Choice)?;
        let (name, _) = self.expect_identifier()?;

        // Parse optional type parameters: of T
        let mut type_params = Vec::new();
        if self.peek() == &Token::Of {
            self.advance();
            let (tp, _) = self.expect_identifier()?;
            type_params.push(tp);
            while self.peek() == &Token::And {
                self.advance();
                let (tp2, _) = self.expect_identifier()?;
                type_params.push(tp2);
            }
        }

        self.expect(Token::Colon)?;

        if self.peek() == &Token::Newline {
            self.advance();
        }

        self.expect(Token::Indent)?;

        let mut variants = Vec::new();
        while self.peek() != &Token::Dedent && self.peek() != &Token::EOF {
            if self.peek() == &Token::Newline {
                self.advance();
                continue;
            }

            // Expect: variant <Name> with <field> as <Type> and <field> as <Type> ...
            // Or:     variant <Name>
            self.expect(Token::Variant)?;
            let (variant_name, _) = self.expect_identifier()?;

            let mut fields = Vec::new();
            if self.peek() == &Token::With {
                self.advance(); // consume "with"
                let (field_name, _) = self.expect_identifier()?;
                self.expect(Token::As)?;
                let field_type = self.parse_type_annotation()?;
                fields.push((field_name, field_type));

                while self.peek() == &Token::And {
                    self.advance(); // consume "and"
                    let (next_field, _) = self.expect_identifier()?;
                    self.expect(Token::As)?;
                    let next_type = self.parse_type_annotation()?;
                    fields.push((next_field, next_type));
                }
            }

            variants.push((variant_name, fields));

            if self.peek() == &Token::Newline {
                self.advance();
            }
        }

        if self.peek() == &Token::Dedent {
            self.advance();
        }

        Ok(EnumDef { name, type_params, variants })
    }

    fn parse_function(&mut self, is_async: bool) -> Result<Function, ParseError> {
        self.expect(Token::Define)?;
        let (name, _name_span) = self.expect_identifier()?;
        
        let mut params = Vec::new();
        if self.peek() == &Token::With {
            self.advance(); // consume "with"
            let mut is_borrow = false;
            if self.peek() == &Token::Borrow {
                self.advance();
                is_borrow = true;
            }
            let (first_param, _) = self.expect_identifier()?;
            let mut first_type = None;
            if self.peek() == &Token::As {
                self.advance(); // consume "as"
                first_type = Some(self.parse_type_annotation()?);
            }
            params.push((first_param, is_borrow, first_type));
            
            while self.peek() == &Token::And {
                self.advance(); // consume "and"
                let mut next_is_borrow = false;
                if self.peek() == &Token::Borrow {
                    self.advance();
                    next_is_borrow = true;
                }
                let (next_param, _) = self.expect_identifier()?;
                let mut next_type = None;
                if self.peek() == &Token::As {
                    self.advance(); // consume "as"
                    next_type = Some(self.parse_type_annotation()?);
                }
                params.push((next_param, next_is_borrow, next_type));
            }
        }

        let mut return_type = None;
        if self.peek() == &Token::Returning {
            self.advance(); // consume "returning"
            return_type = Some(self.parse_type_annotation()?);
        }

        self.expect(Token::Colon)?;
        
        // Skip optional newline before block
        if self.peek() == &Token::Newline {
            self.advance();
        }

        let body = self.parse_block()?;

        Ok(Function { name, params, return_type, body, is_async })
    }

    fn parse_method_def(&mut self) -> Result<MethodDef, ParseError> {
        self.expect(Token::Define)?;
        let (name, _) = self.expect_identifier()?;
        self.expect(Token::On)?;
        let (struct_name, _) = self.expect_identifier()?;

        let mut params = Vec::new();
        if self.peek() == &Token::With {
            self.advance(); // consume "with"
            let mut is_borrow = false;
            if self.peek() == &Token::Borrow {
                self.advance();
                is_borrow = true;
            }
            let (first_param, _) = self.expect_identifier()?;
            let mut first_type = None;
            if self.peek() == &Token::As {
                self.advance();
                first_type = Some(self.parse_type_annotation()?);
            }
            params.push((first_param, is_borrow, first_type));
            while self.peek() == &Token::And {
                self.advance();
                let mut next_is_borrow = false;
                if self.peek() == &Token::Borrow {
                    self.advance();
                    next_is_borrow = true;
                }
                let (next_param, _) = self.expect_identifier()?;
                let mut next_type = None;
                if self.peek() == &Token::As {
                    self.advance();
                    next_type = Some(self.parse_type_annotation()?);
                }
                params.push((next_param, next_is_borrow, next_type));
            }
        }

        let mut return_type = None;
        if self.peek() == &Token::Returning {
            self.advance();
            return_type = Some(self.parse_type_annotation()?);
        }

        self.expect(Token::Colon)?;
        if self.peek() == &Token::Newline {
            self.advance();
        }

        let body = self.parse_block()?;
        Ok(MethodDef { name, struct_name, params, return_type, body })
    }

    fn parse_trait_def(&mut self) -> Result<TraitDef, ParseError> {
        self.expect(Token::Define)?;
        self.expect(Token::Trait)?;
        let (name, _) = self.expect_identifier()?;
        self.expect(Token::Colon)?;
        if self.peek() == &Token::Newline {
            self.advance();
        }

        self.expect(Token::Indent)?;
        let mut method_signatures = Vec::new();

        while self.peek() != &Token::Dedent && self.peek() != &Token::EOF {
            if self.peek() == &Token::Newline {
                self.advance();
                continue;
            }

            self.expect(Token::Define)?;
            let (method_name, _) = self.expect_identifier()?;

            let mut params = Vec::new();
            if self.peek() == &Token::With {
                self.advance();
                // Parse params (skip "self" as it's implicit)
                let (first_param, _) = self.expect_identifier()?;
                if first_param != "self" {
                    let mut first_type = None;
                    if self.peek() == &Token::As {
                        self.advance();
                        first_type = Some(self.parse_type_annotation()?);
                    }
                    params.push((first_param, false, first_type));
                }
                while self.peek() == &Token::And {
                    self.advance();
                    let (next_param, _) = self.expect_identifier()?;
                    let mut next_type = None;
                    if self.peek() == &Token::As {
                        self.advance();
                        next_type = Some(self.parse_type_annotation()?);
                    }
                    params.push((next_param, false, next_type));
                }
            }

            let mut return_type = None;
            if self.peek() == &Token::Returning {
                self.advance();
                return_type = Some(self.parse_type_annotation()?);
            }

            method_signatures.push((method_name, params, return_type));

            if self.peek() == &Token::Newline {
                self.advance();
            }
        }

        if self.peek() == &Token::Dedent {
            self.advance();
        }

        Ok(TraitDef { name, method_signatures })
    }

    fn parse_trait_impl(&mut self) -> Result<TraitImpl, ParseError> {
        self.expect(Token::Implement)?;
        let (trait_name, _) = self.expect_identifier()?;
        self.expect(Token::For)?;
        let (for_type, _) = self.expect_identifier()?;
        self.expect(Token::Colon)?;
        if self.peek() == &Token::Newline {
            self.advance();
        }

        self.expect(Token::Indent)?;
        let mut methods = Vec::new();

        while self.peek() != &Token::Dedent && self.peek() != &Token::EOF {
            if self.peek() == &Token::Newline {
                self.advance();
                continue;
            }

            let func = self.parse_function(false)?;
            methods.push(func);
        }

        if self.peek() == &Token::Dedent {
            self.advance();
        }

        Ok(TraitImpl { trait_name, for_type, methods })
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        self.expect(Token::Indent)?;
        
        let mut statements = Vec::new();
        while self.peek() != &Token::Dedent && self.peek() != &Token::EOF {
            if self.peek() == &Token::Newline {
                self.advance();
                continue;
            }
            statements.push(self.parse_statement()?);
        }

        if self.peek() == &Token::Dedent {
            self.advance();
        } else {
            return Err(ParseError {
                message: "Expected end of block (dedent)".to_string(),
                span: self.peek_span(),
            });
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        let (tok, span) = self.advance();
        match tok {
            Token::Set => {
                let (var_name, _) = self.expect_identifier()?;

                // Check for field set: "set obj.field to expr"
                if self.peek() == &Token::Dot {
                    self.advance(); // consume "."
                    let (field_name, _) = self.expect_identifier()?;

                    // Support chained field access: set obj.field1.field2 to expr
                    let mut obj_expr = Expr::new(ExprNode::Identifier(var_name));
                    let mut last_field = field_name;

                    while self.peek() == &Token::Dot {
                        self.advance(); // consume "."
                        obj_expr = Expr::new(ExprNode::FieldAccess(Box::new(obj_expr), last_field));
                        let (next_field, _) = self.expect_identifier()?;
                        last_field = next_field;
                    }

                    self.expect(Token::To)?;
                    let expr = self.parse_expr(Precedence::Lowest)?;

                    if self.peek() == &Token::Newline {
                        self.advance();
                    }

                    return Ok(Stmt::new(StmtNode::FieldSet(Box::new(obj_expr), last_field, expr)));
                }

                // Optionally parse type annotation: "as Type"
                let mut type_ann = None;
                if self.peek() == &Token::As {
                    self.advance(); // consume "as"
                    type_ann = Some(self.parse_type_annotation()?);
                }

                self.expect(Token::To)?;
                let expr = self.parse_expr(Precedence::Lowest)?;
                
                // Allow statement to end with Newline or EOF
                if self.peek() == &Token::Newline {
                    self.advance();
                }
                
                Ok(Stmt::new(StmtNode::Set(var_name, expr, type_ann)))
            }
            Token::If => {
                // Enable context-aware 'and' for the condition expression
                let prev_in_condition = self.in_condition;
                self.in_condition = true;
                let cond = self.parse_expr(Precedence::Lowest)?;
                self.in_condition = prev_in_condition;
                self.expect(Token::Colon)?;
                
                if self.peek() == &Token::Newline {
                    self.advance();
                }
                
                let then_branch = self.parse_block()?;
                
                let mut else_branch = None;
                if self.peek() == &Token::Else {
                    self.advance(); // consume "else"
                    // Support 'else if' chains — if next token is If, parse as nested if
                    if self.peek() == &Token::If {
                        let elif_stmt = self.parse_statement()?;
                        else_branch = Some(vec![elif_stmt]);
                    } else {
                        self.expect(Token::Colon)?;
                        
                        if self.peek() == &Token::Newline {
                            self.advance();
                        }
                        
                        else_branch = Some(self.parse_block()?);
                    }
                }
                
                Ok(Stmt::new(StmtNode::If(cond, then_branch, else_branch)))
            }
            Token::Repeat => {
                self.expect(Token::While)?;
                let prev_in_condition = self.in_condition;
                self.in_condition = true;
                let cond = self.parse_expr(Precedence::Lowest)?;
                self.in_condition = prev_in_condition;
                self.expect(Token::Colon)?;
                
                if self.peek() == &Token::Newline {
                    self.advance();
                }
                
                let body = self.parse_block()?;
                Ok(Stmt::new(StmtNode::RepeatWhile(cond, body)))
            }
            Token::While => {
                let prev_in_condition = self.in_condition;
                self.in_condition = true;
                let cond = self.parse_expr(Precedence::Lowest)?;
                self.in_condition = prev_in_condition;
                self.expect(Token::Colon)?;
                
                if self.peek() == &Token::Newline {
                    self.advance();
                }
                
                let body = self.parse_block()?;
                Ok(Stmt::new(StmtNode::RepeatWhile(cond, body)))
            }
            Token::Return => {
                let expr = self.parse_expr(Precedence::Lowest)?;
                
                if self.peek() == &Token::Newline {
                    self.advance();
                }
                
                Ok(Stmt::new(StmtNode::Return(expr)))
            }
            Token::Display => {
                let expr = self.parse_expr(Precedence::Lowest)?;
                
                if self.peek() == &Token::Newline {
                    self.advance();
                }
                
                Ok(Stmt::new(StmtNode::Display(expr)))
            }
            Token::Spawn => {
                let (func_name, _) = self.expect_identifier()?;
                self.expect(Token::LeftParen)?;
                let mut args = Vec::new();
                if self.peek() != &Token::RightParen {
                    args.push(self.parse_expr(Precedence::Lowest)?);
                    while self.peek() == &Token::And {
                        self.advance(); // consume "and"
                        args.push(self.parse_expr(Precedence::Lowest)?);
                    }
                }
                self.expect(Token::RightParen)?;
                if self.peek() == &Token::Newline {
                    self.advance();
                }
                Ok(Stmt::new(StmtNode::Spawn(func_name, args)))
            }
            Token::Send => {
                let val = self.parse_expr(Precedence::Lowest)?;
                self.expect(Token::To)?;
                let chan = self.parse_expr(Precedence::Lowest)?;
                if self.peek() == &Token::Newline {
                    self.advance();
                }
                Ok(Stmt::new(StmtNode::Send(chan, Box::new(val))))
            }
            Token::Check => {
                // check <expr>:
                //     if VariantName with binding1 and binding2:
                //         <body>
                //     if VariantName:
                //         <body>
                let match_expr = self.parse_expr(Precedence::Lowest)?;
                self.expect(Token::Colon)?;
                if self.peek() == &Token::Newline {
                    self.advance();
                }

                self.expect(Token::Indent)?;
                let mut arms = Vec::new();

                while self.peek() != &Token::Dedent && self.peek() != &Token::EOF {
                    if self.peek() == &Token::Newline {
                        self.advance();
                        continue;
                    }

                    self.expect(Token::If)?;
                    let (variant_name, _) = self.expect_identifier()?;

                    let mut bindings = Vec::new();
                    if self.peek() == &Token::With {
                        self.advance(); // consume "with"
                        let (first_bind, _) = self.expect_identifier()?;
                        bindings.push(first_bind);
                        while self.peek() == &Token::And {
                            self.advance();
                            let (next_bind, _) = self.expect_identifier()?;
                            bindings.push(next_bind);
                        }
                    }

                    self.expect(Token::Colon)?;
                    if self.peek() == &Token::Newline {
                        self.advance();
                    }

                    let body = self.parse_block()?;
                    arms.push((variant_name, bindings, body));
                }

                if self.peek() == &Token::Dedent {
                    self.advance();
                }

                Ok(Stmt::new(StmtNode::Match(match_expr, arms)))
            }
            Token::For => {
                // "for each <var> in <expr>:"
                self.expect(Token::Each)?;
                let (loop_var, _) = self.expect_identifier()?;
                self.expect(Token::In)?;
                let iterable = self.parse_expr(Precedence::Lowest)?;
                self.expect(Token::Colon)?;

                if self.peek() == &Token::Newline {
                    self.advance();
                }

                let body = self.parse_block()?;
                Ok(Stmt::new(StmtNode::ForEach(loop_var, iterable, body)))
            }
            Token::Break => {
                if self.peek() == &Token::Newline {
                    self.advance();
                }
                Ok(Stmt::new(StmtNode::Break))
            }
            Token::Continue => {
                if self.peek() == &Token::Newline {
                    self.advance();
                }
                Ok(Stmt::new(StmtNode::Continue))
            }
            Token::Identifier(_name) => {
                // Expression statement: e.g., obj.method(args)
                // Put the identifier back and parse as expression
                self.pos -= 1; // back up to re-parse the identifier
                let expr = self.parse_expr(Precedence::Lowest)?;
                if self.peek() == &Token::Newline {
                    self.advance();
                }
                Ok(Stmt::new(StmtNode::ExprStmt(expr)))
            }
            other => Err(ParseError {
                message: format!("Unexpected statement start: {:?}", other),
                span,
            }),
        }
    }

    fn parse_expr(&mut self, precedence: Precedence) -> Result<Expr, ParseError> {
        let (prefix_tok, span) = self.advance();
        let mut left = match prefix_tok {
            Token::Integer(val) => Expr::with_span(ExprNode::Integer(val), span),
            Token::Float(val) => Expr::with_span(ExprNode::FloatLiteral(val), span),
            Token::True => Expr::with_span(ExprNode::BoolLiteral(true), span),
            Token::False => Expr::with_span(ExprNode::BoolLiteral(false), span),
            Token::StringLiteral(s) => Expr::with_span(ExprNode::StringLiteral(s), span),
            Token::InterpStringParts(parts) => {
                // Desugar "hello {name}!" into concat(concat("hello " and name) and "!")
                // Build a list of Expr nodes for each part
                let mut exprs: Vec<Expr> = Vec::new();
                for (literal, expr_src) in &parts {
                    if !literal.is_empty() {
                        exprs.push(Expr::with_span(ExprNode::StringLiteral(literal.clone()), span));
                    }
                    if !expr_src.is_empty() {
                        // Parse the expression source as a mini-program
                        let mut mini_lexer = crate::lexer::Lexer::new(expr_src);
                        let mini_tokens = mini_lexer.tokenize().map_err(|e| ParseError {
                            message: format!("Error in interpolated expression '{{{}}}': {}", expr_src, e.message),
                            span,
                        })?;
                        // Build a temporary parser for the expression tokens
                        let mut mini_parser = Parser::new(mini_tokens);
                        let expr_node = mini_parser.parse_expr(Precedence::Lowest)?;
                        // Wrap non-string expressions in int_to_string
                        let wrapped = Expr::with_span(
                            ExprNode::Call("ep_auto_to_string".to_string(), vec![expr_node]),
                            span,
                        );
                        exprs.push(wrapped);
                    }
                }
                // Chain all parts with concat()
                if exprs.is_empty() {
                    Expr::with_span(ExprNode::StringLiteral(String::new()), span)
                } else if exprs.len() == 1 {
                    exprs.pop().unwrap()
                } else {
                    let mut result = exprs.remove(0);
                    for part in exprs {
                        result = Expr::with_span(
                            ExprNode::Call("concat".to_string(), vec![result, part]),
                            span,
                        );
                    }
                    result
                }
            }
            Token::Channel => Expr::with_span(ExprNode::Channel, span),
            Token::Receive => {
                self.expect(Token::From)?;
                let chan = self.parse_expr(Precedence::Lowest)?;
                Expr::with_span(ExprNode::Receive(Box::new(chan)), span)
            }
            Token::Not => {
                let inner = self.parse_expr(Precedence::Product)?;
                Expr::with_span(ExprNode::UnaryNot(Box::new(inner)), span)
            }
            Token::Minus => {
                let inner = self.parse_expr(Precedence::Product)?;
                Expr::with_span(ExprNode::Binary(
                    Box::new(Expr::with_span(ExprNode::Integer(0), span)),
                    Op::Sub,
                    Box::new(inner),
                ), span)
            }
            Token::Await => {
                let inner = self.parse_expr(Precedence::Lowest)?;
                Expr::with_span(ExprNode::Await(Box::new(inner)), span)
            }
            Token::Try => {
                let inner = self.parse_expr(Precedence::Lowest)?;
                Expr::with_span(ExprNode::TryExpr(Box::new(inner)), span)
            }
            Token::Given => {
                // given x and y: (newline indent body dedent)
                let mut params = Vec::new();
                let (first_param, _) = self.expect_identifier()?;
                params.push(first_param);
                while self.peek() == &Token::And {
                    self.advance(); // consume "and"
                    let (next_param, _) = self.expect_identifier()?;
                    params.push(next_param);
                }
                self.expect(Token::Colon)?;
                if self.peek() == &Token::Newline {
                    self.advance();
                }
                // parse_block handles the Indent/Dedent
                let body = self.parse_block()?;
                Expr::with_span(ExprNode::Closure(params, body), span)
            }
            Token::Borrow => {
                let target = self.parse_expr(Precedence::Lowest)?;
                Expr::with_span(ExprNode::Borrow(Box::new(target)), span)
            }
            Token::Create => {
                // "create StructName:" or "create StructName of T and U:" followed by indented field assignments
                let (struct_name, _) = self.expect_identifier()?;

                // Parse optional generic type args: of Int and Str
                if self.peek() == &Token::Of {
                    self.advance(); // consume "of"
                    // Parse and discard type args (used for type checking, not C codegen)
                    let _first = self.parse_type_annotation()?;
                    while self.peek() == &Token::And {
                        self.advance();
                        let _next = self.parse_type_annotation()?;
                    }
                }

                self.expect(Token::Colon)?;

                if self.peek() == &Token::Newline {
                    self.advance();
                }

                // Parse the indented block of field initializers
                self.expect(Token::Indent)?;
                let mut field_inits = Vec::new();
                while self.peek() != &Token::Dedent && self.peek() != &Token::EOF {
                    if self.peek() == &Token::Newline {
                        self.advance();
                        continue;
                    }

                    // Expect: <field_name> is <expr>
                    let (field_name, _) = self.expect_identifier()?;
                    self.expect(Token::Is)?;
                    let value = self.parse_expr(Precedence::Lowest)?;
                    field_inits.push((field_name, value));

                    if self.peek() == &Token::Newline {
                        self.advance();
                    }
                }
                if self.peek() == &Token::Dedent {
                    self.advance();
                }

                Expr::with_span(ExprNode::StructCreate(struct_name, field_inits), span)
            }
            Token::Identifier(name) => {
                // Check if this is an enum variant creation: Name with args
                // We detect this when an identifier is followed by With at expression level
                if self.peek() == &Token::With {
                    // Could be an enum variant creation: VariantName with val1 and val2
                    // But only if the name starts with uppercase (convention)
                    if name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                        self.advance(); // consume "with"
                        let mut args = Vec::new();
                        args.push(self.parse_expr(Precedence::Lowest)?);
                        while self.peek() == &Token::And {
                            self.advance();
                            args.push(self.parse_expr(Precedence::Lowest)?);
                        }
                        // The enum_name is resolved later in codegen; for now store "" as placeholder
                        Expr::with_span(ExprNode::EnumCreate(String::new(), name, args), span)
                    } else {
                        Expr::with_span(ExprNode::Identifier(name), span)
                    }
                } else if self.peek() == &Token::LeftParen {
                    self.advance(); // consume "("
                    self.call_depth += 1;
                    let mut args = Vec::new();
                    if self.peek() != &Token::RightParen {
                        args.push(self.parse_expr(Precedence::Lowest)?);
                        while self.peek() == &Token::And || self.peek() == &Token::Identifier(String::from(",")) {
                            // Let's consume whatever separator is used. We'll support both "and" and standard commas for lists.
                            // In our lexer, we didn't define comma, but we did define Token::And. So let's check for Token::And.
                            if self.peek() == &Token::And {
                                self.advance();
                            }
                            args.push(self.parse_expr(Precedence::Lowest)?);
                        }
                    }
                    self.call_depth -= 1;
                    self.expect(Token::RightParen)?;
                    Expr::with_span(ExprNode::Call(name, args), span)
                } else {
                    Expr::with_span(ExprNode::Identifier(name), span)
                }
            }
            Token::LeftParen => {
                let expr = self.parse_expr(Precedence::Lowest)?;
                self.expect(Token::RightParen)?;
                expr
            }
            Token::Range => {
                // range(start and end) or range(end) - parsed as a function call
                self.expect(Token::LeftParen)?;
                let mut args = Vec::new();
                if self.peek() != &Token::RightParen {
                    args.push(self.parse_expr(Precedence::Lowest)?);
                    while self.peek() == &Token::And {
                        self.advance();
                        args.push(self.parse_expr(Precedence::Lowest)?);
                    }
                }
                self.expect(Token::RightParen)?;
                Expr::with_span(ExprNode::Call("range".to_string(), args), span)
            }
            // Allow keywords as variable names in expression position
            ref kw @ (Token::Choice | Token::Field | Token::Variant | Token::Structure |
            Token::Check | Token::Trait | Token::Implement) => {
                let name = match kw {
                    Token::Choice => "choice", Token::Field => "field",
                    Token::Variant => "variant", Token::Structure => "structure",
                    Token::Check => "check",
                    Token::Trait => "trait", Token::Implement => "implement",
                    _ => unreachable!(),
                };
                Expr::with_span(ExprNode::Identifier(name.to_string()), span)
            }
            other => {
                return Err(ParseError {
                    message: format!("Expected expression, found {:?}", other),
                    span,
                });
            }
        };

        while precedence < self.token_precedence(self.peek()) {
            let next_tok = self.peek().clone();
            
            // Check infix operators
            match next_tok {
                Token::Dot => {
                    self.advance(); // consume "."
                    let (field_or_method, _) = self.expect_identifier()?;
                    // Check if this is a method call: obj.method(...)
                    if self.peek() == &Token::LeftParen {
                        self.advance(); // consume "("
                        self.call_depth += 1;
                        let mut args = Vec::new();
                        if self.peek() != &Token::RightParen {
                            args.push(self.parse_expr(Precedence::Lowest)?);
                            while self.peek() == &Token::And {
                                self.advance();
                                args.push(self.parse_expr(Precedence::Lowest)?);
                            }
                        }
                        self.call_depth -= 1;
                        self.expect(Token::RightParen)?;
                        left = Expr::with_span(ExprNode::MethodCall(Box::new(left), field_or_method, args), span);
                    } else {
                        left = Expr::with_span(ExprNode::FieldAccess(Box::new(left), field_or_method), span);
                    }
                }
                Token::Plus | Token::Minus | Token::Multiply | Token::Divide | Token::Modulo => {
                    let op_tok = self.advance().0;
                    let op = match op_tok {
                        Token::Plus => Op::Add,
                        Token::Minus => Op::Sub,
                        Token::Multiply => Op::Mul,
                        Token::Divide => Op::Div,
                        Token::Modulo => Op::Mod,
                        _ => unreachable!(),
                    };
                    let right = self.parse_expr(self.token_precedence(&op_tok))?;
                    left = Expr::with_span(ExprNode::Binary(Box::new(left), op, Box::new(right)), span);
                }
                Token::LessThan | Token::GreaterThan | Token::LessEqual | Token::GreaterEqual | Token::Equals | Token::NotEquals => {
                    let op_tok = self.advance().0;
                    let op = match op_tok {
                        Token::LessThan => CompOp::LessThan,
                        Token::GreaterThan => CompOp::GreaterThan,
                        Token::LessEqual => CompOp::LessEqual,
                        Token::GreaterEqual => CompOp::GreaterEqual,
                        Token::Equals => CompOp::Equals,
                        Token::NotEquals => CompOp::NotEquals,
                        _ => unreachable!(),
                    };
                    let right = self.parse_expr(self.token_precedence(&op_tok))?;
                    left = Expr::with_span(ExprNode::Comparison(Box::new(left), op, Box::new(right)), span);
                }
                Token::LogicalAnd | Token::LogicalOr => {
                    let op_tok = self.advance().0;
                    let op = match op_tok {
                        Token::LogicalAnd => LogicalOp::And,
                        Token::LogicalOr => LogicalOp::Or,
                        _ => unreachable!(),
                    };
                    let right = self.parse_expr(self.token_precedence(&op_tok))?;
                    left = Expr::with_span(ExprNode::Logical(Box::new(left), op, Box::new(right)), span);
                }
                // Context-aware 'and' in conditions: treat as logical AND
                Token::And if self.in_condition && self.call_depth == 0 => {
                    self.advance(); // consume 'and'
                    let right = self.parse_expr(Precedence::LogicalAnd)?;
                    left = Expr::with_span(ExprNode::Logical(Box::new(left), LogicalOp::And, Box::new(right)), span);
                }
                _ => break,
            }
        }

        Ok(left)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse_helper(src: &str) -> Program {
        let mut lexer = Lexer::new(src);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse_program().unwrap()
    }

    #[test]
    fn test_parse_math_precedence() {
        let program = parse_helper(
            "define main:\n    set x to 1 plus 2 multiplied by 3 minus 4 divided by 2\n    return x"
        );
        assert_eq!(program.functions.len(), 1);
        let main_func = &program.functions[0];
        assert_eq!(main_func.name, "main");
        assert_eq!(main_func.params.len(), 0);
        assert_eq!(main_func.body.len(), 2);

        // check set statement: x = (1 + (2 * 3)) - (4 / 2)
        if let Stmt { node: StmtNode::Set(var, expr, _type_ann), .. } = &main_func.body[0]  {
            assert_eq!(var, "x");
            // Check that multiplication and division have higher precedence than plus and minus
            assert!(matches!(expr.node, ExprNode::Binary(_, Op::Sub, _)));
        } else {
            panic!("Expected Set statement");
        }
    }

    #[test]
    fn test_parse_function_params() {
        let program = parse_helper(
            "define add with a and b:\n    return a plus b"
        );
        assert_eq!(program.functions.len(), 1);
        let func = &program.functions[0];
        assert_eq!(func.name, "add");
        assert_eq!(func.params, vec![("a".to_string(), false, None), ("b".to_string(), false, None)]);
    }

    #[test]
    fn test_parse_if_else() {
        let program = parse_helper(
            "define main:\n    if 5 is less than 10:\n        display \"yes\"\n    else:\n        display \"no\""
        );
        let main_func = &program.functions[0];
        assert_eq!(main_func.body.len(), 1);
        if let Stmt { node: StmtNode::If(cond, then_b, else_b), .. } = &main_func.body[0] {
            assert!(matches!(cond.node, ExprNode::Comparison(_, CompOp::LessThan, _)));
            assert_eq!(then_b.len(), 1);
            assert_eq!(else_b.as_ref().unwrap().len(), 1);
        } else {
            panic!("Expected If statement");
        }
    }

    #[test]
    fn test_parse_repeat_while() {
        let program = parse_helper(
            "define main:\n    repeat while x is less than 10:\n        set x to x plus 1"
        );
        let main_func = &program.functions[0];
        assert_eq!(main_func.body.len(), 1);
        if let Stmt { node: StmtNode::RepeatWhile(cond, body), .. } = &main_func.body[0]  {
            assert!(matches!(cond.node, ExprNode::Comparison(_, CompOp::LessThan, _)));
            assert_eq!(body.len(), 1);
            assert!(matches!(body[0].node, StmtNode::Set(_, _, _)));
        } else {
            panic!("Expected RepeatWhile statement");
        }
    }

    #[test]
    fn test_parse_logical() {
        let program = parse_helper(
            "define main:\n    if x equals 10 and also y equals 20:\n        display x"
        );
        let main_func = &program.functions[0];
        assert_eq!(main_func.body.len(), 1);
        if let Stmt { node: StmtNode::If(cond, then_b, _), .. } = &main_func.body[0]  {
            assert!(matches!(cond.node, ExprNode::Logical(_, LogicalOp::And, _)));
            assert_eq!(then_b.len(), 1);
        } else {
            panic!("Expected If statement");
        }
    }

    #[test]
    fn test_parse_shorthand_while() {
        let program = parse_helper(
            "define main:\n    while x < 10:\n        set x to x + 1"
        );
        let main_func = &program.functions[0];
        assert_eq!(main_func.body.len(), 1);
        if let Stmt { node: StmtNode::RepeatWhile(cond, body), .. } = &main_func.body[0]  {
            assert!(matches!(cond.node, ExprNode::Comparison(_, CompOp::LessThan, _)));
            assert_eq!(body.len(), 1);
        } else {
            panic!("Expected RepeatWhile statement");
        }
    }

    #[test]
    fn test_parse_struct_definition() {
        let program = parse_helper(
            "define structure User:\n    field name as Str\n    field age as Int"
        );
        assert_eq!(program.struct_defs.len(), 1);
        let sd = &program.struct_defs[0];
        assert_eq!(sd.name, "User");
        assert_eq!(sd.fields.len(), 2);
        assert_eq!(sd.fields[0].0, "name");
        assert_eq!(sd.fields[0].1, TypeAnnotation::Str);
        assert_eq!(sd.fields[1].0, "age");
        assert_eq!(sd.fields[1].1, TypeAnnotation::Int);
    }

    #[test]
    fn test_parse_struct_create_and_field_access() {
        let program = parse_helper(
            "define main:\n    set user to create User:\n        name is \"Alice\"\n        age is 30\n    display user.name\n    return 0"
        );
        let main_func = &program.functions[0];
        assert_eq!(main_func.body.len(), 3);

        // First statement: set user to create User: ...
        if let Stmt { node: StmtNode::Set(name, expr, _type_ann), .. } = &main_func.body[0] {
            assert_eq!(name, "user");
            if let ExprNode::StructCreate(sname, fields) = &expr.node {
                assert_eq!(sname, "User");
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0].0, "name");
                assert_eq!(fields[1].0, "age");
            } else {
                panic!("Expected StructCreate");
            }
        } else {
            panic!("Expected Set with StructCreate");
        }

        // Second statement: display user.name
        if let Stmt { node: StmtNode::Display(expr), .. } = &main_func.body[1] {
            if let ExprNode::FieldAccess(obj, field) = &expr.node {
                assert!(matches!(obj.node, ExprNode::Identifier(ref n) if n == "user"));
                assert_eq!(field, "name");
            } else {
                panic!("Expected FieldAccess");
            }
        } else {
            panic!("Expected Display with FieldAccess");
        }
    }

    #[test]
    fn test_parse_field_set() {
        let program = parse_helper(
            "define main:\n    set user.score to 100\n    return 0"
        );
        let main_func = &program.functions[0];

        if let Stmt { node: StmtNode::FieldSet(obj, field, val), .. } = &main_func.body[0] {
            assert!(matches!(obj.node, ExprNode::Identifier(ref n) if n == "user"));
            assert_eq!(field, "score");
            assert!(matches!(val.node, ExprNode::Integer(100)));
        } else {
            panic!("Expected FieldSet statement");
        }
    }

    #[test]
    fn test_parse_typed_params() {
        let program = parse_helper(
            "define add with a as Int and b as Int:\n    return a plus b"
        );
        assert_eq!(program.functions.len(), 1);
        let func = &program.functions[0];
        assert_eq!(func.name, "add");
        assert_eq!(func.params, vec![
            ("a".to_string(), false, Some(TypeAnnotation::Int)),
            ("b".to_string(), false, Some(TypeAnnotation::Int)),
        ]);
        assert_eq!(func.return_type, None);
    }

    #[test]
    fn test_parse_return_type() {
        let program = parse_helper(
            "define add with a as Int and b as Int returning Int:\n    return a plus b"
        );
        let func = &program.functions[0];
        assert_eq!(func.name, "add");
        assert_eq!(func.return_type, Some(TypeAnnotation::Int));
        assert_eq!(func.params, vec![
            ("a".to_string(), false, Some(TypeAnnotation::Int)),
            ("b".to_string(), false, Some(TypeAnnotation::Int)),
        ]);
    }

    #[test]
    fn test_parse_typed_set() {
        let program = parse_helper(
            "define main:\n    set x as Int to 42\n    display x\n    return 0"
        );
        let main_func = &program.functions[0];
        assert_eq!(main_func.body.len(), 3);

        if let Stmt { node: StmtNode::Set(var, expr, type_ann), .. } = &main_func.body[0] {
            assert_eq!(var, "x");
            assert!(matches!(expr.node, ExprNode::Integer(42)));
            assert_eq!(*type_ann, Some(TypeAnnotation::Int));
        } else {
            panic!("Expected Set statement with type annotation");
        }
    }

    #[test]
    fn test_parse_mixed_typed_untyped_params() {
        let program = parse_helper(
            "define process with name as Str and count:\n    display name\n    return count"
        );
        let func = &program.functions[0];
        assert_eq!(func.params, vec![
            ("name".to_string(), false, Some(TypeAnnotation::Str)),
            ("count".to_string(), false, None),
        ]);
    }

    #[test]
    fn test_parse_returning_dynstr() {
        let program = parse_helper(
            "define greet with name as Str returning DynStr:\n    return name"
        );
        let func = &program.functions[0];
        assert_eq!(func.return_type, Some(TypeAnnotation::DynStr));
        assert_eq!(func.params, vec![
            ("name".to_string(), false, Some(TypeAnnotation::Str)),
        ]);
    }

    #[test]
    fn test_parse_no_params_with_return_type() {
        let program = parse_helper(
            "define get_value returning Int:\n    return 42"
        );
        let func = &program.functions[0];
        assert_eq!(func.name, "get_value");
        assert_eq!(func.params.len(), 0);
        assert_eq!(func.return_type, Some(TypeAnnotation::Int));
    }

    #[test]
    fn test_parse_struct_typed_param() {
        let program = parse_helper(
            "define show_user with user as User:\n    display user\n    return 0"
        );
        let func = &program.functions[0];
        assert_eq!(func.params, vec![
            ("user".to_string(), false, Some(TypeAnnotation::UserDefined("User".to_string()))),
        ]);
    }
}
