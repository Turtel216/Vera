// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file.

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuneTimeError,
}

use crate::chunk::Chunk;
use crate::chunk::OpCode;

pub struct VM<'v> {
    pub chunk: &'v mut Chunk,
    pub ip: Vec<OpCode>,
}

use crate::value::Value;

impl<'v> VM<'v> {
    // Close vm
    pub fn free_vm(&mut self) -> () {}

    // Run vm instuctions
    fn run(&mut self) -> InterpretResult {
        loop {
            match self.ip.pop() {
                Some(OpCode::OpReturn) => return InterpretResult::InterpretOk,
                Some(OpCode::OpConstant) => {
                    Value::print_value(
                        self.chunk
                            .constants
                            .array
                            .pop()
                            .expect("Couldn't retrieve value from constants array"),
                    );
                    println!("")
                }
                None => return InterpretResult::InterpretRuneTimeError,
            }
        }
    }
    // Interpret a chunk of bytecode
    pub fn interpret(&mut self, chunk: &'v mut Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.ip.push(
            self.chunk
                .code
                .pop()
                .expect("Error getting byte code instruction"),
        );
        self.run()
    }
}
