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
            self.ch = '\0';
            return
        }
        self.ch = self.input[self.read_position];
        self.current_position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token{
        println!("Current char: {}", self.ch);
        let token = match self.ch {
            '=' => Lexer::new_token(TokenKind::Assign, "=".to_string()),
            '+' => Lexer::new_token(TokenKind::Plus, "+".to_string()),
            '(' => Lexer::new_token(TokenKind::Lparen, "(".to_string()),
            ')' => Lexer::new_token(TokenKind::Rparen, ")".to_string()),
            '{' => Lexer::new_token(TokenKind::Lbrace, "{".to_string()),
            '}' => Lexer::new_token(TokenKind::Rbrace, "}".to_string()),
            ',' => Lexer::new_token(TokenKind::Comma, ",".to_string()),
            ';' => Lexer::new_token(TokenKind::Semicolon, ";".to_string()),
            '\0' => Lexer::new_token(TokenKind::Eof, "".to_string()),
            _ =>  Lexer::new_token(TokenKind::Illegal, self.ch.to_string())
        };

        self.read_ch();

        token
    }

    fn new_token(token_kind: TokenKind, token_literal: String) -> Token{
        Token { kind: token_kind, literal: token_literal }
    }

}

#[cfg(test)]
mod tests{
    use crate::token::{Token, TokenKind};

    use super::Lexer;

    #[test]
    fn test_next_token(){
        let input = "=+(),{};";

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

        let mut lexer = Lexer::new(input_to_lexer);

        for (idx, expected_token) in expected.into_iter().enumerate(){
            let got_token = lexer.next_token();
            assert_eq!(
                got_token.kind, 
                expected_token.kind,
                "test-kind {}, token type wrong. Expected {:?} Got {:?}",
                idx,
                expected_token.kind,
                got_token.kind
            );
            assert_eq!(
                got_token.literal, 
                expected_token.literal,
                "test-literal {}, token type wrong. Expected {:?} Got {:?}",
                idx,
                expected_token.literal,
                got_token.literal
            )
        }
    }
}