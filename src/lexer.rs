use crate::token::{Token, TokenKind};
pub struct Lexer{
    input: Vec<char>,
    current_position: usize,
    read_position: usize,
    ch: char
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Lexer{
        let mut lexer = Lexer{
            input,
            read_position: 0,
            current_position: 0,
            ch: Default::default()
        };

        lexer.read_ch();

        lexer
    }

    pub fn read_ch(&mut self){
        if self.read_position >= self.input.len(){
            self.ch = '\0'
        }
        self.ch = self.input[self.read_position];
        self.current_position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&self) -> Token{
        todo!()
    }
}

#[cfg(test)]
mod tests{
    use crate::token::{Token, TokenKind};

    use super::Lexer;

    #[test]
    fn test_next_token(){
        let input = "+=(),{};";

        let expected: Vec<Token> = vec![
            Token{
                kind: TokenKind::Assign,
                literal: "=".to_string()
            },
            Token{
                kind: TokenKind::Plus,
                literal: "+".to_string()
            },
            Token{
                kind: TokenKind::Lparen,
                literal: "(".to_string()
            },
            Token{
                kind: TokenKind::Rparen,
                literal: ")".to_string()
            },
            Token{
                kind: TokenKind::Comma,
                literal: ",".to_string()
            },
            Token{
                kind: TokenKind::Lbrace,
                literal: "{".to_string()
            },
            Token{
                kind: TokenKind::Rbrace,
                literal: "}".to_string()
            },
            Token{
                kind: TokenKind::Semicolon,
                literal: ";".to_string()
            },
            Token{
                kind: TokenKind::Eof,
                literal: "".to_string()
            }
        ];

        let input_to_lexer = input.chars().collect();

        let lexer = Lexer::new(input_to_lexer);

        for (idx, expected_token) in expected.into_iter().enumerate(){
            let got_token = lexer.next_token();
            assert_eq!(
                got_token.kind, 
                expected_token.kind,
                "test {}, token type wrong. Expected {:?} Got {:?}",
                idx,
                expected_token.kind,
                got_token.kind
            );
            assert_eq!(
                got_token.literal, 
                expected_token.literal,
                "test {}, token type wrong. Expected {:?} Got {:?}",
                idx,
                expected_token.literal,
                got_token.literal
            )
        }
    }
}