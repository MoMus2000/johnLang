use crate::token::{Token, TokenKind, match_literal_to_kind};
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
        self.skip_whitespace();

        let token = match self.ch {
            '=' =>{
                if self.peek_ahead() == '='{
                    self.read_ch();
                    Lexer::new_token(TokenKind::Eq, "==".to_string())
                }else{
                    Lexer::new_token(TokenKind::Assign, "=".to_string())
                }
            },
            '+' => Lexer::new_token(TokenKind::Plus, "+".to_string()),
            '(' => Lexer::new_token(TokenKind::Lparen, "(".to_string()),
            ')' => Lexer::new_token(TokenKind::Rparen, ")".to_string()),
            '{' => Lexer::new_token(TokenKind::Lbrace, "{".to_string()),
            '}' => Lexer::new_token(TokenKind::Rbrace, "}".to_string()),
            ',' => Lexer::new_token(TokenKind::Comma, ",".to_string()),
            ';' => Lexer::new_token(TokenKind::Semicolon, ";".to_string()),
            '-' => Lexer::new_token(TokenKind::Minus, "-".to_string()),
            '/' => Lexer::new_token(TokenKind::Slash, "/".to_string()),
            '*' => Lexer::new_token(TokenKind::Asterisk, "*".to_string()),
            '!' => {
                if self.peek_ahead() == '='{
                    self.read_ch();
                    Lexer::new_token(TokenKind::NotEq, "!=".to_string())
                }
                else{
                    Lexer::new_token(TokenKind::Bang, "!".to_string())
                }
            },
            '<' => {
                if self.peek_ahead() == '='{
                    self.read_ch();
                    Lexer::new_token(TokenKind::LtEq, "<=".to_string())
                }
                else{
                    Lexer::new_token(TokenKind::Lt, "<".to_string())
                }
            },
            '>' => {
                if self.peek_ahead() == '='{
                    self.read_ch();
                    Lexer::new_token(TokenKind::GtEq, ">=".to_string())
                }
                else{
                    Lexer::new_token(TokenKind::Gt, ">".to_string())
                }
            },
            '\0' => Lexer::new_token(TokenKind::Eof, "".to_string()),
            _ =>{
                if Lexer::is_letter(self.ch){
                    let literal = self.read_identifier();
                    let kind = match_literal_to_kind(&literal);
                    return Token{kind, literal}
                }
                else if Lexer::is_digit(self.ch){
                    let kind = TokenKind::Int;
                    let literal = self.read_number();
                    return Token{kind, literal}
                }
                else{
                    return Lexer::new_token(TokenKind::Illegal, self.ch.to_string())
                }
            } 
        };

        self.read_ch();

        token
    }

    fn read_number(&mut self) -> String{
        let mut num = "".to_string();
        while Lexer::is_digit(self.ch) {
            num.push(self.ch);
            self.read_ch();
        }
        num
    }

    fn peek_ahead(&self) -> char {
        if self.read_position >= self.input.len(){
            return '\0'
        }else{
            return self.input[self.read_position]
        }
    }

    fn is_digit(ch: char) -> bool {
        return ch.is_numeric()
    }

    fn read_identifier(&mut self) -> String{
        let mut identifier = "".to_string();
        while Lexer::is_letter(self.ch) {
            identifier.push(self.ch);
            self.read_ch();
        }
        identifier
    }

    fn is_letter(ch: char) -> bool{
        return ch.is_alphabetic() || ch == '_';
    }

    fn skip_whitespace(&mut self){
        while self.ch.is_ascii_whitespace(){
            self.read_ch();
        }
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
    #[test]
    fn test_multiline_text(){
        let input = r#"
        let five = 5;

        let seven = 7;

        let add = fn(x,y){
            x+y;
        }

        let result = add(five, seven);
        "#;

        let expected: Vec<Token> = vec![
            Token{
                kind: TokenKind::Let,
                literal : "let".to_string()
            },
            Token{
                kind: TokenKind::Ident,
                literal : "five".to_string()
            },
            Token{
                kind: TokenKind::Assign,
                literal : "=".to_string()
            },
            Token{
                kind: TokenKind::Int,
                literal : "5".to_string()
            },
            Token{
                kind: TokenKind::Semicolon,
                literal : ";".to_string()
            },

            Token{
                kind: TokenKind::Let,
                literal : "let".to_string()
            },
            Token{
                kind: TokenKind::Ident,
                literal : "seven".to_string()
            },
            Token{
                kind: TokenKind::Assign,
                literal : "=".to_string()
            },
            Token{
                kind: TokenKind::Int,
                literal : "7".to_string()
            },
            Token{
                kind: TokenKind::Semicolon,
                literal : ";".to_string()
            },

            Token{
                kind: TokenKind::Let,
                literal : "let".to_string()
            },
            Token{
                kind: TokenKind::Ident,
                literal : "add".to_string()
            },
            Token{
                kind: TokenKind::Assign,
                literal : "=".to_string()
            },
            Token{
                kind: TokenKind::Fn,
                literal : "fn".to_string()
            },
            Token{
                kind: TokenKind::Lparen,
                literal : "(".to_string()
            },
            Token{
                kind: TokenKind::Ident,
                literal : "x".to_string()
            },
            Token{
                kind: TokenKind::Comma,
                literal : ",".to_string()
            },
            Token{
                kind: TokenKind::Ident,
                literal : "y".to_string()
            },
            Token{
                kind: TokenKind::Rparen,
                literal : ")".to_string()
            },
            Token{
                kind: TokenKind::Lbrace,
                literal : "{".to_string()
            },
            Token{
                kind: TokenKind::Ident,
                literal : "x".to_string()
            },
            Token{
                kind: TokenKind::Plus,
                literal : "+".to_string()
            },
            Token{
                kind: TokenKind::Ident,
                literal : "y".to_string()
            },
            Token{
                kind: TokenKind::Semicolon,
                literal : ";".to_string()
            },
            Token{
                kind: TokenKind::Rbrace,
                literal : "}".to_string()
            },


            Token{
                kind: TokenKind::Let,
                literal : "let".to_string()
            },
            Token{
                kind: TokenKind::Ident,
                literal : "result".to_string()
            },
            Token{
                kind: TokenKind::Assign,
                literal : "=".to_string()
            },
            Token{
                kind: TokenKind::Ident,
                literal : "add".to_string()
            },
            Token{
                kind: TokenKind::Lparen,
                literal : "(".to_string()
            },
            Token{
                kind: TokenKind::Ident,
                literal : "five".to_string()
            },
            Token{
                kind: TokenKind::Comma,
                literal : ",".to_string()
            },
            Token{
                kind: TokenKind::Ident,
                literal : "seven".to_string()
            },
            Token{
                kind: TokenKind::Rparen,
                literal : ")".to_string()
            },
            Token{
                kind: TokenKind::Semicolon,
                literal : ";".to_string()
            },
        ];

        let input_to_lexer = input.chars().collect();

        let mut lexer = Lexer::new(input_to_lexer);

        for (idx, expected_token) in expected.into_iter().enumerate(){
            let got_token = lexer.next_token();
            assert_eq!(
                got_token.kind, 
                expected_token.kind,
                "test-multiline-kind {}, token type wrong. Expected {:?} Got {:?}",
                idx,
                expected_token.kind,
                got_token.kind
            );
            assert_eq!(
                got_token.literal, 
                expected_token.literal,
                "test-multiline-literal {}, token type wrong. Expected {:?} Got {:?}",
                idx,
                expected_token.literal,
                got_token.literal
            )
        }
    }

    #[test]
    fn test_extended_symbols(){
        let input = "*/-!<>";
        let expected : Vec<Token> = vec![
            Token{
                kind: TokenKind::Asterisk,
                literal : "*".to_string()
            },
            Token{
                kind: TokenKind::Slash,
                literal : "/".to_string()
            },
            Token{
                kind: TokenKind::Minus,
                literal : "-".to_string()
            },
            Token{
                kind: TokenKind::Bang,
                literal : "!".to_string()
            },
            Token{
                kind: TokenKind::Lt,
                literal : "<".to_string()
            },
            Token{
                kind: TokenKind::Gt,
                literal : ">".to_string()
            },
        ];
        let input_to_lexer = input.chars().collect();

        let mut lexer = Lexer::new(input_to_lexer);

        for (idx, expected_token) in expected.into_iter().enumerate(){
            let got_token = lexer.next_token();
            assert_eq!(
                got_token.kind, 
                expected_token.kind,
                "test-extended-symbols-kind {}, token type wrong. Expected {:?} Got {:?}",
                idx,
                expected_token.kind,
                got_token.kind
            );
            assert_eq!(
                got_token.literal, 
                expected_token.literal,
                "test-extended-symbols-literal {}, token type wrong. Expected {:?} Got {:?}",
                idx,
                expected_token.literal,
                got_token.literal
            )
        }
    }

    #[test]
    fn test_double_char_symbols(){
        let input = r#"
        5 == 7;

        7 != 8;

        5 < 6;
        6 <= 6;

        7 > 9;
        7 >= 7;
        "#;
        let expected : Vec<Token> = vec![
            Token{
                kind: TokenKind::Int,
                literal : "5".to_string()
            },
            Token{
                kind: TokenKind::Eq,
                literal : "==".to_string()
            },
            Token{
                kind: TokenKind::Int,
                literal : "7".to_string()
            },
            Token{
                kind: TokenKind::Semicolon,
                literal : ";".to_string()
            },

            Token{
                kind: TokenKind::Int,
                literal : "7".to_string()
            },
            Token{
                kind: TokenKind::NotEq,
                literal : "!=".to_string()
            },
            Token{
                kind: TokenKind::Int,
                literal : "8".to_string()
            },
            Token{
                kind: TokenKind::Semicolon,
                literal : ";".to_string()
            },

            Token{
                kind: TokenKind::Int,
                literal : "5".to_string()
            },
            Token{
                kind: TokenKind::Lt,
                literal : "<".to_string()
            },
            Token{
                kind: TokenKind::Int,
                literal : "6".to_string()
            },
            Token{
                kind: TokenKind::Semicolon,
                literal : ";".to_string()
            },

            Token{
                kind: TokenKind::Int,
                literal : "6".to_string()
            },
            Token{
                kind: TokenKind::LtEq,
                literal : "<=".to_string()
            },
            Token{
                kind: TokenKind::Int,
                literal : "6".to_string()
            },
            Token{
                kind: TokenKind::Semicolon,
                literal : ";".to_string()
            },

            Token{
                kind: TokenKind::Int,
                literal : "7".to_string()
            },
            Token{
                kind: TokenKind::Gt,
                literal : ">".to_string()
            },
            Token{
                kind: TokenKind::Int,
                literal : "9".to_string()
            },
            Token{
                kind: TokenKind::Semicolon,
                literal : ";".to_string()
            },

            Token{
                kind: TokenKind::Int,
                literal : "7".to_string()
            },
            Token{
                kind: TokenKind::GtEq,
                literal : ">=".to_string()
            },
            Token{
                kind: TokenKind::Int,
                literal : "7".to_string()
            },
            Token{
                kind: TokenKind::Semicolon,
                literal : ";".to_string()
            },
        ];
        let input_to_lexer = input.chars().collect();

        let mut lexer = Lexer::new(input_to_lexer);

        for (idx, expected_token) in expected.into_iter().enumerate(){
            let got_token = lexer.next_token();
            assert_eq!(
                got_token.kind, 
                expected_token.kind,
                "test-extended-double-symbols-kind {}, token type wrong. Expected {:?} Got {:?}",
                idx,
                expected_token.kind,
                got_token.kind
            );
            assert_eq!(
                got_token.literal, 
                expected_token.literal,
                "test-extended-double-symbols-literal {}, token type wrong. Expected {:?} Got {:?}",
                idx,
                expected_token.literal,
                got_token.literal
            )
        }
    }
}