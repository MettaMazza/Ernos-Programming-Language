use crate::token::{Span, Token};
use crate::ast::{Program, Function, Stmt, Expr, Op, CompOp, LogicalOp};

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
}

pub struct Parser {
    tokens: Vec<(Token, Span)>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<(Token, Span)>) -> Self {
        Self { tokens, pos: 0 }
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
        let (actual, span) = self.advance();
        if let Token::Identifier(name) = actual {
            Ok((name, span))
        } else {
            Err(ParseError {
                message: format!("Expected identifier, found {:?}", actual),
                span,
            })
        }
    }

    fn token_precedence(&self, tok: &Token) -> Precedence {
        match tok {
            Token::LogicalOr => Precedence::LogicalOr,
            Token::LogicalAnd => Precedence::LogicalAnd,
            Token::LessThan | Token::GreaterThan | Token::Equals | Token::NotEquals => Precedence::Comparison,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Multiply | Token::Divide => Precedence::Product,
            Token::LeftParen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut imports = Vec::new();
        let mut functions = Vec::new();

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
            } else if self.peek() == &Token::Define {
                let func = self.parse_function()?;
                functions.push(func);
            } else {
                return Err(ParseError {
                    message: format!("Unexpected token at top level: {:?}", self.peek()),
                    span: self.peek_span(),
                });
            }
        }

        Ok(Program { imports, functions })
    }

    fn parse_function(&mut self) -> Result<Function, ParseError> {
        self.expect(Token::Define)?;
        let (name, _name_span) = self.expect_identifier()?;
        
        let mut params = Vec::new();
        if self.peek() == &Token::With {
            self.advance(); // consume "with"
            let (first_param, _) = self.expect_identifier()?;
            params.push(first_param);
            
            while self.peek() == &Token::And {
                self.advance(); // consume "and"
                let (next_param, _) = self.expect_identifier()?;
                params.push(next_param);
            }
        }

        self.expect(Token::Colon)?;
        
        // Skip optional newline before block
        if self.peek() == &Token::Newline {
            self.advance();
        }

        let body = self.parse_block()?;

        Ok(Function { name, params, body })
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
                self.expect(Token::To)?;
                let expr = self.parse_expr(Precedence::Lowest)?;
                
                // Allow statement to end with Newline or EOF
                if self.peek() == &Token::Newline {
                    self.advance();
                }
                
                Ok(Stmt::Set(var_name, expr))
            }
            Token::If => {
                let cond = self.parse_expr(Precedence::Lowest)?;
                self.expect(Token::Colon)?;
                
                if self.peek() == &Token::Newline {
                    self.advance();
                }
                
                let then_branch = self.parse_block()?;
                
                let mut else_branch = None;
                if self.peek() == &Token::Else {
                    self.advance(); // consume "else"
                    self.expect(Token::Colon)?;
                    
                    if self.peek() == &Token::Newline {
                        self.advance();
                    }
                    
                    else_branch = Some(self.parse_block()?);
                }
                
                Ok(Stmt::If(cond, then_branch, else_branch))
            }
            Token::Repeat => {
                self.expect(Token::While)?;
                let cond = self.parse_expr(Precedence::Lowest)?;
                self.expect(Token::Colon)?;
                
                if self.peek() == &Token::Newline {
                    self.advance();
                }
                
                let body = self.parse_block()?;
                Ok(Stmt::RepeatWhile(cond, body))
            }
            Token::While => {
                let cond = self.parse_expr(Precedence::Lowest)?;
                self.expect(Token::Colon)?;
                
                if self.peek() == &Token::Newline {
                    self.advance();
                }
                
                let body = self.parse_block()?;
                Ok(Stmt::RepeatWhile(cond, body))
            }
            Token::Return => {
                let expr = self.parse_expr(Precedence::Lowest)?;
                
                if self.peek() == &Token::Newline {
                    self.advance();
                }
                
                Ok(Stmt::Return(expr))
            }
            Token::Display => {
                let expr = self.parse_expr(Precedence::Lowest)?;
                
                if self.peek() == &Token::Newline {
                    self.advance();
                }
                
                Ok(Stmt::Display(expr))
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
            Token::Integer(val) => Expr::Integer(val),
            Token::StringLiteral(s) => Expr::StringLiteral(s),
            Token::Identifier(name) => {
                // If next is a function call
                if self.peek() == &Token::LeftParen {
                    self.advance(); // consume "("
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
                    self.expect(Token::RightParen)?;
                    Expr::Call(name, args)
                } else {
                    Expr::Identifier(name)
                }
            }
            Token::LeftParen => {
                let expr = self.parse_expr(Precedence::Lowest)?;
                self.expect(Token::RightParen)?;
                expr
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
                Token::Plus | Token::Minus | Token::Multiply | Token::Divide => {
                    let op_tok = self.advance().0;
                    let op = match op_tok {
                        Token::Plus => Op::Add,
                        Token::Minus => Op::Sub,
                        Token::Multiply => Op::Mul,
                        Token::Divide => Op::Div,
                        _ => unreachable!(),
                    };
                    let right = self.parse_expr(self.token_precedence(&op_tok))?;
                    left = Expr::Binary(Box::new(left), op, Box::new(right));
                }
                Token::LessThan | Token::GreaterThan | Token::Equals | Token::NotEquals => {
                    let op_tok = self.advance().0;
                    let op = match op_tok {
                        Token::LessThan => CompOp::LessThan,
                        Token::GreaterThan => CompOp::GreaterThan,
                        Token::Equals => CompOp::Equals,
                        Token::NotEquals => CompOp::NotEquals,
                        _ => unreachable!(),
                    };
                    let right = self.parse_expr(self.token_precedence(&op_tok))?;
                    left = Expr::Comparison(Box::new(left), op, Box::new(right));
                }
                Token::LogicalAnd | Token::LogicalOr => {
                    let op_tok = self.advance().0;
                    let op = match op_tok {
                        Token::LogicalAnd => LogicalOp::And,
                        Token::LogicalOr => LogicalOp::Or,
                        _ => unreachable!(),
                    };
                    let right = self.parse_expr(self.token_precedence(&op_tok))?;
                    left = Expr::Logical(Box::new(left), op, Box::new(right));
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
        if let Stmt::Set(var, expr) = &main_func.body[0] {
            assert_eq!(var, "x");
            // Check that multiplication and division have higher precedence than plus and minus
            assert!(matches!(expr, Expr::Binary(_, Op::Sub, _)));
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
        assert_eq!(func.params, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn test_parse_if_else() {
        let program = parse_helper(
            "define main:\n    if 5 is less than 10:\n        display \"yes\"\n    else:\n        display \"no\""
        );
        let main_func = &program.functions[0];
        assert_eq!(main_func.body.len(), 1);
        if let Stmt::If(cond, then_b, else_b) = &main_func.body[0] {
            assert_eq!(
                *cond,
                Expr::Comparison(
                    Box::new(Expr::Integer(5)),
                    CompOp::LessThan,
                    Box::new(Expr::Integer(10))
                )
            );
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
        if let Stmt::RepeatWhile(cond, body) = &main_func.body[0] {
            assert!(matches!(cond, Expr::Comparison(_, CompOp::LessThan, _)));
            assert_eq!(body.len(), 1);
            assert!(matches!(body[0], Stmt::Set(_, _)));
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
        if let Stmt::If(cond, then_b, _) = &main_func.body[0] {
            assert!(matches!(cond, Expr::Logical(_, LogicalOp::And, _)));
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
        if let Stmt::RepeatWhile(cond, body) = &main_func.body[0] {
            assert!(matches!(cond, Expr::Comparison(_, CompOp::LessThan, _)));
            assert_eq!(body.len(), 1);
        } else {
            panic!("Expected RepeatWhile statement");
        }
    }
}
