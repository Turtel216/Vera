// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file

#[derive(Copy, Clone)]
pub struct Value {
    pub value: f64,
}

impl Value {
    pub fn print_value(self) -> () {
        println!("{}", self.value);
    }
}

pub struct ValueArray {
    pub array: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> ValueArray {
        ValueArray { array: Vec::new() }
    }
    pub fn write_value_array(&mut self, value: Value) -> () {
        self.array.push(value);
    }
    pub fn free_value_array(&mut self) -> () {
        self.array.clear();
    }
}
