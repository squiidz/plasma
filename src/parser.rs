use std::collections::HashMap;
use std::fmt::{self, Formatter, Display};

use lexer::Lexer;
use token::{Token, TokenType};
use ast::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
enum PrecedenceType {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL
}


lazy_static! {
    static ref PRECEDENCES: HashMap<TokenType, PrecedenceType> = {
        let mut hm = HashMap::new();
        hm.insert(TokenType::EQ, PrecedenceType::EQUALS);
        hm.insert(TokenType::NOT_EQ, PrecedenceType::EQUALS);
        hm.insert(TokenType::LT, PrecedenceType::LESSGREATER);
        hm.insert(TokenType::GT, PrecedenceType::LESSGREATER);
        hm.insert(TokenType::PLUS, PrecedenceType::SUM);
        hm.insert(TokenType::MINUS, PrecedenceType::SUM);
        hm.insert(TokenType::SLASH, PrecedenceType::PRODUCT);
        hm.insert(TokenType::ASTERISK, PrecedenceType::PRODUCT);
        hm.insert(TokenType::LPAREN, PrecedenceType::CALL);
        hm
    };
}

type prefixParseFn = fn() -> Expression;
type infixParseFn = fn(Expression) -> Expression;

pub struct Parser {
    lex: Lexer,
    pub cur_token: Token,
    peek_token: Token,
    prefix_parse_fns: HashMap<TokenType, prefixParseFn>,
    infix_parse_fns: HashMap<TokenType, infixParseFn>,
    errors: Vec<String>
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        let mut parser = Parser{
            lex: l,
            cur_token: Token{token: TokenType::EOF, literal: String::new()},
            peek_token: Token{token: TokenType::EOF, literal: String::new()},
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
            errors: Vec::new(),
        };
        parser.next_token();
        parser.next_token();
        //parser.init_prefix();
        //parser.init_infix();
        parser
    }

    fn init_prefix(&mut self) {
        unimplemented!()
        //self.register_prefix(TokenType::IDENT, self.parseIdentifier);
    }

    fn init_infix(&mut self) {
        unimplemented!()
    }

    fn cur_precedence(&self) -> PrecedenceType {
        match PRECEDENCES.get(&self.cur_token.token) {
            Some(p) => { p.clone() },
            None => { PrecedenceType::LOWEST }
        }
    }

    fn peek_precedence(&self) -> PrecedenceType {
        match PRECEDENCES.get(&self.peek_token.token) {
            Some(p) => { p.clone() },
            None => { PrecedenceType::LOWEST }
        }
    }

    fn register_prefix(&mut self, tt: TokenType, func: prefixParseFn) {
        self.prefix_parse_fns.insert(tt, func);
    }

    fn register_infix(&mut self, tt: TokenType, func: infixParseFn) {
        self.infix_parse_fns.insert(tt, func);
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.to_owned();
        self.peek_token = self.lex.next_token();
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program{
            statements: Vec::new()
        };

        while !self.current_token_is(TokenType::EOF) {
            let stmt = self.parse_statement();
            match stmt {
                Some(s) => {
                    program.statements.push(s)
                },
                None => { break },
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&self) -> Option<Statement> {
        match self.cur_token.token {
            TokenType::LET => { unimplemented!() },
            TokenType::RETURN => { unimplemented!() },
            _ => { unimplemented!() }
        }
    }

    fn parse_expression(&mut self, preced: PrecedenceType) -> Option<Expression> {
        let prefix = match self.prefix_parse_fns.get(&self.cur_token.token).cloned() {
            Some(f) => { f },
            None => {
                self.no_prefix_parse_fn_error(self.cur_token.token);
                return None
            }
        };

        let mut left_exp = prefix();
        let mut peek_tok = !self.peek_token_is(TokenType::SEMICOLON);
        while peek_tok && preced < self.peek_precedence() {
            let infix = match self.infix_parse_fns.get(&self.peek_token.token).cloned() {
                Some(i) => { i },
                None => { return Some(left_exp) }
            };
            
            self.next_token();
            left_exp = infix(left_exp);
            peek_tok = !self.peek_token_is(TokenType::SEMICOLON);
        }
        Some(left_exp)
    }

    fn current_token_is(&self, tt: TokenType) -> bool {
        self.cur_token.token == tt
    }

    fn peek_token_is(&self, tt: TokenType) -> bool {
        self.peek_token.token == tt
    }

    fn expect_peek(&mut self, tt: TokenType) -> bool {
        if self.peek_token_is(tt) {
            self.next_token();
            return true
        }
        self.peek_error(tt);
        false
    }

    fn no_prefix_parse_fn_error(&self, tt: TokenType) {
        let msg = format!("no prefix parse function for {:?} found", tt);
        //self.errors.push(msg.to_owned());
    }

    fn peek_error(&mut self, tt: TokenType) {
        let msg = format!("expect next token to be {:?}, got {:?} instead", tt, self.peek_token.token);
        self.errors.push(msg.to_owned());
    }
}

impl Display for Parser {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Current Token: {}\nPeek Token: {}", self.cur_token, self.peek_token)
    }
}