use c4_rust_team_jabal_ali_village::c4::{Lexer, Parser, Token, VM};

// to test if "return 7 + 3;" is handled correctly by all components (lexer, parser, and VM)
#[test]
fn test_return_addition() {
    // give the lexer the input string
    let mut lexer = Lexer::new("return 7 + 3;");
    let mut tokens = vec![];

    // collect all the tokens until EOF
    loop {
        let t = lexer.next_token();
        if let Token::Eof = t { break; }
        tokens.push(t);
    }

    // pass the tokens to the parser
    let mut parser = Parser::new(tokens);

    // try to parse it as a return statement
    let ast = parser.parse_return().unwrap();

    // check that the result of evaluating it is 10
    assert_eq!(VM::eval(ast), 10);
}
