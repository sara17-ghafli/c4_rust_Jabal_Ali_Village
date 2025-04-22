use std::str::Chars;
use std::iter::Peekable;

/// This keeps track of where each token is in the text for error reporting
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

///These are the types of tokens our compiler understands which are the value and position output for lexer
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i32, Position),         // Numbers like 42
    Identifier(String, Position), // Variable names like x, y
    Keyword(String, Position),    // Reserved words like return
    Symbol(char, Position),       // Characters like +, =, ;
    Eof,                          // End of file
}

/// The Lexer reads the source text one char at a time
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,   // What we're reading
    current_line: usize,
    current_column: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer from source code
    pub fn new(source: &'a str) -> Self {
        Lexer {
            input: source.chars().peekable(),
            current_line: 1,
            current_column: 0,
        }
    }
    /// Read the next token ie word,number, or symbol
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace(); // Skip spaces and newlines

        let pos = Position {
            line: self.current_line,
            column: self.current_column,
        };
         // Check whats coming next
        let ch = match self.input.peek() {
            Some(&c) => c,
            None => return Token::Eof,
        };
        // Decide what kind of token it is
        if ch.is_ascii_digit() {
            return self.lex_number(pos);
        }

        if ch.is_ascii_alphabetic() || ch == '_' {
            return self.lex_identifier_or_keyword(pos);
        }
        // determine if its just a symbl like = or ;
        self.input.next();
        self.current_column += 1;
        Token::Symbol(ch, pos)
    }
    /// Handle numbers like our test 42
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
    /// Handle keywords like `return` or variables like `x`
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
            "int" | "return" | "if" | "else" | "while" | "for" => Token::Keyword(ident, pos),
            _ => Token::Identifier(ident, pos),
        }
    }
    /// Ignores the spaces and line breaks
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

// ----------------------------
// IMPLEMENTING PARSER + AST
// ----------------------------

/// These are the shapes of code we want to recognize
#[derive(Debug)]
pub enum ASTNode {
    Number(i32),                              // A number
    Identifier(String),                       // A variable name
    BinaryOp {                                // (Not used yet)
        op: char,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    Assignment(String, Box<ASTNode>),         // x = 42
    Return(Box<ASTNode>),                     // return 42
}

pub struct Parser {
    tokens: Vec<Token>,  // Tokens from the lexer
    pos: usize,          // Where we are in the list
}

impl Parser {
    /// Make a new parser from tokens
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }
    /// Look at the current token
    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }
    /// Move to the next token
    fn advance(&mut self) {
        self.pos += 1;
    }
    /// Parse a number, identifier, or asssignment
    pub fn parse_expression(&mut self) -> ASTNode {
    match self.peek() {
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
        _ => panic!("Unexpected token in expression: {:?}", self.peek()),
    }
}
    /// Parse return statements, in our cas the `return 42;`
    pub fn parse_return(&mut self) -> ASTNode {
        if let Token::Keyword(k, _) = self.peek() {
            if k == "return" {
                self.advance();
                let expr = self.parse_expression();
                return ASTNode::Return(Box::new(expr));
            }
        }
        panic!("Expected 'return'");
    }
}

// ----------------------------
// IMPLEMENTING VIRTUAL MACHINE
// ----------------------------

pub struct VM;

impl VM {
    /// Run the parsed code (AST)
    pub fn run(ast: ASTNode) {
        match ast {
            ASTNode::Return(expr) => {
                let value = VM::eval(*expr);
                println!("Returned: {}", value); // Print the result
            }
            _ => println!("Unsupported AST"),
        }
    }
    /// Evaluate (finding the answer) the result of an expression
    fn eval(node: ASTNode) -> i32 {
        match node {
            ASTNode::Number(n) => n,
            ASTNode::BinaryOp { .. } => 0, // Not used yet
            ASTNode::Assignment(_, val) => VM::eval(*val),
            ASTNode::Identifier(_) => 0,   // We don't store variables yet
            ASTNode::Return(expr) => VM::eval(*expr),
        }
    }
}
