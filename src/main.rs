use std::io;

mod ast;
mod builtins;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod repl;
mod token;

/// Entry point for the Monkey interpreter REPL.
fn main() {
    use crate::repl::start;
    println!("Hello! This is the Monkey Programming language!");
    println!("Feel free to type in the code");
    start(io::stdin(), io::stdout());
}
