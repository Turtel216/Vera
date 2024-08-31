pub struct Value {
    value: f64,
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
    pub fn write_value_array(&mut self, value: Value) -> () {
        self.array.push(value);
    }
    pub fn free_value_array(&mut self) -> () {
        self.array.clear();
    }
}
