use c4_rust::c4::{Lexer, Parser, Token, ASTNode, VM};

//tests if 'return 42;' is parsed and executed correctly
#[test]
fn test_parse_return() {
    let mut lexer = Lexer::new("return 42;");
    let mut tokens = Vec::new();

    //collect tokens til eof
    loop {
        let t = lexer.next_token();
        if let Token::Eof = t {
            break;
        }
        tokens.push(t);
    }

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_return();
    VM::run(ast); //outputs: Returned: 42
}
