use c4::{Lexer, Parser, Token, ASTNode, VM};

#[test]
fn test_parse_return() {
    let mut lexer = Lexer::new("return 42;");
    let mut tokens = Vec::new();
    loop {
        let t = lexer.next_token();
        if let Token::Eof = t {
            break;
        }
        tokens.push(t);
    }

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_return();
    VM::run(ast); // Should print: Returned: 42
}
