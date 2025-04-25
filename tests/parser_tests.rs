use c4_rust_team_jabal_ali_village::c4::{Lexer, Parser, Token, VM};

//to test if return 7 + 3 is parsed and evaluated correctly
#[test]
fn test_return_addition() {
    let mut lexer = Lexer::new("return 7 + 3;");
    let mut tokens = vec![];
    loop {
        let t = lexer.next_token();
        if let Token::Eof = t { break; }
        tokens.push(t);
    }
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_return();
    assert_eq!(VM::eval(ast), 10);
}
