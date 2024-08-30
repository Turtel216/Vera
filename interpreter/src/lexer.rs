// Types of language tokens

use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum TokenType {
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

// Token's meta data
pub struct Token<'sourcecode> {
    pub kind: TokenType, // The type of language token
    line: usize,         // Line number of token in source file
    pub lexeme: &'sourcecode str,
}

impl<'sourcecode> Token<'sourcecode> {
    pub fn synthetic(text: &'sourcecode str) -> Token<'sourcecode> {
        Token {
            kind: TokenType::TokenError,
            lexeme: text,
            line: 0,
        }
    }
}

pub struct Scanner<'sourcecode> {
    keywords: HashMap<&'static str, TokenType>,
    code: &'sourcecode str,
    start: usize,
    current: usize,
    line: usize,
}

impl<'sourcecode> Scanner<'sourcecode> {
    pub fn new(code: &'sourcecode str) -> Scanner {
        let mut keywords = HashMap::with_capacity(16);
        keywords.insert("and", TokenType::TokenAnd);
        keywords.insert("else", TokenType::TokenElse);
        keywords.insert("false", TokenType::TokenFalse);
        keywords.insert("for", TokenType::TokenFor);
        keywords.insert("fun", TokenType::TokenFun);
        keywords.insert("if", TokenType::TokenIf);
        keywords.insert("nil", TokenType::TokenNil);
        keywords.insert("or", TokenType::TokenOr);
        keywords.insert("print", TokenType::TokenPrint);
        keywords.insert("return", TokenType::TokenReturn);
        keywords.insert("true", TokenType::TokenTrue);
        keywords.insert("var", TokenType::TokenVar);
        keywords.insert("while", TokenType::TokenWhile);

        Scanner {
            keywords,
            code,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token<'sourcecode> {
        self.skip_whitespace();
        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenType::TokenEOF);
        }

        match self.advance() {
            b'(' => self.make_token(TokenType::TokenLeftParen),
            b')' => self.make_token(TokenType::TokenRightParen),
            b'{' => self.make_token(TokenType::TokenLeftBrace),
            b'}' => self.make_token(TokenType::TokenRightBrace),
            b';' => self.make_token(TokenType::TokenSemicolon),
            b',' => self.make_token(TokenType::TokenComma),
            b'.' => self.make_token(TokenType::TokenDot),
            b'-' => self.make_token(TokenType::TokenMinus),
            b'+' => self.make_token(TokenType::TokenPlus),
            b'/' => self.make_token(TokenType::TokenSlash),
            b'*' => self.make_token(TokenType::TokenStar),
            b'!' if self.matches(b'=') => self.make_token(TokenType::TokenBangEqual),
            b'!' => self.make_token(TokenType::TokenBang),
            b'=' if self.matches(b'=') => self.make_token(TokenType::TokenEqualEqual),
            b'=' => self.make_token(TokenType::TokenEqual),
            b'<' if self.matches(b'=') => self.make_token(TokenType::TokenLessEqual),
            b'<' => self.make_token(TokenType::TokenLess),
            b'>' if self.matches(b'=') => self.make_token(TokenType::TokenGreaterEqual),
            b'>' => self.make_token(TokenType::TokenGreater),
            b'"' => self.string(),
            c if is_digit(c) => self.number(),
            c if is_alpha(c) => self.identifier(),
            _ => self.error_token("Unexpected character."),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current == self.code.len()
    }

    fn lexeme(&self) -> &'sourcecode str {
        &self.code[self.start..self.current]
    }

    fn make_token(&self, kind: TokenType) -> Token<'sourcecode> {
        Token {
            kind,
            lexeme: self.lexeme(),
            line: self.line,
        }
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            0
        } else {
            self.code.as_bytes()[self.current]
        }
    }
    fn peek_next(&self) -> u8 {
        if self.current > self.code.len() - 2 {
            b'\0'
        } else {
            self.code.as_bytes()[self.current + 1]
        }
    }

    fn error_token(&self, message: &'static str) -> Token<'static> {
        Token {
            kind: TokenType::TokenError,
            lexeme: message,
            line: self.line,
        }
    }

    fn advance(&mut self) -> u8 {
        let char = self.peek();
        self.current += 1;
        char
    }

    fn matches(&mut self, expected: u8) -> bool {
        if self.is_at_end() || self.peek() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                b' ' | b'\r' | b'\t' => {
                    self.advance();
                }
                b'\n' => {
                    self.line += 1;
                    self.advance();
                }
                b'/' if self.peek_next() == b'/' => {
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                _ => return,
            }
        }
    }

    fn string(&mut self) -> Token<'sourcecode> {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error_token("Unterminated string.")
        } else {
            self.advance();
            self.make_token(TokenType::TokenString)
        }
    }

    fn number(&mut self) -> Token<'sourcecode> {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == b'.' && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }

        self.make_token(TokenType::TokenNumber)
    }

    fn identifier(&mut self) -> Token<'sourcecode> {
        while is_alpha(self.peek()) || is_digit(self.peek()) {
            self.advance();
        }
        self.make_token(self.identifier_type())
    }

    fn identifier_type(&self) -> TokenType {
        self.keywords
            .get(self.lexeme())
            .cloned()
            .unwrap_or(TokenType::TokenIdentifier)
    }
}

fn is_digit(c: u8) -> bool {
    c.is_ascii_digit()
}

fn is_alpha(c: u8) -> bool {
    c.is_ascii_alphabetic() || c == b'_'
}
