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
}

use crate::lexer::Scanner;
use crate::value::Value;

impl VM {
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
                Some(OpCode::OpConstant(index)) => {
                    let value = self.chunk.read_constant(index);
                    self.push(value);
                }
                Some(OpCode::OpNegate) => {
                    let mut value = self.pop();
                    value.value = value.value - 1.0;
                    self.push(value);
                }
                Some(OpCode::OpAdd) => {
                    let value_a = self.pop();
                    println!("Value a: {}", value_a.value);
                    let value_b = self.pop();
                    println!("Value b: {}", value_b.value);
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
                Some(_) => todo!(),
                None => return InterpretResult::InterpretRuneTimeError,
            }
        }
    }
    // Interpret a chunk of bytecode
    pub fn interpret(&mut self, source: &String) -> InterpretResult {
        // Compile source file
        let mut chunk = Chunk::new();

        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        let mut parser = Compiler::new(tokens, &mut chunk);

        // Compile source string
        if !parser.compile() {
            chunk.free_chunk();
            return InterpretResult::InterpretCompileError;
        }

        chunk.code.reverse();

        // Init vm
        self.chunk = chunk;
        self.ip = self.chunk.code.clone();

        // Run instructions
        let result = self.run();

        //chunk.free_chunk();

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
