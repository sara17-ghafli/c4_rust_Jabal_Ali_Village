use std::io::{self, Read};
use c4_rust_team_jabal_ali_village::{Lexer, Parser, Token, VM};

fn main() {
    // Read all input from stdin
    let mut source = String::new();
    io::stdin().read_to_string(&mut source).unwrap();

    // Lexing: convert input into tokens
    let mut lexer = Lexer::new(&source);
    let mut tokens = vec![];
    loop {
        let t = lexer.next_token();
        if let Token::Eof = t { break; }
        tokens.push(t);
    }

    // Parsing: convert tokens into an AST
    let mut parser = Parser::new(tokens);
    match parser.parse_return() {
        // Evaluation: run the AST
        Ok(ast) => VM::run(ast),
        Err(e) => eprintln!("Syntax Error: {}", e),
    }
}
