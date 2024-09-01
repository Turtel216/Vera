use std::str::Chars;

use crate::lexer::Scanner;
use crate::lexer::TokenType;

pub fn compile(source: &String) -> () {
    let mut lexer = Scanner::new(source);
}
