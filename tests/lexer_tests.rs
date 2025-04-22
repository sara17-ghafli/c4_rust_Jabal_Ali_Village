use c4_rust::c4::{Lexer, Token, Position};

//were numbers tokenized correctly?
#[test]
fn test_lex_number() {
    let mut lexer = Lexer::new("123");
    assert_eq!(lexer.next_token(), Token::Number(123, Position { line: 1, column: 0 }));
}

//were identifiers handled corredtly?
#[test]
fn test_lex_identifier() {
    let mut lexer = Lexer::new("abc");
    assert_eq!(lexer.next_token(), Token::Identifier("abc".to_string(), Position { line: 1, column: 0 }));
}

//was a keyword detected?
#[test]
fn test_lex_keyword() {
    let mut lexer = Lexer::new("return");
    assert_eq!(lexer.next_token(), Token::Keyword("return".to_string(), Position { line: 1, column: 0 }));
}

//were symbols tokenized?
#[test]
fn test_lex_symbol() {
    let mut lexer = Lexer::new("+");
    assert_eq!(lexer.next_token(), Token::Symbol('+', Position { line: 1, column: 0 }));
}

//was EOF returned correctly?
#[test]
fn test_eof() {
    let mut lexer = Lexer::new("");
    assert_eq!(lexer.next_token(), Token::Eof);
}
