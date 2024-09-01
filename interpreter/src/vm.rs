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
    pub stack: Vec<Value>,
}

use crate::value::Value;

impl<'v> VM<'v> {
    // Close vm
    pub fn free_vm(&mut self) -> () {}

    // Run vm instuctions
    fn run(&mut self) -> InterpretResult {
        loop {
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
    pub fn interpret(&mut self, chunk: &'v mut Chunk) -> InterpretResult {
        self.chunk = chunk;

        // Add all op codes to vm instraction codes
        let code_iter = self.chunk.code.iter().rev();
        for op_code in code_iter {
            self.ip.push(*op_code);
        }

        // Add all constants to vm stack
        let _value_iter = self.chunk.constants.array.iter().rev();
        for value_ in _value_iter {
            self.stack.push(*value_);
        }

        self.run()
    }

    // Clear stack
    fn reset_stack(&mut self) -> () {
        self.stack.clear();
    }

    // Push onto stack
    pub fn push(&mut self, value: Value) -> () {
        self.stack.push(value);
    }
    pub fn pop(&mut self) -> Value {
        self.stack.pop().expect("Couldn't Pop from VM stack")
    }
}
