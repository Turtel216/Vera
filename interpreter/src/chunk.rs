// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file.

// Byte code instructions
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum OpCode {
    OpReturn,
    OpConstant(u8),
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
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
    pub fn write_chunk(&mut self, byte: OpCode, line: usize) -> () {
        self.code.push(byte);
        self.line.push(line);
    }
    // Clear data hold by chunk
    pub fn free_chunk(&mut self) -> () {
        self.code.clear();
        self.constants.free_value_array();
    }
    // Add a constant value to chunk
    pub fn add_constant(&mut self, value: Value) -> Result<u8, TryFromIntError> {
        self.constants.write_value_array(value);
        let result = self.constants.array.len().try_into();
        result
    }

    pub fn read_constant(&self, index: u8) -> Value {
        self.constants.array[index as usize]
    }
}
