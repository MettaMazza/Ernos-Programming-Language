use crate::token::{Span, Token};

#[derive(Debug)]
pub struct LexError {
    pub message: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
enum RawToken {
    Word(String),
    Number(i64),
    StringVal(String),
    Symbol(char),
    Spaces(usize),
    Newline,
    Comment,
}

pub struct Lexer<'a> {
    _input: &'a str,
    chars: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            _input: input,
            chars: input.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        if self.pos < self.chars.len() {
            Some(self.chars[self.pos])
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            self.pos += 1;
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            Some(c)
        } else {
            None
        }
    }

    // Pass 1: Lex into raw tokens
    fn lex_raw(&mut self) -> Result<Vec<(RawToken, Span)>, LexError> {
        let mut raw_tokens = Vec::new();

        while let Some(c) = self.peek() {
            let span = Span::new(self.line, self.col);
            if c == '\n' {
                self.advance();
                raw_tokens.push((RawToken::Newline, span));
            } else if c == '\r' {
                self.advance();
                if self.peek() == Some('\n') {
                    self.advance();
                }
                raw_tokens.push((RawToken::Newline, span));
            } else if c == ' ' || c == '\t' {
                let mut count = 0;
                while let Some(ch) = self.peek() {
                    if ch == ' ' {
                        count += 1;
                        self.advance();
                    } else if ch == '\t' {
                        count += 4; // Treat tab as 4 spaces
                        self.advance();
                    } else {
                        break;
                    }
                }
                raw_tokens.push((RawToken::Spaces(count), span));
            } else if c == '#' {
                // Comment
                self.advance();
                while let Some(ch) = self.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    self.advance();
                }
                raw_tokens.push((RawToken::Comment, span));
            } else if c.is_ascii_digit() {
                let mut val = 0;
                while let Some(ch) = self.peek() {
                    if ch.is_ascii_digit() {
                        val = val * 10 + (ch.to_digit(10).unwrap() as i64);
                        self.advance();
                    } else {
                        break;
                    }
                }
                raw_tokens.push((RawToken::Number(val), span));
            } else if c.is_alphabetic() || c == '_' {
                let mut s = String::new();
                while let Some(ch) = self.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        s.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }
                raw_tokens.push((RawToken::Word(s), span));
            } else if c == '"' {
                self.advance(); // consume open quote
                let mut s = String::new();
                let mut closed = false;
                while let Some(ch) = self.peek() {
                    if ch == '"' {
                        self.advance();
                        closed = true;
                        break;
                    } else if ch == '\\' {
                        self.advance(); // consume backslash
                        match self.peek() {
                            Some('n') => {
                                s.push('\n');
                                self.advance();
                            }
                            Some('t') => {
                                s.push('\t');
                                self.advance();
                            }
                            Some('r') => {
                                s.push('\r');
                                self.advance();
                            }
                            Some('"') => {
                                s.push('"');
                                self.advance();
                            }
                            Some('\\') => {
                                s.push('\\');
                                self.advance();
                            }
                            Some(other) => {
                                s.push('\\');
                                s.push(other);
                                self.advance();
                            }
                            None => {
                                return Err(LexError {
                                    message: "Unterminated string literal at escape sequence".to_string(),
                                    span,
                                });
                            }
                        }
                    } else if ch == '\n' || ch == '\r' {
                        return Err(LexError {
                            message: "Unterminated string literal".to_string(),
                            span,
                        });
                    } else {
                        s.push(ch);
                        self.advance();
                    }
                }
                if !closed {
                    return Err(LexError {
                        message: "Unterminated string literal".to_string(),
                        span,
                    });
                }
                raw_tokens.push((RawToken::StringVal(s), span));
            } else if c == ':' || c == '(' || c == ')' || c == '+' || c == '-' || c == '*' || c == '/'
                   || c == '<' || c == '>' || c == '&' || c == '|' || c == '=' || c == '!' {
                self.advance();
                raw_tokens.push((RawToken::Symbol(c), span));
            } else {
                return Err(LexError {
                    message: format!("Unexpected character: '{}'", c),
                    span,
                });
            }
        }

        Ok(raw_tokens)
    }

    // Pass 2: Combine keywords, handle indentation
    pub fn tokenize(&mut self) -> Result<Vec<(Token, Span)>, LexError> {
        let raw = self.lex_raw()?;
        let mut tokens = Vec::new();
        let mut indent_stack = vec![0];
        
        let mut i = 0;
        let mut at_line_start = true;

        // We process line-by-line, handling indentation at the start of each line
        while i < raw.len() {
            if at_line_start {
                at_line_start = false;

                // Let's look ahead to find if this line has indentation and if it's empty
                let mut spaces = 0;
                let mut temp_i = i;
                
                // Skip initial spaces to find the actual content of the line
                if temp_i < raw.len() {
                    if let (RawToken::Spaces(s), _) = &raw[temp_i] {
                        spaces = *s;
                        temp_i += 1;
                    }
                }

                // If the line is empty (contains only comments, newlines, or nothing), we ignore its indentation
                let is_empty_line = temp_i >= raw.len() 
                    || matches!(raw[temp_i].0, RawToken::Newline | RawToken::Comment);

                if !is_empty_line {
                    let last_indent = *indent_stack.last().unwrap();
                    if spaces > last_indent {
                        indent_stack.push(spaces);
                        let span = raw[i].1.clone();
                        tokens.push((Token::Indent, span));
                    } else if spaces < last_indent {
                        while let Some(&top) = indent_stack.last() {
                            if spaces < top {
                                indent_stack.pop();
                                let span = raw[i].1.clone();
                                tokens.push((Token::Dedent, span));
                            } else {
                                break;
                            }
                        }
                        if *indent_stack.last().unwrap() != spaces {
                            return Err(LexError {
                                message: format!(
                                    "Indentation error: expected matching level, found {} spaces",
                                    spaces
                                ),
                                span: raw[i].1.clone(),
                            });
                        }
                    }
                }
                
                // Advance past spaces if we processed them
                if i < raw.len() {
                    if let (RawToken::Spaces(_), _) = &raw[i] {
                        i += 1;
                        continue;
                    }
                }
            }

            if i >= raw.len() {
                break;
            }

            let (raw_tok, span) = &raw[i];
            
            match raw_tok {
                RawToken::Comment => {
                    i += 1;
                }
                RawToken::Newline => {
                    // Only push newline if the last token wasn't already a newline
                    // (and ignore newlines at the start of the file)
                    if let Some((Token::Newline, _)) = tokens.last() {
                        // skip duplicate
                    } else if !tokens.is_empty() {
                        tokens.push((Token::Newline, span.clone()));
                    }
                    at_line_start = true;
                    i += 1;
                }
                RawToken::Spaces(_) => {
                    // Internal spaces (not at line start) are skipped
                    i += 1;
                }
                RawToken::Number(val) => {
                    tokens.push((Token::Integer(*val), span.clone()));
                    i += 1;
                }
                RawToken::StringVal(s) => {
                    tokens.push((Token::StringLiteral(s.clone()), span.clone()));
                    i += 1;
                }
                RawToken::Symbol(c) => {
                    let mut tok = None;
                    
                    // Lookahead helper to see if we match double-character symbols
                    if i + 1 < raw.len() {
                        if let (RawToken::Symbol(next_c), _) = &raw[i + 1] {
                            match (*c, *next_c) {
                                ('=', '=') => {
                                    tok = Some(Token::Equals);
                                    i += 2;
                                }
                                ('!', '=') => {
                                    tok = Some(Token::NotEquals);
                                    i += 2;
                                }
                                ('&', '&') => {
                                    tok = Some(Token::LogicalAnd);
                                    i += 2;
                                }
                                ('|', '|') => {
                                    tok = Some(Token::LogicalOr);
                                    i += 2;
                                }
                                _ => {}
                            }
                        }
                    }
                    
                    // If not combined, match single characters
                    if tok.is_none() {
                        let t = match c {
                            ':' => Token::Colon,
                            '(' => Token::LeftParen,
                            ')' => Token::RightParen,
                            '+' => Token::Plus,
                            '-' => Token::Minus,
                            '*' => Token::Multiply,
                            '/' => Token::Divide,
                            '<' => Token::LessThan,
                            '>' => Token::GreaterThan,
                            _ => {
                                return Err(LexError {
                                    message: format!("Unexpected symbol character: '{}'", c),
                                    span: span.clone(),
                                });
                            }
                        };
                        tok = Some(t);
                        i += 1;
                    }
                    
                    tokens.push((tok.unwrap(), span.clone()));
                }
                RawToken::Word(word) => {
                    // Lookahead helper to see if we match a multi-word phrase
                    let match_phrase = |mut next_idx: usize, expected: &[&str]| -> Option<usize> {
                        for &exp in expected {
                            // Skip any spaces
                            if next_idx < raw.len() {
                                if let (RawToken::Spaces(_), _) = &raw[next_idx] {
                                    next_idx += 1;
                                }
                            }
                            if next_idx >= raw.len() {
                                return None;
                            }
                            if let (RawToken::Word(w), _) = &raw[next_idx] {
                                if w.to_lowercase() == exp {
                                    next_idx += 1;
                                    continue;
                                }
                            }
                            return None;
                        }
                        Some(next_idx)
                    };

                    let w_lower = word.to_lowercase();
                    
                    if w_lower == "multiplied" {
                        if let Some(next_idx) = match_phrase(i + 1, &["by"]) {
                            tokens.push((Token::Multiply, span.clone()));
                            i = next_idx;
                            continue;
                        }
                    } else if w_lower == "divided" {
                        if let Some(next_idx) = match_phrase(i + 1, &["by"]) {
                            tokens.push((Token::Divide, span.clone()));
                            i = next_idx;
                            continue;
                        }
                    } else if w_lower == "is" {
                        if let Some(next_idx) = match_phrase(i + 1, &["less", "than"]) {
                            tokens.push((Token::LessThan, span.clone()));
                            i = next_idx;
                            continue;
                        } else if let Some(next_idx) = match_phrase(i + 1, &["greater", "than"]) {
                            tokens.push((Token::GreaterThan, span.clone()));
                            i = next_idx;
                            continue;
                        } else if let Some(next_idx) = match_phrase(i + 1, &["equal", "to"]) {
                            tokens.push((Token::Equals, span.clone()));
                            i = next_idx;
                            continue;
                        } else if let Some(next_idx) = match_phrase(i + 1, &["not", "equal", "to"]) {
                            tokens.push((Token::NotEquals, span.clone()));
                            i = next_idx;
                            continue;
                        }
                    } else if w_lower == "and" {
                        if let Some(next_idx) = match_phrase(i + 1, &["also"]) {
                            tokens.push((Token::LogicalAnd, span.clone()));
                            i = next_idx;
                            continue;
                        }
                    } else if w_lower == "or" {
                        if let Some(next_idx) = match_phrase(i + 1, &["else"]) {
                            tokens.push((Token::LogicalOr, span.clone()));
                            i = next_idx;
                            continue;
                        }
                    }

                    // Simple single-word keywords
                    let tok = match w_lower.as_str() {
                        "define" => Token::Define,
                        "with" => Token::With,
                        "and" => Token::And,
                        "set" => Token::Set,
                        "to" => Token::To,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "return" => Token::Return,
                        "display" => Token::Display,
                        "repeat" => Token::Repeat,
                        "while" => Token::While,
                        "plus" => Token::Plus,
                        "minus" => Token::Minus,
                        "equals" => Token::Equals,
                        _ => Token::Identifier(word.clone()),
                    };
                    
                    tokens.push((tok, span.clone()));
                    i += 1;
                }
            }
        }

        // Clean up remaining indentation stack
        let end_span = if let Some((_, last_span)) = tokens.last() {
            last_span.clone()
        } else {
            Span::new(self.line, self.col)
        };

        // Pop all remaining indentation
        while indent_stack.len() > 1 {
            indent_stack.pop();
            tokens.push((Token::Dedent, end_span.clone()));
        }

        // Always end with EOF
        tokens.push((Token::EOF, end_span));

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_math_and_set() {
        let mut lexer = Lexer::new("set result to 10 plus 20 multiplied by 5");
        let tokens = lexer.tokenize().unwrap();
        let tok_types: Vec<Token> = tokens.into_iter().map(|(t, _)| t).collect();
        
        assert_eq!(
            tok_types,
            vec![
                Token::Set,
                Token::Identifier("result".to_string()),
                Token::To,
                Token::Integer(10),
                Token::Plus,
                Token::Integer(20),
                Token::Multiply,
                Token::Integer(5),
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_lexer_comparisons() {
        let mut lexer = Lexer::new("if x is less than y:\n    return a is not equal to b");
        let tokens = lexer.tokenize().unwrap();
        let tok_types: Vec<Token> = tokens.into_iter().map(|(t, _)| t).collect();
        
        assert_eq!(
            tok_types,
            vec![
                Token::If,
                Token::Identifier("x".to_string()),
                Token::LessThan,
                Token::Identifier("y".to_string()),
                Token::Colon,
                Token::Newline,
                Token::Indent,
                Token::Return,
                Token::Identifier("a".to_string()),
                Token::NotEquals,
                Token::Identifier("b".to_string()),
                Token::Dedent,
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_lexer_logical_combinators() {
        let mut lexer = Lexer::new("x and also y or else z");
        let tokens = lexer.tokenize().unwrap();
        let tok_types: Vec<Token> = tokens.into_iter().map(|(t, _)| t).collect();
        
        assert_eq!(
            tok_types,
            vec![
                Token::Identifier("x".to_string()),
                Token::LogicalAnd,
                Token::Identifier("y".to_string()),
                Token::LogicalOr,
                Token::Identifier("z".to_string()),
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_lexer_shorthand_symbols() {
        let mut lexer = Lexer::new("set x to a + b * c - d / e");
        let tokens = lexer.tokenize().unwrap();
        let tok_types: Vec<Token> = tokens.into_iter().map(|(t, _)| t).collect();
        
        assert_eq!(
            tok_types,
            vec![
                Token::Set,
                Token::Identifier("x".to_string()),
                Token::To,
                Token::Identifier("a".to_string()),
                Token::Plus,
                Token::Identifier("b".to_string()),
                Token::Multiply,
                Token::Identifier("c".to_string()),
                Token::Minus,
                Token::Identifier("d".to_string()),
                Token::Divide,
                Token::Identifier("e".to_string()),
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_lexer_escape_sequences() {
        let mut lexer = Lexer::new("\"hello\\nworld\\t\\\"\\\\\"");
        let tokens = lexer.tokenize().unwrap();
        let tok_types: Vec<Token> = tokens.into_iter().map(|(t, _)| t).collect();
        
        assert_eq!(
            tok_types,
            vec![
                Token::StringLiteral("hello\nworld\t\"\\".to_string()),
                Token::EOF
            ]
        );
    }
}
