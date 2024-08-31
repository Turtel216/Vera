// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file.

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuneTimeError,
}

struct VM {
    ip: u8,
}

pub fn interpret(source: &String) -> InterpretResult {
    let vm = VM::new(2);
    vm.run()
}

impl VM {
    fn new(ip: u8) -> VM {
        VM { ip }
    }
    fn run(self) -> InterpretResult {
        InterpretResult::InterpretOk
    }
}
