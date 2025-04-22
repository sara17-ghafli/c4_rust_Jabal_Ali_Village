use std::io::{self, Read};
use c4::{Lexer, Parser, VM, Token};

fn main() {
    // Read input C code from stdin
    let mut source = String::new();
    io::stdin().read_to_string(&mut source).unwrap();

    // Lexing
    let mut lexer = Lexer::new(&source);
    let mut tokens = Vec::new();
    loop {
        let tok = lexer.next_token();
        if let Token::Eof = tok {
            break;
        }
        tokens.push(tok);
    }

    // Parsing and running
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_return();
    VM::run(ast);
}
