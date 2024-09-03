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
pub struct VM<'v> {
    pub chunk: &'v mut Chunk, // Byte code chunk
    pub ip: Vec<OpCode>,      // VM instructions
    pub stack: Vec<Value>,    // VM value stack
}

use crate::value::Value;

impl<'v> VM<'v> {
    // Close vm
    // TODO
    pub fn free_vm(&mut self) -> () {}

    // Run vm instuctions
    fn run(&mut self) -> InterpretResult {
        loop {
            // Execute next instruction
            match self.ip.pop() {
                Some(OpCode::OpReturn) => {
                    Value::print_value(self.pop());
                    return InterpretResult::InterpretOk;
                }
                Some(OpCode::OpConstant) => {
                    let constant = self
                        .chunk
                        .constants
                        .array
                        .pop()
                        .expect("Couldn't retrieve value from constants array");

                    self.push(constant)
                }
                Some(OpCode::OpNegate) => {
                    let mut value = self.pop();
                    value.value = value.value - 1.0;
                    self.push(value);
                }
                Some(OpCode::OpAdd) => {
                    let value_a = self.pop();
                    let value_b = self.pop();
                    let add = value_a.value + value_b.value;
                    self.push(Value { value: add });
                }
                Some(OpCode::OpSubtract) => {
                    let value_a = self.pop();
                    let value_b = self.pop();
                    let sub = value_a.value - value_b.value;
                    self.push(Value { value: sub });
                }
                Some(OpCode::OpMultiply) => {
                    let value_a = self.pop();
                    let value_b = self.pop();
                    let mult = value_a.value * value_b.value;
                    self.push(Value { value: mult });
                }
                Some(OpCode::OpDivide) => {
                    let value_a = self.pop();
                    let value_b = self.pop();
                    let div = value_a.value / value_b.value;
                    self.push(Value { value: div });
                }
                None => return InterpretResult::InterpretRuneTimeError,
            }
        }
    }
    // Interpret a chunk of bytecode
    pub fn interpret(&mut self, _source: &String) -> InterpretResult {
        // Compile source file
        let chunk = Chunk::new();

        // Compile source string
        if !Compiler::compile(_source, &chunk) {
            chunk.free_chunk();
            return InterpretResult::InterpretCompileError;
        }

        // Init vm
        self.chunk = &mut chunk;
        self.ip = self.chunk.code;

        // Run instructions
        let result = self.run();

        chunk.free_chunk();

        return result;
    }

    // Clear stack
    fn reset_stack(&mut self) -> () {
        self.stack.clear();
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
