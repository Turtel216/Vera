mod lexer;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use lexer::Scanner;

fn main() {
    let args: Vec<String> = env::args().collect();

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

        println!("typed: {}", str); //TODO change to new line
        str.clear();
    }
}

// File interpreter
fn run_file(_path: &String) -> std::io::Result<()> {
    let file = File::open(_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    println!("{contents}");
    Ok(())
}
