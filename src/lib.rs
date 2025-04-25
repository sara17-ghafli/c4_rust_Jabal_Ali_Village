// Make c4.rs available as a library
pub mod c4;
pub use c4::{Lexer, Token, Position, Parser, ASTNode, VM};
