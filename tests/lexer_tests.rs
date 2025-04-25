use c4_rust_team_jabal_ali_village::c4::{Lexer, Token, Position};

//is number token is correct?
#[test]
fn test_lex_number() {
    let mut lexer = Lexer::new("123");
    assert_eq!(lexer.next_token(), Token::Number(123, Position { line: 1, column: 0 }));
}

//is identifier token is correct?
#[test]
fn test_lex_identifier() {
    let mut lexer = Lexer::new("abc");
    assert_eq!(lexer.next_token(), Token::Identifier("abc".to_string(), Position { line: 1, column: 0 }));
}

//is keyword token is correct?
#[test]
fn test_lex_keyword() {
    let mut lexer = Lexer::new("return");
    assert_eq!(lexer.next_token(), Token::Keyword("return".to_string(), Position { line: 1, column: 0 }));
}

//is symbol token is correct?
#[test]
fn test_lex_symbol() {
    let mut lexer = Lexer::new("+");
    assert_eq!(lexer.next_token(), Token::Symbol('+', Position { line: 1, column: 0 }));
}

//is EOF is handled?
#[test]
fn test_eof() {
    let mut lexer = Lexer::new("");
    assert_eq!(lexer.next_token(), Token::Eof);
}
