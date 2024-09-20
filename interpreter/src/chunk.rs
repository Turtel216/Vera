// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file.

// Byte code instructions
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum OpCode {
    OpReturn,
    OpConstant(u8),
    OpDefineGlobal(u8),
    OpGetGlobal(u8),
    OpSetGlobal(u8),
    OpGetLocal(u8),
    OpSetLocal(u8),
    OpNil,
    OpTrue,
    OpFalse,
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpPow,
    OpIncrement,
    OpDecrement,
    OpLeftShift,
    OpRightShift,
    OpNot,
    OpEqual,
    OpGreater,
    OpLess,
    OpPrint,
    OpPop,
    OpJumpIfFalse(u16),
    OpJump(u16),
    OpLoop(u16),
}

use std::fmt;

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpCode::OpReturn => write!(f, "OpReturn"),
            OpCode::OpConstant(_) => write!(f, "OpConstant"),
            OpCode::OpAdd => write!(f, "OpAdd"),
            OpCode::OpSubtract => write!(f, "OpSubtract"),
            OpCode::OpMultiply => write!(f, "OpMultiply"),
            OpCode::OpDivide => write!(f, "OpDivide"),
            OpCode::OpNegate => write!(f, "OpNegate"),
            OpCode::OpPow => write!(f, "OpPow"),
            OpCode::OpIncrement => write!(f, "OpIncrement"),
            OpCode::OpDecrement => write!(f, "OpDecrement"),
            OpCode::OpRightShift => write!(f, "OpRightShift"),
            OpCode::OpLeftShift => write!(f, "OpLeftShift"),
            OpCode::OpTrue => write!(f, "OpTrue"),
            OpCode::OpFalse => write!(f, "OpFalse"),
            OpCode::OpNil => write!(f, "OpNil"),
            OpCode::OpNot => write!(f, "OpNot"),
            OpCode::OpEqual => write!(f, "OpEqual"),
            OpCode::OpGreater => write!(f, "OpGreater"),
            OpCode::OpLess => write!(f, "OpLess"),
            OpCode::OpPrint => write!(f, "OpPrint"),
            OpCode::OpPop => write!(f, "OpPop"),
            OpCode::OpDefineGlobal(v) => write!(f, "OpDefineGlobal {}", v),
            OpCode::OpGetGlobal(v) => write!(f, "OpGetGlobal {}", v),
            OpCode::OpSetGlobal(v) => write!(f, "OpSetGlobal {}", v),
            OpCode::OpGetLocal(v) => write!(f, "OpGetLocal {}", v),
            OpCode::OpSetLocal(v) => write!(f, "OpSetLocal {}", v),
            OpCode::OpJumpIfFalse(v) => write!(f, "OpJumpIfFalse {}", v),
            OpCode::OpJump(v) => write!(f, "OpJump {}", v),
            OpCode::OpLoop(v) => write!(f, "OpLoop {}", v),
        }
    }
}

use std::num::TryFromIntError;
use std::usize;

use crate::value::Value;
use crate::value::ValueArray;

// Chunk of byte code
pub struct Chunk {
    pub code: Vec<OpCode>,     // Dynamic array if bytes
    pub constants: ValueArray, // Array of Vera values
    pub line: Vec<usize>,      // Line of each chunk in Vera source code
}

impl Clone for Chunk {
    fn clone(&self) -> Self {
        Chunk {
            code: self.code.clone(),
            constants: self.constants.clone(),
            line: self.line.clone(),
        }
    }
}

impl<'c> Chunk {
    // Create a new Chunk
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: ValueArray { array: Vec::new() },
            line: Vec::new(),
        }
    }
    // Append a byte to the chunk
    pub fn write_chunk(&mut self, byte: OpCode, line: usize) -> usize {
        self.code.push(byte);
        self.line.push(line);
        self.code.len() - 1
    }
    // Clear data hold by chunk
    pub fn free_chunk(&mut self) -> () {
        self.code.clear();
        self.constants.free_value_array();
    }
    // Add a constant value to chunk
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write_value_array(value);
        let result = self.constants.array.len() - 1;
        result
    }

    pub fn read_constant(&self, index: u8) -> Value {
        self.constants.array[index as usize].clone()
    }

    pub fn read_string(&self, index: u8) -> String {
        if let Value::Object(s) = self.read_constant(index) {
            s.chars
        } else {
            panic!("Constant is not String!");
        }
    }
}
