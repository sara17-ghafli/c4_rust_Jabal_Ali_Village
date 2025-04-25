use std::io::{self, Read};
use c4_rust::{Lexer, Parser, Token, VM};

fn main() {
    let mut source = String::new();
    io::stdin().read_to_string(&mut source).unwrap();

    let mut lexer = Lexer::new(&source);
    let mut tokens = vec![];
    loop {
        let t = lexer.next_token();
        if let Token::Eof = t { break; }
        tokens.push(t);
    }

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_return();
    VM::run(ast);
}
