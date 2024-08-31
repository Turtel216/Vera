// Byte code instructions
pub enum OpCode {
    OpReturn,
    OpConstant,
}

use std::usize;

use crate::value::Value;
use crate::value::ValueArray;

// Chunk of byte code
pub struct Chunk {
    code: Vec<u8>,         // Dynamic array if bytes
    constants: ValueArray, // Array of Vera values
    line: Vec<usize>,      // Line of each chunk in Vera source code
}

impl Chunk {
    // Append a byte to the chunk
    pub fn write_chunk(&mut self, byte: u8, line: usize) -> () {
        self.code.push(byte);
        self.line.push(line);
    }
    // Clear data hold by chunk
    pub fn free_chunk(&mut self) -> () {
        self.code.clear();
        self.constants.free_value_array();
    }
    // Add a constant value to chunk
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write_value_array(value);
        self.constants.array.len()
    }
}
