// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file

// Types of language tokens

use std::str::Chars;

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
    line: usize,
}

// Scanner is used to tokenize the source string
pub struct Scanner<'s> {
    source: &'s str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'s> Scanner<'s> {
    pub fn new(source: &'s str) -> Scanner<'s> {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;

            self.scan_token();
        }

        self.tokens.push(Token {
            _type: TokenType::TokenEOF,
            start: "".to_string(),
            line: self.line,
        });

        return &self.tokens;
    }

    fn scan_token(&mut self) -> () {
        let current_char = self.advance();

        match current_char {
            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => (),
        }

        Token::error_token("Unexpected character.".to_string(), self);
    }

    fn is_at_end(&self) -> bool {
        return self.current > self.source.len();
    }

    fn advance(&mut self) -> char {
        let char = self.source.chars().nth(self.current).unwrap_or_else(|| {
            println!(
                "No characters at index {} were found. Last character was {}.",
                self.current,
                self.source.chars().nth(self.current - 1).unwrap()
            );
            std::process::exit(1);
        });

        self.current += 1;

        return char;
    }
}

impl Token {
    pub fn new(_type: TokenType, scanner: &Scanner) -> Token {
        Token {
            _type,
            start: scanner.start.to_string(), //TODO
            line: scanner.line,
        }
    }
    pub fn error_token(msg: String, scanner: &Scanner) -> Token {
        Token {
            _type: TokenType::TokenError,
            start: msg,
            line: scanner.line,
        }
    }
}
