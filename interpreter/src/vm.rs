// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file.

// Enum representing the possible outcomes of interpreting bytecode.
pub enum InterpretResult {
    InterpretOk,            // Execution completed successfully.
    InterpretCompileError,  // There was an error during the compilation phase.
    InterpretRuneTimeError, // An error occurred during execution.
}

use std::any::Any;
use std::collections::HashMap;

use crate::chunk::Chunk;
use crate::chunk::OpCode;
use crate::object::ObjString;
use crate::parser::Parser;

/// Virtual Machine (VM) for executing Vera bytecode in a stack-based architecture.
pub struct VM {
    pub chunk: Chunk,      // Byte code chunk
    pub code: Vec<OpCode>, // VM instructions
    pub stack: Vec<Value>, // VM value stack
    pub globals: HashMap<String, Value>,
    pub ip: usize,
}

use crate::lexer::Scanner;
use crate::value::Value;

impl VM {
    /// Executes bytecode instructions stored in the `chunk`.
    ///
    /// The function loops over each instruction, processes it,
    /// and handles various opcodes such as mathematical operations,
    /// stack manipulations, and conditional operations.
    ///
    /// Returns `InterpretResult` indicating the result of execution.
    fn run(&mut self) -> InterpretResult {
        // Loop over all instruction inside the byte code chunk
        // and execute them
        loop {
            match self.code[self.ip] {
                OpCode::OpReturn => {
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
                    // Get first two values
                    let (b, a) = (self.pop(), self.pop());
                    // Pattern match the String and number types while ensuring
                    // both a and b are the same type
                    match (&a, &b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.push(Value::Number(a + b));
                        }

                        (Value::Object(a), Value::Object(b)) => {
                            // Concatenate both strings
                            // and push the result onto the stack
                            let result = format!("{}{}", a, b);
                            self.push(Value::Object(ObjString { chars: result }));
                        }

                        _ => {
                            /* Consider adding the values
                            back onto the stack again
                            on failure TODO

                            self.push(a);
                            self.push(b);
                            */
                            self.runtime_error("Operand must be a Number or String.");
                            return InterpretResult::InterpretRuneTimeError;
                        }
                    }
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
                    //TODO ansure the numbers can be downcast to integer
                    //without loss of information

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
                    //TODO ansure the numbers can be downcast to integer
                    //without loss of information

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
                OpCode::OpTrue => self.push(Value::Bool(true)), // Push `true` onto the stack.
                OpCode::OpFalse => self.push(Value::Bool(false)), // Push `false` onto the stack.
                OpCode::OpNil => self.push(Value::Nil),         // Push `nil` onto the stack.
                OpCode::OpNot => {
                    // Negate the top boolean value or return `nil`.
                    let val = match self.pop() {
                        Value::Nil => Value::Nil,
                        Value::Bool(true) => Value::Bool(false),
                        Value::Bool(false) => Value::Bool(true),
                        _ => {
                            self.runtime_error("Operand must be a bool or nil.");
                            return InterpretResult::InterpretRuneTimeError;
                        }
                    };
                    self.push(val);
                }
                OpCode::OpEqual => {
                    // Get first 2 values from stack
                    // compare them and
                    // Push the result back onto the stack
                    let value_a = self.pop();
                    let value_b = self.pop();
                    self.push(Value::Bool(self.values_equal(value_a, value_b)))
                }
                OpCode::OpGreater => {
                    // Get first 2 values from stack
                    // compare them and
                    // Push the result back onto the stack
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
                    let result = value_a > value_b;
                    self.push(Value::Bool(result));
                }
                OpCode::OpLess => {
                    // Get first 2 values from stack
                    // compare them and
                    // Push the result back onto the stack
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
                    let result = value_a < value_b;
                    self.push(Value::Bool(result));
                }
                OpCode::OpPrint => Value::print_value(self.pop()),
                OpCode::OpPop => _ = self.pop(),
                OpCode::OpDefineGlobal(i) => {
                    let global_name = self.chunk.read_string(i);
                    let value = self.pop();
                    self.globals.insert(global_name, value);
                }
                OpCode::OpGetGlobal(i) => {
                    let global_name = self.chunk.read_string(i);
                    match self.globals.get(&global_name) {
                        Some(value) => self.push(value.clone()),
                        None => {
                            let msg = format!("Undefined variable '{}'.", global_name);
                            self.runtime_error(&msg);
                            return InterpretResult::InterpretRuneTimeError;
                        }
                    }
                }
                OpCode::OpSetGlobal(i) => {
                    let global_name = self.chunk.read_string(i);
                    match self.globals.get(&global_name) {
                        Some(value) => self.push(value.clone()),
                        None => {
                            let msg = format!("Undefined variable '{}'.", global_name);
                            self.runtime_error(&msg);
                            return InterpretResult::InterpretRuneTimeError;
                        }
                    }
                }
                OpCode::OpGetLocal(i) => {
                    let local_name = self.chunk.read_string(i);
                    match self.globals.get(&local_name) {
                        Some(value) => self.push(value.clone()),
                        None => {
                            let msg = format!("Undefined variable '{}'.", local_name);
                            self.runtime_error(&msg);
                            return InterpretResult::InterpretRuneTimeError;
                        }
                    }
                }
                OpCode::OpSetLocal(i) => {
                    let local_name = self.chunk.read_string(i);
                    match self.globals.get(&local_name) {
                        Some(value) => self.push(value.clone()),
                        None => {
                            let msg = format!("Undefined variable '{}'.", local_name);
                            self.runtime_error(&msg);
                            return InterpretResult::InterpretRuneTimeError;
                        }
                    }
                }
                OpCode::OpJumpIfFalse(offset) => {
                    if self.peek(0).is_falsey() {
                        self.ip += offset as usize; //TODO
                    }
                }
                OpCode::OpJump(offset) => {
                    self.ip += offset as usize;
                }
                OpCode::OpLoop(offset) => {
                    self.ip -= offset as usize + 1;
                }
            }
            // Continue to next instruction
            self.ip += 1;
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
        let mut parser = Parser::new(tokens, &mut chunk);

        // Compile tokens
        if !parser.compile() {
            chunk.free_chunk();
            return InterpretResult::InterpretCompileError;
        }

        // Init vm
        self.chunk = chunk;
        self.code = self.chunk.code.clone();

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

    fn peek(&self, n: usize) -> Value {
        let size = self.stack.len();
        self.stack[size - 1 - n].clone()
    }

    // Check if two value types are equal
    fn values_equal(&self, a: Value, b: Value) -> bool {
        // Check if the two values have the same type
        if a.type_id() != b.type_id() {
            return false;
        }

        return a == b;
    }

    fn runtime_error(&self, msg: &str) -> () {
        println!("{}", msg);
    }
}
