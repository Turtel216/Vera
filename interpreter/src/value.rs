// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file

#[derive(Copy, Clone)]
pub enum Value {
    Bool(bool),
    Number(f64),
}

impl Value {
    pub fn print_value(self) -> () {
        match self {
            Value::Bool(v) => println!("{}", v),
            Value::Number(v) => println!("{}", v),
        }
    }
}

pub struct ValueArray {
    pub array: Vec<Value>,
}

impl Clone for ValueArray {
    fn clone(&self) -> Self {
        ValueArray {
            array: self.array.clone(),
        }
    }
}

impl ValueArray {
    pub fn write_value_array(&mut self, value: Value) -> () {
        self.array.push(value);
    }
    pub fn free_value_array(&mut self) -> () {
        self.array.clear();
    }
}
