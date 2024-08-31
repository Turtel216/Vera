// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file.

mod chunk;
mod lexer;
mod value;
mod vm;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use chunk::Chunk;
use vm::VM;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Initialize vm
    let mut _vm = VM {
        chunk: &mut Chunk::new(),
        ip: Vec::new(),
        stack: Vec::new(),
    };

    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(()) => println!("Success"),
            Err(e) => println!("Error: {e:?}"),
        }
    } else {
        println!("Usage: pf [path]");
    }

    // Close VM
    _vm.free_vm();
}

// Command line interpreter
fn repl() -> () {
    use std::io::{stdin, stdout, Write};
    let mut str = String::new();

    loop {
        print!("> ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut str)
            .expect("Did not enter a corrent string.");

        if let Some('\n') = str.chars().next_back() {
            str.pop();
        }
        if let Some('\r') = str.chars().next_back() {
            str.pop();
        }

        //let _ = vm::interpret(&str);
        str.clear();
    }
}

// File interpreter
fn run_file(_path: &String) -> std::io::Result<()> {
    // Read from file
    let file = File::open(_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    // Interpret each line
    let lines = contents.lines();
    for line in lines {
        //vm::interpret(&line.to_string());
    }
    Ok(())
}
