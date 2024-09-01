use std::str::Chars;

use crate::lexer::Scanner;
use crate::lexer::TokenType;

pub fn compile(_source: &String) -> () {
    let mut lexer = Scanner::new(&_source.chars());
}
