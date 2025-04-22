use std::io::{self, Read};                     // read input from the user
use c4_rust::{Lexer, Parser, VM, Token};       // bring in the compiler parts from the crate

fn main() {
    //read input C code from the terminal/file
    let mut source = String::new();
    io::stdin().read_to_string(&mut source).unwrap(); // Read entire input into a string

    //convert the code into tokens using lexer
    let mut lexer = Lexer::new(&source);       
    let mut tokens = Vec::new();               // stores all the tokens

    // read until eof
    loop {
        let tok = lexer.next_token();          // Get the next token
        if let Token::Eof = tok {              // If it's the end, stop
            break;
        }
        tokens.push(tok);                      // else save the token
    }

    //turn tokens into an AST using the parser
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_return();           // Only supports parsing return statements now

    //run the AST using our vm
    VM::run(ast);                              //prints result
}
