use crate::lexer::Scanner;

pub fn compile(source: &String) -> () {
    let mut lexer = Scanner::new(source);
    let tokens = lexer.scan_tokens();

    for token in tokens {
        //TODO remove, only tmp
        println!("{} : {} : {}", token.source_str, token._type, token.line);
    }
}
