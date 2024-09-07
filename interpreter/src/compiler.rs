// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file

use crate::object::ObjString;
use std::collections::HashMap;

use crate::{
    chunk::{Chunk, OpCode},
    lexer::{Token, TokenType},
    value::Value,
};

#[derive(Copy, Clone, PartialOrd, PartialEq)]
enum Precedence {
    PrecNone,
    PrecAssignment, // =
    PrecOr,         // or
    PrecAnd,        // and
    PrecEquality,   // == !=
    PrecComparsion, // < > <= >=
    PrecTerm,       // + -
    PrecFactor,     // * /
    PrecUnary,      // ! -
    PrecCall,       // . ()
    PrecPrimary,
}

impl Precedence {
    fn next(&self) -> Precedence {
        match self {
            Precedence::PrecNone => Precedence::PrecAssignment,
            Precedence::PrecAssignment => Precedence::PrecOr,
            Precedence::PrecOr => Precedence::PrecAnd,
            Precedence::PrecAnd => Precedence::PrecEquality,
            Precedence::PrecEquality => Precedence::PrecComparsion,
            Precedence::PrecComparsion => Precedence::PrecTerm,
            Precedence::PrecTerm => Precedence::PrecFactor,
            Precedence::PrecFactor => Precedence::PrecUnary,
            Precedence::PrecUnary => Precedence::PrecCall,
            Precedence::PrecCall => Precedence::PrecPrimary,
            Precedence::PrecPrimary => Precedence::PrecNone,
        }
    }
}

type ParseFn<'sourcecode> = fn(&mut Compiler<'sourcecode>) -> ();

#[derive(Copy, Clone)]
struct ParseRule<'p> {
    precedence: Precedence,
    prefix: Option<ParseFn<'p>>,
    infix: Option<ParseFn<'p>>,
}

impl<'p> ParseRule<'p> {
    fn new(
        prefix: Option<ParseFn<'p>>,
        infix: Option<ParseFn<'p>>,
        precedence: Precedence,
    ) -> ParseRule<'p> {
        ParseRule {
            precedence,
            prefix,
            infix,
        }
    }
}

pub struct Compiler<'c> {
    current: usize,
    tokens: &'c Vec<Token>,
    pub chunk: &'c mut Chunk,
    had_error: bool,
    panic_mode: bool,
    rules: HashMap<TokenType, ParseRule<'c>>,
}

impl<'c> Compiler<'c> {
    pub fn new(tokens: &'c Vec<Token>, chunk: &'c mut Chunk) -> Self {
        let mut rules = HashMap::new();
        let mut rule = |kind, prefix, infix, precedence| {
            rules.insert(kind, ParseRule::new(prefix, infix, precedence));
        };

        //TODO add TokenMinusMinus and TokenPlusPlus

        rule(
            TokenType::TokenLeftParen,
            Some(Compiler::grouping),
            None,
            Precedence::PrecNone,
        );
        rule(TokenType::TokenRightParen, None, None, Precedence::PrecNone);
        rule(TokenType::TokenLeftBrace, None, None, Precedence::PrecNone);
        rule(TokenType::TokenRightBrace, None, None, Precedence::PrecNone);
        rule(TokenType::TokenComma, None, None, Precedence::PrecNone);
        rule(TokenType::TokenDot, None, None, Precedence::PrecNone);
        rule(
            TokenType::TokenMinus,
            Some(Compiler::unary),
            Some(Compiler::binary),
            Precedence::PrecTerm,
        );
        rule(
            TokenType::TokenPlus,
            None,
            Some(Compiler::binary),
            Precedence::PrecTerm,
        );
        rule(
            TokenType::TokenPow,
            None,
            Some(Compiler::binary),
            Precedence::PrecTerm,
        );
        rule(
            TokenType::TokenShiftLeft,
            None,
            Some(Compiler::binary),
            Precedence::PrecTerm,
        );
        rule(
            TokenType::TokenShiftRigth,
            None,
            Some(Compiler::binary),
            Precedence::PrecTerm,
        );
        rule(TokenType::TokenSemicolon, None, None, Precedence::PrecNone);
        rule(
            TokenType::TokenSlash,
            None,
            Some(Compiler::binary),
            Precedence::PrecFactor,
        );
        rule(
            TokenType::TokenStar,
            None,
            Some(Compiler::binary),
            Precedence::PrecFactor,
        );
        rule(
            TokenType::TokenBang,
            Some(Compiler::unary),
            None,
            Precedence::PrecNone,
        );
        rule(
            TokenType::TokenBangEqual,
            None,
            Some(Compiler::binary),
            Precedence::PrecEquality,
        );
        rule(TokenType::TokenEqual, None, None, Precedence::PrecNone);
        rule(
            TokenType::TokenEqualEqual,
            None,
            Some(Compiler::binary),
            Precedence::PrecEquality,
        );
        rule(
            TokenType::TokenGreater,
            None,
            Some(Compiler::binary),
            Precedence::PrecComparsion,
        );
        rule(
            TokenType::TokenGreaterEqual,
            None,
            Some(Compiler::binary),
            Precedence::PrecComparsion,
        );
        rule(
            TokenType::TokenLess,
            None,
            Some(Compiler::binary),
            Precedence::PrecComparsion,
        );
        rule(
            TokenType::TokenLessEqual,
            None,
            Some(Compiler::binary),
            Precedence::PrecComparsion,
        );
        rule(TokenType::TokenIdentifier, None, None, Precedence::PrecNone);
        rule(
            TokenType::TokenString,
            Some(Compiler::parse_string),
            None,
            Precedence::PrecNone,
        );
        rule(
            TokenType::TokenNumber,
            Some(Compiler::parse_number),
            None,
            Precedence::PrecNone,
        );
        rule(
            TokenType::TokenTrue,
            Some(Compiler::literal),
            None,
            Precedence::PrecNone,
        );
        rule(
            TokenType::TokenFalse,
            Some(Compiler::literal),
            None,
            Precedence::PrecNone,
        );
        rule(
            TokenType::TokenNil,
            Some(Compiler::literal),
            None,
            Precedence::PrecNone,
        );
        rule(TokenType::TokenAnd, None, None, Precedence::PrecNone);
        rule(TokenType::TokenClass, None, None, Precedence::PrecNone);
        rule(TokenType::TokenElse, None, None, Precedence::PrecNone);
        rule(
            TokenType::TokenFalse,
            Some(Compiler::literal),
            None,
            Precedence::PrecNone,
        );
        rule(TokenType::TokenFor, None, None, Precedence::PrecNone);
        rule(TokenType::TokenFor, None, None, Precedence::PrecNone);
        rule(TokenType::TokenFun, None, None, Precedence::PrecNone);
        rule(TokenType::TokenIf, None, None, Precedence::PrecNone);
        rule(TokenType::TokenNil, None, None, Precedence::PrecNone);
        rule(TokenType::TokenOr, None, None, Precedence::PrecNone);
        rule(TokenType::TokenPrint, None, None, Precedence::PrecNone);
        rule(TokenType::TokenReturn, None, None, Precedence::PrecNone);
        rule(TokenType::TokenTrue, None, None, Precedence::PrecNone);
        rule(TokenType::TokenVar, None, None, Precedence::PrecNone);
        rule(TokenType::TokenWhile, None, None, Precedence::PrecNone);
        rule(TokenType::TokenError, None, None, Precedence::PrecNone);
        rule(TokenType::TokenEOF, None, None, Precedence::PrecNone);

        return Compiler {
            tokens,
            current: 0,
            had_error: false,
            panic_mode: false,
            chunk,
            rules,
        };
    }
    pub fn compile(&mut self) -> bool {
        self.expression();
        self.consume(TokenType::TokenEOF, "Expected end of expression.");

        self.end_compiler();
        return !self.had_error;
    }

    fn end_compiler(&mut self) -> () {
        self.emit_return();
    }

    fn advance(&mut self) -> () {
        self.current += 1;

        if self.current == self.tokens.len() {
            return;
        }

        if self.tokens[self.current]._type != TokenType::TokenError {
            return;
        }

        self.error_at_current(&self.tokens[self.current].source_str);
    }

    fn expression(&mut self) -> () {
        self.parse_precedence(Precedence::PrecAssignment);
    }

    fn consume(&mut self, _type: TokenType, msg: &'c str) -> () {
        if self.tokens[self.current]._type == _type {
            self.advance();
            return;
        }

        self.error_at_current(msg);
    }

    fn grouping(&mut self) -> () {
        self.expression();
        self.consume(TokenType::TokenRightParen, "Expect ')' after expression.");
    }

    fn unary(&mut self) -> () {
        let operator_type = self.tokens[self.current - 1]._type;

        // Compile the operand
        self.parse_precedence(Precedence::PrecUnary);

        // Emit le operator instuction
        match operator_type {
            TokenType::TokenMinus => self.emit_byte(OpCode::OpNegate),
            TokenType::TokenBang => self.emit_byte(OpCode::OpNot),
            _ => return,
        }
    }

    fn binary(&mut self) -> () {
        let operator_type = self.tokens[self.current - 1]._type;
        let rule = self.get_rule(operator_type);
        self.parse_precedence(rule.precedence.next());

        match operator_type {
            TokenType::TokenPlus => self.emit_byte(OpCode::OpAdd),
            TokenType::TokenMinus => self.emit_byte(OpCode::OpSubtract),
            TokenType::TokenStar => self.emit_byte(OpCode::OpMultiply),
            TokenType::TokenSlash => self.emit_byte(OpCode::OpDivide),
            TokenType::TokenPow => self.emit_byte(OpCode::OpPow),
            TokenType::TokenShiftLeft => self.emit_byte(OpCode::OpLeftShift),
            TokenType::TokenShiftRigth => self.emit_byte(OpCode::OpRightShift),
            TokenType::TokenBangEqual => self.emit_bytes(OpCode::OpEqual, OpCode::OpNot),
            TokenType::TokenEqualEqual => self.emit_byte(OpCode::OpEqual),
            TokenType::TokenGreater => self.emit_byte(OpCode::OpGreater),
            TokenType::TokenGreaterEqual => self.emit_bytes(OpCode::OpLess, OpCode::OpNot),
            TokenType::TokenLess => self.emit_byte(OpCode::OpLess),
            TokenType::TokenLessEqual => self.emit_bytes(OpCode::OpGreater, OpCode::OpNot),
            _ => return,
        }
    }

    fn literal(&mut self) -> () {
        match self.tokens[self.current - 1]._type {
            TokenType::TokenFalse => self.emit_byte(OpCode::OpFalse),
            TokenType::TokenTrue => self.emit_byte(OpCode::OpTrue),
            TokenType::TokenNil => self.emit_byte(OpCode::OpNil),
            _ => return,
        };
    }

    fn parse_string(&mut self) -> () {
        self.emit_constant(Value::Object(ObjString {
            chars: self.tokens[self.current - 1].source_str.clone(),
        }));
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> () {
        self.advance();
        let prefix_rule = match self.get_rule(self.tokens[self.current - 1]._type).prefix {
            Some(rule) => rule,
            None => {
                println!("On type: {}", self.tokens[self.current - 1]._type);
                self.error("Expected expression");
                return;
            }
        };

        prefix_rule(self);

        while self.is_lower_precedence(precedence) {
            self.advance();
            let infix_rule = self
                .get_rule(self.tokens[self.current - 1]._type)
                .infix
                .unwrap();
            infix_rule(self);
        }
    }

    fn get_rule(&self, _type: TokenType) -> ParseRule<'c> {
        return self.rules.get(&_type).cloned().unwrap();
    }

    fn is_lower_precedence(&self, precedence: Precedence) -> bool {
        let current_precedence = self.get_rule(self.tokens[self.current]._type).precedence;
        precedence <= current_precedence
    }

    fn parse_number(&mut self) -> () {
        let value = match self.tokens[self.current - 1].source_str.parse() {
            Ok(v) => v,
            Err(_) => 0.0, //TODO proper error handling
        };

        self.emit_constant(Value::Number(value));
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        let constant = u8::from(match self.chunk.add_constant(value) {
            Ok(v) => v,
            Err(_) => {
                println!("Too many constants in one chunk.");
                return 0;
            }
        });

        return constant - 1; //TODO
    }

    fn emit_byte(&mut self, byte: OpCode) -> () {
        self.chunk
            .write_chunk(byte, self.tokens[self.current - 1].line);
    }

    fn emit_bytes(&mut self, byte1: OpCode, byte2: OpCode) -> () {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_constant(&mut self, value: Value) -> () {
        let index = self.make_constant(value);
        self.emit_byte(OpCode::OpConstant(index));
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
