use c4::*; // Make sure c4.rs is exposed in lib.rs or main.rs

#[test]
fn test_lex_number() {
    let mut lexer = Lexer::new("123");
    assert_eq!(lexer.next_token(), Token::Number(123));
}

#[test]
fn test_lex_identifier() {
    let mut lexer = Lexer::new("abc");
    assert_eq!(lexer.next_token(), Token::Identifier("abc".to_string()));
}

#[test]
fn test_lex_keyword() {
    let mut lexer = Lexer::new("return");
    assert_eq!(lexer.next_token(), Token::Keyword("return".to_string()));
}

#[test]
fn test_lex_symbol() {
    let mut lexer = Lexer::new("+");
    assert_eq!(lexer.next_token(), Token::Symbol('+'));
}

#[test]
fn test_eof() {
    let mut lexer = Lexer::new("");
    assert_eq!(lexer.next_token(), Token::Eof);
}
