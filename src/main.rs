pub mod token;
pub mod lexer;
pub mod repl;

use std::io;

use crate::repl::start;

fn main() {
    println!("JohnLang V0.0.1, all rights reversed so fuck off");
    start(io::stdin(), io::stdout());
}
