// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file

use crate::{
    chunk::{Chunk, OpCode},
    lexer::{Token, TokenType},
};

pub struct Compiler<'c> {
    current: usize,
    tokens: &'c Vec<Token>,
    chunk: Chunk,
    had_error: bool,
    panic_mode: bool,
}

impl<'c> Compiler<'c> {
    pub fn new(tokens: &'c Vec<Token>, chunk: Chunk) -> Self {
        return Compiler {
            tokens,
            current: 0,
            had_error: false,
            panic_mode: false,
            chunk,
        };
    }
    pub fn compile(&mut self, source: &'c String, chunk: Chunk) -> bool {
        self.had_error = false;
        self.panic_mode = false;
        self.chunk = chunk;

        self.advance();
        self.expression();
        self.consume(TokenType::TokenEOF, "Expected end of expression.");

        self.end_compiler();
        return !self.had_error;
    }

    fn end_compiler(&mut self) -> () {
        self.emit_return();
    }

    fn advance(&mut self) -> () {
        for token in self.tokens {
            if token._type != TokenType::TokenError {
                break;
            }

            // Error encountered, throw compiler error
            self.error_at_current(&token.source_str);
        }
    }

    fn expression(&mut self) -> () {}

    fn consume(&mut self, _type: TokenType, msg: &'c str) -> () {
        if self.tokens[self.current]._type == _type {
            self.advance();
            return;
        }

        self.error_at_current(msg);
    }

    fn emit_byte(&mut self, byte: OpCode) -> () {
        self.chunk
            .write_chunk(byte, self.tokens[self.current - 1].line);
    }

    fn emit_bytes(&mut self, byte1: OpCode, byte2: OpCode) -> () {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_return(&mut self) -> () {
        self.emit_byte(OpCode::OpReturn);
    }

    fn error_at_current(&mut self, msg: &'c str) -> () {
        self.error_at(msg, self.current);
    }

    fn error(&mut self, msg: &'c str) -> () {
        self.error_at(msg, self.current - 1);
    }

    fn error_at(&mut self, msg: &'c str, index: usize) -> () {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;

        let token = &self.tokens[index];
        print!("[line {}] Error", token.line);

        match token._type {
            TokenType::TokenEOF => print!(" at end"),
            TokenType::TokenError => print!(""),
            _ => print!(" at '{}'", token.source_str),
        };

        println!(": {}", msg);
        self.had_error = true;
    }
}
