// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file

// Types of language tokens

use std::str::{CharIndices, Chars};

// Types of language tokens
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum TokenType {
    // Single-character tokens.
    TokenRightParen,
    TokenLeftParen,
    TokenLeftBrace,
    TokenRightBrace,
    TokenComma,
    TokenDot,
    TokenMinus,
    TokenPlus,
    TokenSemicolon,
    TokenSlash,
    TokenStar,
    // One or two character tokens.
    TokenBang,
    TokenBangEqual,
    TokenEqual,
    TokenEqualEqual,
    TokenGreater,
    TokenGreaterEqual,
    TokenLess,
    TokenLessEqual,
    TokenPlusPlus,
    // Literals.
    TokenIdentifier,
    TokenString,
    TokenNumber,
    // Keywords.
    TokenAnd,
    TokenClass,
    TokenElse,
    TokenFalse,
    TokenFor,
    TokenFun,
    TokenIf,
    TokenNil,
    TokenOr,
    TokenPrint,
    TokenReturn,
    TokenTrue,
    TokenVar,
    TokenWhile,

    TokenError,
    TokenEOF,
}

struct Token {
    _type: TokenType,
    start: String,
    length: u64,
    line: u64,
}

pub struct Scanner<'s> {
    start: Chars<'s>,
    current: Chars<'s>,
    line: u64,
}

impl<'s> Scanner<'s> {
    pub fn new(_source: &'s String) -> Scanner {
        Scanner {
            start: _source.chars(),
            current: _source.chars(),
            line: 1,
        }
    }
    pub fn scan_token(&'s mut self) -> Token {
        self.start = self.current.clone();

        Token::error_token("Unexpected character.".to_string(), self)
    }
}

impl Token {
    pub fn new(_type: TokenType, scanner: &Scanner) -> Token {
        Token {
            _type,
            start: scanner.start.as_str().to_string(), //TODO
            length: 5,                                 //TODO
            line: scanner.line,
        }
    }
    pub fn error_token(msg: String, scanner: &Scanner) -> Token {
        Token {
            _type: TokenType::TokenError,
            start: msg,
            length: 5, //TODO
            line: scanner.line,
        }
    }
}
