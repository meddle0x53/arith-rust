#![feature(box_patterns)]
#![feature(box_syntax)]

#![feature(libc)]

mod lexer;
mod parser;
mod interpretator;
mod repl;

fn start_repl() {
    println!("\nWelcome to the Arith REPL!");
    repl::start("> ", (|s| interpretator::run(&s)))
}

fn main() {
    start_repl();
}
