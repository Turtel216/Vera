// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file.

mod chunk;
mod lexer;
mod object;
mod parser;
mod value;
mod vm;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

use chunk::Chunk;
use vm::VM;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(()) => (),
            Err(e) => println!("Error: {e:?}"),
        }
    } else {
        println!("Usage: pf [path]");
    }
}

// Command line interpreter
fn repl() -> () {
    use std::io::{stdin, stdout, Write};
    let mut str = String::new();

    // Initialize vm
    let mut vm = VM {
        chunk: Chunk::new(),
        code: Vec::new(),
        ip: 0,
        stack: Vec::new(),
        current: 0,
        globals: HashMap::new(),
    };

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

        vm.interpret(&str);
        str.clear();
    }
}

// File interpreter
fn run_file(path: &String) -> std::io::Result<()> {
    //TODO add proper error handling
    let code = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            eprint!("Unable to read file {}: {}", path, error);
            process::exit(74);
        }
    };

    // Initialize vm
    let mut vm = VM {
        chunk: Chunk::new(),
        code: Vec::new(),
        ip: 0,
        stack: Vec::new(),
        current: 0,
        globals: HashMap::new(),
    };

    vm.interpret(&code);

    Ok(())
}
