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
                    // Check if its a number
                    // negate it by 1
                    // Push it back onto the stack
                    let mut value = match self.pop() {
                        Value::Number(v) => v,
                        _ => {
                            self.runtime_error("Operand must be a number.");
                            return InterpretResult::InterpretRuneTimeError;
                        }
                    };

                    value = -value;
                    self.push(Value::Number(value));
                }
                OpCode::OpDecrement => {
                    // Get first value from stack
                    // Check if its a number
                    // decrement it by 1
                    // Push it back onto the stack
                    let mut value = match self.pop() {
                        Value::Number(v) => v,
                        _ => {
                            self.runtime_error("Operand must be a number.");
                            return InterpretResult::InterpretRuneTimeError;
                        }
                    };

                    value = value - 1.0;
                    self.push(Value::Number(value));
                }
                OpCode::OpIncrement => {
                    // Get first value from stack
                    // Check if its a number
                    // increment it by 1
                    // Push it back onto the stack
                    let mut value = match self.pop() {
                        Value::Number(v) => v,
                        _ => {
                            self.runtime_error("Operand must be a number.");
                            return InterpretResult::InterpretRuneTimeError;
                        }
                    };

                    value = value + 1.0;
                    self.push(Value::Number(value));
                }
                OpCode::OpAdd => {
                    // Get first 2 values from stack
                    // Push them back onto the stack
                    let value_a = match self.pop() {
                        // Check for valid types
                        Value::Number(v) => v,
                        _ => {
                            self.runtime_error("Operand must be a number");
                            return InterpretResult::InterpretRuneTimeError;
                        }
                    };
                    let value_b = match self.pop() {
                        // Check for valid types
                        Value::Number(v) => v,
                        _ => {
                            self.runtime_error("Operand must be a number");
                            return InterpretResult::InterpretRuneTimeError;
                        }
                    };
                    let add = value_a + value_b;
                    self.push(Value::Number(add));
                }
                OpCode::OpSubtract => {
                    // Get first 2 values from stack
                    // substract them
                    // Push them back onto the stack
                    let value_a = match self.pop() {
                        // Check for valid types
                        Value::Number(v) => v,
                        _ => {
                            self.runtime_error("Operand must be a number");
                            return InterpretResult::InterpretRuneTimeError;
                        }
                    };
                    let value_b = match self.pop() {
                        // Check for valid types
                        Value::Number(v) => v,
                        _ => {
                            self.runtime_error("Operand must be a number");
                            return InterpretResult::InterpretRuneTimeError;
                        }
                    };
                    let sub = value_a - value_b;
                    self.push(Value::Number(sub));
                }
                OpCode::OpMultiply => {
                    // Get first 2 values from stack
                    // multiply them
                    // Push them back onto the stack
                    let value_a = match self.pop() {
                        // Check for valid types
                        Value::Number(v) => v,
                        _ => {
                            self.runtime_error("Operand must be a number");
                            return InterpretResult::InterpretRuneTimeError;
                        }
                    };
                    let value_b = match self.pop() {
                        // Check for valid types
                        Value::Number(v) => v,
                        _ => {
                            self.runtime_error("Operand must be a number");
                            return InterpretResult::InterpretRuneTimeError;
                        }
                    };
                    let mult = value_a * value_b;
                    self.push(Value::Number(mult));
                }
                OpCode::OpDivide => {
                    // Get first 2 values from stack
                    // Devide them
                    // Push them back onto the stack
                    let value_a = match self.pop() {
                        // Check for valid types
                        Value::Number(v) => v,
                        _ => {
                            self.runtime_error("Operand must be a number");
                            return InterpretResult::InterpretRuneTimeError;
                        }
                    };
                    let value_b = match self.pop() {
                        // Check for valid types
                        Value::Number(v) => v,
                        _ => {
                            self.runtime_error("Operand must be a number");
                            return InterpretResult::InterpretRuneTimeError;
                        }
                    };
                    let div = value_a / value_b;
                    self.push(Value::Number(div));
                }
                OpCode::OpLeftShift => {
                    // Get first 2 values from stack
                    // bit shift them
                    // Push them back onto the stack
                    let value_a = match self.pop() {
                        // Check for valid types
                        Value::Number(v) => v,
                        _ => {
                            self.runtime_error("Operand must be a number");
                            return InterpretResult::InterpretRuneTimeError;
                        }
                    };
                    let value_b = match self.pop() {
                        // Check for valid types
                        Value::Number(v) => v,
                        _ => {
                            self.runtime_error("Operand must be a number");
                            return InterpretResult::InterpretRuneTimeError;
                        }
                    };
                    let shift = (value_a as isize) << (value_b as isize);
                    self.push(Value::Number(shift as f64));
                }
                OpCode::OpRightShift => {
                    // Get first 2 values from stack
                    // bit shift them
                    // Push them back onto the stack
                    let value_a = match self.pop() {
                        // Check for valid types
                        Value::Number(v) => v,
                        _ => {
                            self.runtime_error("Operand must be a number");
                            return InterpretResult::InterpretRuneTimeError;
                        }
                    };
                    let value_b = match self.pop() {
                        // Check for valid types
                        Value::Number(v) => v,
                        _ => {
                            self.runtime_error("Operand must be a number");
                            return InterpretResult::InterpretRuneTimeError;
                        }
                    };
                    let shift = (value_a as isize) >> (value_b as isize);
                    self.push(Value::Number(shift as f64));
                }
                OpCode::OpPow => {
                    // Get first 2 values from stack
                    // calculate the one to the power of the other
                    // Push them back onto the stack
                    let value_a = match self.pop() {
                        // Check for valid types
                        Value::Number(v) => v,
                        _ => {
                            self.runtime_error("Operand must be a number");
                            return InterpretResult::InterpretRuneTimeError;
                        }
                    };
                    let value_b = match self.pop() {
                        // Check for valid types
                        Value::Number(v) => v,
                        _ => {
                            self.runtime_error("Operand must be a number");
                            return InterpretResult::InterpretRuneTimeError;
                        }
                    };
                    let pow = value_a.powf(value_b);
                    self.push(Value::Number(pow));
                }
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
        self.current = 0;

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

    fn runtime_error(&self, msg: &str) -> () {
        println!("{}", msg);
    }
}
