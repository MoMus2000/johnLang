use std::io::{Stdin, Stdout, Write};

use crate::{lexer::Lexer, token::TokenKind};

pub fn start(stdin: Stdin, mut stdout: Stdout){
    loop{
        write!(stdout, ">> ").expect("Write to the terminal");
        stdout.flush().expect("Flushed");
        let mut input = String::new();

        stdin.read_line(&mut input).expect("ERROR: Reading from the stdin");

        let input = input.chars().collect();
        let mut lexer = Lexer::new(input);
        loop{
            let token = lexer.next_token();
            if token.kind == TokenKind::Eof{
                break
            }else{
                writeln!(stdout, "{:?}", token).expect("ERROR writing token to screen");
            }
        }

    }
}