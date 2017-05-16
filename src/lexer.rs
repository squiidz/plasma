use token::{Token, TokenType};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Lexer {
    pub input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut lex = Lexer {
            input: input.to_owned(),
            position: 0,
            read_position: 0,
            ch: '0',
        };
        lex.read_char();
        lex
    }

    pub fn next_token(&mut self) -> Token {
        let tok: Token;
        self.skip_whitespace();

        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    tok = Token {
                        token: TokenType::EQ,
                        literal: ch.to_string() + &self.ch.to_string(),
                    };
                } else {
                    tok = Token::new(TokenType::ASSIGN, self.ch)
                }
            }
            '+' => tok = Token::new(TokenType::PLUS, '+'),
            '-' => tok = Token::new(TokenType::MINUS, '-'),
            '^' => tok = Token::new(TokenType::POW, '^'),
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    tok = Token {
                        token: TokenType::NOT_EQ,
                        literal: ch.to_string() + &self.ch.to_string(),
                    };
                } else {
                    tok = Token::new(TokenType::BANG, self.ch)
                }
            }
            '"' => {
                tok = Token {
                    token: TokenType::STRING,
                    literal: self.read_string(),
                }
            }
            '/' => tok = Token::new(TokenType::SLASH, self.ch),
            '*' => tok = Token::new(TokenType::ASTERISK, self.ch),
            '<' => tok = Token::new(TokenType::LT, self.ch),
            '>' => tok = Token::new(TokenType::GT, self.ch),
            ';' => tok = Token::new(TokenType::SEMICOLON, self.ch),
            '(' => tok = Token::new(TokenType::LPAREN, self.ch),
            ')' => tok = Token::new(TokenType::RPAREN, self.ch),
            '[' => tok = Token::new(TokenType::LBRACKET, self.ch),
            ']' => tok = Token::new(TokenType::RBRACKET, self.ch),
            ',' => tok = Token::new(TokenType::COMMA, self.ch),
            '{' => tok = Token::new(TokenType::LBRACE, self.ch),
            '}' => tok = Token::new(TokenType::RBRACE, self.ch),
            '0' => {
                tok = Token {
                    token: TokenType::EOF,
                    literal: "".to_owned(),
                }
            }
            _ => {
                if self.ch.is_alphabetic() {
                    let tok_lit = self.read_identifier();
                    let tok_type = Token::lookup_ident(tok_lit.as_str());
                    tok = Token {
                        token: tok_type,
                        literal: tok_lit,
                    };
                    return tok;
                } else if self.ch.is_numeric() {
                    let tok_type = TokenType::INT;
                    let tok_lit = self.read_number();
                    tok = Token {
                        token: tok_type,
                        literal: tok_lit,
                    };
                    return tok;
                }
                tok = Token::new(TokenType::ILLEGAL, '0')
            }
        }
        self.read_char();
        tok
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_alphabetic() {
            self.read_char()
        }
        self.input[position..self.position].to_owned()
    }

    fn read_string(&mut self) -> String {
        let position = self.position + 1;
        loop {
            self.read_char();
            if self.ch == '"' {
                break;
            }
        }
        self.input[position..self.position].to_owned()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_numeric() {
            self.read_char();
        }
        self.input[position..self.position].to_owned()
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return '0';
        }
        self.input.chars().nth(self.read_position).unwrap()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }
}

impl Display for Lexer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.input)
    }
}
