use std::iter::Peekable;
use std::str::Chars;

/// Position info (bonus feature)
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

/// Token types
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i32, Position),
    Identifier(String, Position),
    Keyword(String, Position),
    Symbol(char, Position),
    Eof,
}

/// Lexer
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    current_line: usize,
    current_column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            input: source.chars().peekable(),
            current_line: 1,
            current_column: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let pos = Position {
            line: self.current_line,
            column: self.current_column,
        };

        let ch = match self.input.peek() {
            Some(&c) => c,
            None => return Token::Eof,
        };

        if ch.is_ascii_digit() {
            return self.lex_number(pos);
        }

        if ch.is_ascii_alphabetic() || ch == '_' {
            return self.lex_identifier_or_keyword(pos);
        }

        self.input.next();
        self.current_column += 1;
        Token::Symbol(ch, pos)
    }

    fn lex_number(&mut self, pos: Position) -> Token {
        let mut number = 0;
        while let Some(&ch) = self.input.peek() {
            if let Some(d) = ch.to_digit(10) {
                number = number * 10 + d as i32;
                self.input.next();
                self.current_column += 1;
            } else {
                break;
            }
        }
        Token::Number(number, pos)
    }

    fn lex_identifier_or_keyword(&mut self, pos: Position) -> Token {
        let mut ident = String::new();
        while let Some(&ch) = self.input.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.input.next();
                self.current_column += 1;
            } else {
                break;
            }
        }

        match ident.as_str() {
            "int" | "return" => Token::Keyword(ident, pos),
            _ => Token::Identifier(ident, pos),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.input.peek() {
            if ch == '\n' {
                self.current_line += 1;
                self.current_column = 0;
                self.input.next();
            } else if ch.is_whitespace() {
                self.current_column += 1;
                self.input.next();
            } else {
                break;
            }
        }
    }
}

/// AST
#[derive(Debug, PartialEq)]
pub enum ASTNode {
    Number(i32),
    Identifier(String),
    Assignment(String, Box<ASTNode>),
    BinaryOp {
        op: char,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    Return(Box<ASTNode>),
}

/// Parser
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn expect_symbol(&mut self, ch: char) {
        match self.peek() {
            Token::Symbol(c, _) if *c == ch => self.advance(),
            _ => panic!("Expected symbol: '{}'", ch),
        }
    }

    pub fn parse_expression(&mut self) -> ASTNode {
        self.parse_term()
    }

    fn parse_term(&mut self) -> ASTNode {
        let mut node = self.parse_factor();

        while let Token::Symbol(op, _) = self.peek() {
            if *op == '+' || *op == '-' {
                let op = *op;
                self.advance();
                let right = self.parse_factor();
                node = ASTNode::BinaryOp {
                    op,
                    left: Box::new(node),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        node
    }

    fn parse_factor(&mut self) -> ASTNode {
        let mut node = match self.peek() {
            Token::Number(n, _) => {
                let n = *n;
                self.advance();
                ASTNode::Number(n)
            }
            Token::Identifier(name, _) => {
                let name = name.clone();
                self.advance();
                if let Token::Symbol('=', _) = self.peek() {
                    self.advance();
                    let expr = self.parse_expression();
                    ASTNode::Assignment(name, Box::new(expr))
                } else {
                    ASTNode::Identifier(name)
                }
            }
            Token::Symbol('(', _) => {
                self.advance();
                let expr = self.parse_expression();
                self.expect_symbol(')');
                expr
            }
            _ => panic!("Unexpected token"),
        };

        while let Token::Symbol(op, _) = self.peek() {
            if *op == '*' || *op == '/' {
                let op = *op;
                self.advance();
                let right = self.parse_factor();
                node = ASTNode::BinaryOp {
                    op,
                    left: Box::new(node),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        node
    }

    pub fn parse_return(&mut self) -> ASTNode {
        if let Token::Keyword(k, _) = self.peek() {
            if k == "return" {
                self.advance();
                let expr = self.parse_expression();
                self.expect_symbol(';');
                return ASTNode::Return(Box::new(expr));
            }
        }
        panic!("Expected 'return'");
    }
}

/// Virtual Machine
pub struct VM;

impl VM {
    pub fn run(ast: ASTNode) {
        let result = VM::eval(ast);
        println!("Returned: {}", result);
    }

    pub fn eval(node: ASTNode) -> i32 {
        match node {
            ASTNode::Number(n) => n,
            ASTNode::Identifier(_) => 0, // not yet storing values
            ASTNode::Assignment(_, val) => VM::eval(*val),
            ASTNode::BinaryOp { op, left, right } => {
                let l = VM::eval(*left);
                let r = VM::eval(*right);
                match op {
                    '+' => l + r,
                    '-' => l - r,
                    '*' => l * r,
                    '/' => l / r,
                    _ => panic!("Unsupported operator"),
                }
            }
            ASTNode::Return(expr) => VM::eval(*expr),
        }
    }
}
