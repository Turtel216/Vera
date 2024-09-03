use crate::{
    chunk::Chunk,
    lexer::{Token, TokenType},
};

pub struct Compiler<'c> {
    current: usize,
    previous: usize,
    tokens: &'c Vec<Token>,
}

impl<'c> Compiler<'c> {
    pub fn new(tokens: &'c Vec<Token>) -> Self {
        return Compiler {
            tokens,
            current: 0,
            previous: 0,
        };
    }
    pub fn compile(&mut self, source: &'c String, chunk: Chunk) -> bool {
        self.advance();
        self.expression();
        self.consume(TokenType::TokenEOF, "Expected end of expression.");

        true
    }

    fn advance(&mut self) -> () {
        for token in self.tokens {
            if token._type != TokenType::TokenError {
                break;
            }

            // Error encountered, throw compiler error
            Compiler::error_at_current(&token.source_str);
        }
    }

    fn expression(&mut self) -> () {}

    fn consume(&mut self, _type: TokenType, _msg: &str) -> () {}

    fn error_at_current(msg: &'c str) -> () {
        panic!("error on Token string: {}", msg); //TODO
    }
}
