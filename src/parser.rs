use std::collections::HashMap;
use std::fmt::{self, Formatter, Display};

use lexer::Lexer;
use token::{Token, TokenType};
use types::*;
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

pub struct Parser {
    lex: Lexer,
    pub cur_token: Token,
    pub peek_token: Token,
    errors: Vec<String>
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        let mut parser = Parser{
            lex: l,
            cur_token: Token{token: TokenType::EOF, literal: String::new()},
            peek_token: Token{token: TokenType::EOF, literal: String::new()},
            errors: Vec::new(),
        };
        parser.next_token();
        parser.next_token();

        parser
    }

    fn prefix_parse_fns(&mut self, tok: Token) -> Result<Expression, String> {
        match tok.token {
            TokenType::IDENT => Ok(self.parse_identifier().unwrap()),
            TokenType::BANG => Ok(self.parse_prefix_expression().unwrap()),
            TokenType::MINUS => Ok(self.parse_prefix_expression().unwrap()),
            //TokenType::INT => Ok(self.parse_integer().unwrap()),
            _ => unimplemented!(),
        }
    }

    fn infix_parse_fns(&mut self, tok: Token, exp: Expression) -> Option<Expression> {
        match tok.token {
            TokenType::LPAREN => unimplemented!(), 
            _ => self.parse_infix_expression(exp),
        }
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

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.to_owned();
        self.peek_token = self.lex.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program{
            statements: Vec::new()
        };

        while !self.current_token_is(TokenType::EOF) {
            let stmt = self.parse_statement();
            println!("Statement => {:?}", stmt);
            match stmt {
                Some(s) => {
                    println!("Token => {:?}", s);
                    program.statements.push(Box::new(s));
                },
                None => { break },
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        println!("Current Token => {:?}", self.cur_token.token);
        match self.cur_token.token {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => { unimplemented!() },
            _ => { unimplemented!() }
        }
    }

    fn parse_expression(&mut self, preced: PrecedenceType) -> Option<Expression> {
        let cur_tok = self.cur_token.clone();
        let mut left_exp = self.prefix_parse_fns(cur_tok).unwrap();
        let mut peek_pred = !self.peek_token_is(TokenType::SEMICOLON);
        while peek_pred && preced < self.peek_precedence() {
            let peek_tok = self.peek_token.clone();
            if let Some(infix) = self.infix_parse_fns(peek_tok, left_exp.clone()) {
                self.next_token();
                left_exp = infix;
                peek_pred = !self.peek_token_is(TokenType::SEMICOLON);
            }
        }
        Some(left_exp)
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let cur_tok = self.cur_token.clone();
        let cur_op = cur_tok.literal.clone();

        self.next_token();
        let right = self.parse_expression(PrecedenceType::PREFIX);

        let exp = Expression::PREFIX(PrefixExpression{
            token: cur_tok,
            operator: cur_op,
            right: Box::new(right.unwrap()),
        });
        Some(exp)
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let preced = self.cur_precedence();
        let cur_tok = self.cur_token.clone();
        let cur_op = cur_tok.literal.clone();

        self.next_token();
        let right = self.parse_expression(preced);
        
        let exp = Expression::INFIX(InfixExpression{
            token: cur_tok,
            operator: cur_op,
            left: Box::new(left),
            right: Box::new(right.unwrap()),
        });
        Some(exp)
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let tok = self.cur_token.clone();
        if !self.expect_peek(TokenType::IDENT) {
            return None
        }
        let iden = Identifier{token: tok.clone(), value: tok.clone().literal};
        if !self.expect_peek(TokenType::ASSIGN) {
            return None
        }
        self.next_token();
        let value = self.parse_expression(PrecedenceType::LOWEST);
        while !self.current_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        let stmt = Statement::LET(LetStatement{token: tok.clone(), name: iden, value: Some(Box::new(value.unwrap()))});
        println!("Statement => {:?}", stmt);
        return Some(stmt)
    }

    fn parse_identifier(&self) -> Option<Expression> {
        Some(Expression::IDENT(Identifier{token: self.cur_token.clone(), value: self.cur_token.clone().literal}))
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

    fn no_prefix_parse_fn_error(&mut self, tt: TokenType) {
        let msg = format!("no prefix parse function for {:?} found", tt);
        self.errors.push(msg.to_owned());
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