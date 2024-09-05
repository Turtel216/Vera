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
use crate::compiler::Compiler;

// Vera stack based Virtual Machine
pub struct VM {
    pub chunk: Chunk,      // Byte code chunk
    pub ip: Vec<OpCode>,   // VM instructions
    pub stack: Vec<Value>, // VM value stack
    pub current: usize,    // Index of current instruction
}

use crate::lexer::Scanner;
use crate::value::Value;

impl VM {
    // Close vm
    // TODO
    pub fn free_vm(&mut self) -> () {}

    // Run vm instuctions
    fn run(&mut self) -> InterpretResult {
        // Loop over all instruction inside the byte code chunk
        // and execute them
        loop {
            match self.ip[self.current] {
                OpCode::OpReturn => {
                    //TODO
                    Value::print_value(self.pop());
                    return InterpretResult::InterpretOk;
                }
                OpCode::OpConstant(index) => {
                    // Get the value specified by the given index
                    // from the byte code chunk and push it onto the stack
                    let value = self.chunk.read_constant(index);
                    self.push(value);
                }
                OpCode::OpNegate => {
                    // Get first value from stack
                    // negate it by 1
                    // Push it back onto the stack
                    let mut value = self.pop();
                    value.value = value.value - 1.0;
                    self.push(value);
                }
                OpCode::OpAdd => {
                    // Get first 2 values from stack
                    // add them together
                    // Push them back onto the stack
                    let value_a = self.pop();
                    let value_b = self.pop();
                    let add = value_a.value + value_b.value;
                    self.push(Value { value: add });
                }
                OpCode::OpSubtract => {
                    // Get first 2 values from stack
                    // substract them
                    // Push them back onto the stack
                    let value_a = self.pop();
                    let value_b = self.pop();
                    let sub = value_b.value - value_a.value;
                    self.push(Value { value: sub });
                }
                OpCode::OpMultiply => {
                    // Get first 2 values from stack
                    // multiply them
                    // Push them back onto the stack
                    let value_a = self.pop();
                    let value_b = self.pop();
                    let mult = value_a.value * value_b.value;
                    self.push(Value { value: mult });
                }
                OpCode::OpDivide => {
                    // Get first 2 values from stack
                    // Devide them
                    // Push them back onto the stack
                    let value_a = self.pop();
                    let value_b = self.pop();
                    let div = value_a.value / value_b.value;
                    self.push(Value { value: div });
                }
                _ => return InterpretResult::InterpretRuneTimeError,
            }
            // Continue to next instruction
            self.current += 1;
        }
    }
    // Interpret a chunk of bytecode
    pub fn interpret(&mut self, source: &String) -> InterpretResult {
        // Create new byte chunk to hold incoming instruction
        let mut chunk = Chunk::new();

        // Generate Tokens from source while
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        // Create parser
        let mut parser = Compiler::new(tokens, &mut chunk);

        // Compile tokens
        if !parser.compile() {
            chunk.free_chunk();
            return InterpretResult::InterpretCompileError;
        }

        // Init vm
        self.chunk = chunk;
        self.ip = self.chunk.code.clone();

        // Run instructions
        let result = self.run();

        return result;
    }

    // push onto value stack
    pub fn push(&mut self, value: Value) -> () {
        self.stack.push(value);
    }

    // pop from value stack
    pub fn pop(&mut self) -> Value {
        self.stack.pop().expect("Couldn't Pop from VM stack")
    }
}
