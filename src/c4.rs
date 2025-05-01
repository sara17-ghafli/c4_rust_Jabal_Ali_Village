use std::iter::Peekable;
use std::str::Chars;

/// Keeps track of where we are in the input (line and column)
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

/// Different types of tokens we find
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i32, Position),
    Identifier(String, Position),
    Keyword(String, Position),
    Symbol(char, Position),
    Eof,
}

/// Lexer to break input into tokens
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

/// AST nodes to represent the code
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

/// Parser to build the AST
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

    fn expect_symbol(&mut self, ch: char) -> Result<(), String> {
        match self.peek() {
            Token::Symbol(c, pos) if *c == ch => {
                self.advance();
                Ok(())
            }
            Token::Symbol(found, pos) => Err(format!(
                "Error at line {}, column {}: Expected '{}', found '{}'",
                pos.line, pos.column, ch, found
            )),
            Token::Eof => Err("Unexpected end of input.".to_string()),
            _ => Err("Expected symbol but found something else.".to_string()),
        }
    }

    pub fn parse_expression(&mut self) -> Result<ASTNode, String> {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Result<ASTNode, String> {
        let mut node = self.parse_factor()?;

        while let Token::Symbol(op, _) = self.peek() {
            if *op == '+' || *op == '-' {
                let op = *op;
                self.advance();
                let right = self.parse_factor()?;
                node = ASTNode::BinaryOp {
                    op,
                    left: Box::new(node),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(node)
    }

    fn parse_factor(&mut self) -> Result<ASTNode, String> {
        let current = self.peek().clone();
        let mut node = match current {
            Token::Number(n, _) => {
                self.advance();
                ASTNode::Number(n)
            }
            Token::Identifier(ref name, _) => {
                let name = name.clone();
                self.advance();
                if let Token::Symbol('=', _) = self.peek() {
                    self.advance();
                    let expr = self.parse_expression()?;
                    ASTNode::Assignment(name, Box::new(expr))
                } else {
                    ASTNode::Identifier(name)
                }
            }
            Token::Symbol('(', _) => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect_symbol(')')?;
                expr
            }
            _ => return Err(format!("Unexpected token: {:?}", current)),
        };

        while let Token::Symbol(op, _) = self.peek() {
            if *op == '*' || *op == '/' {
                let op = *op;
                self.advance();
                let right = self.parse_factor()?;
                node = ASTNode::BinaryOp {
                    op,
                    left: Box::new(node),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(node)
    }

    pub fn parse_return(&mut self) -> Result<ASTNode, String> {
        if let Token::Keyword(k, pos) = self.peek() {
            if k == "return" {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect_symbol(';')?;
                return Ok(ASTNode::Return(Box::new(expr)));
            }
        }
        Err(format!("Expected 'return' keyword, found: {:?}", self.peek()))
    }
}

/// VM to evaluate the AST
pub struct VM;

impl VM {
    pub fn run(ast: ASTNode) {
        let result = VM::eval(ast);
        println!("Returned: {}", result);
    }

    pub fn eval(node: ASTNode) -> i32 {
        match node {
            ASTNode::Number(n) => n,
            ASTNode::Identifier(_) => 0,
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
