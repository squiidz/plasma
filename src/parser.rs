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
    pub errors: Vec<String>
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

    fn prefix_parse_fns(&mut self, tok: Token) -> Option<Expression> {
        match tok.token {
            TokenType::IDENT => self.parse_identifier(),
            TokenType::BANG => self.parse_prefix_expression(),
            TokenType::MINUS => self.parse_prefix_expression(),
            TokenType::INT => self.parse_integer(),
            TokenType::TRUE => self.parse_boolean(),
            TokenType::FALSE => self.parse_boolean(),
            TokenType::LPAREN => self.parse_group_expression(),
            TokenType::IF => self.parse_if_expression(),
            TokenType::FUNCTION => self.parse_function(),
            TokenType::STRING => self.parse_string(),
            TokenType::LBRACKET => self.parse_array(),
            _ => None,
        }
    }

    fn infix_parse_fns(&mut self, tok: Token, exp: Expression) -> Option<Expression> {
        match tok.token {
            TokenType::LPAREN => self.parse_call_expression(exp),
            TokenType::PLUS => self.parse_infix_expression(exp),
            TokenType::MINUS => self.parse_infix_expression(exp),
            TokenType::SLASH => self.parse_infix_expression(exp),
            TokenType::ASTERISK => self.parse_infix_expression(exp),
            TokenType::EQ => self.parse_infix_expression(exp),
            TokenType::NOT_EQ => self.parse_infix_expression(exp),
            TokenType::LT => self.parse_infix_expression(exp),
            TokenType::GT => self.parse_infix_expression(exp),
            _ => None,
        }
    }

    fn cur_precedence(&self) -> PrecedenceType {
        match PRECEDENCES.get(&self.cur_token.token) {
            Some(p) => { *p },
            None => { PrecedenceType::LOWEST }
        }
    }

    fn peek_precedence(&self) -> PrecedenceType {
        match PRECEDENCES.get(&self.peek_token.token) {
            Some(p) => { *p },
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
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(Box::new(stmt));
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.token {
            TokenType::VAR => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let cur_tok = self.cur_token.clone();
        if let Some(exp) = self.parse_expression(PrecedenceType::LOWEST) {
            if self.peek_token_is(TokenType::SEMICOLON) {
                self.next_token();
            }

            let exp_stmt =  Statement::EXPR_STMT(ExpressionStatement{
                token: cur_tok,
                expression: Some(Box::new(exp)),
            });
            return Some(exp_stmt)
        }
        None
    }

    fn parse_expression(&mut self, preced: PrecedenceType) -> Option<Expression> {
        let cur_tok = self.cur_token.clone();
        let mut left_exp = match self.prefix_parse_fns(cur_tok) {
            Some(exp) => exp,
            None => { self.no_prefix_parse_fn_error(); return None },
        };

        while !self.peek_token_is(TokenType::SEMICOLON) && preced < self.peek_precedence() {
            let peek_tok = self.peek_token.clone();
            self.next_token();
            if let Some(infix) = self.infix_parse_fns(peek_tok, left_exp.clone()) {
                left_exp = infix;
            } else {
                return Some(left_exp)
            }
        }
        Some(left_exp)
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let cur_tok = self.cur_token.clone();
        let cur_op = cur_tok.literal.clone();

        self.next_token();
        if let Some(right) = self.parse_expression(PrecedenceType::PREFIX) {
            let exp = Expression::PREFIX(PrefixExpression{
                token: cur_tok,
                operator: cur_op,
                right: Box::new(right),
            });
            return Some(exp)
        }
        None
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let preced = self.cur_precedence();
        let cur_tok = self.cur_token.clone();
        let cur_op = cur_tok.literal.clone();

        self.next_token();
        if let Some(right) = self.parse_expression(preced) {        
            let exp = Expression::INFIX(InfixExpression{
                token: cur_tok,
                operator: cur_op,
                left: Box::new(left),
                right: Box::new(right),
            });
            return Some(exp)
        }
        None
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let cur_tok = self.cur_token.clone();
        if !self.expect_peek(TokenType::IDENT) {
            return None
        }
        let iden = Identifier{token: self.cur_token.clone(), value: self.cur_token.clone().literal};
        if !self.expect_peek(TokenType::ASSIGN) {
            return None
        }
        self.next_token();
        if let Some(value) = self.parse_expression(PrecedenceType::LOWEST) {
            while !self.current_token_is(TokenType::SEMICOLON) {
                self.next_token();
            }
            let stmt = Statement::VAR(VarStatement{token: cur_tok.clone(), name: iden, value: Some(Box::new(value))});
            return Some(stmt)
        }
        None
    }

    fn parse_integer(&self) -> Option<Expression> {
        let cur_tok = self.cur_token.clone();
        if let Ok(integer) = self.cur_token.literal.parse::<i64>() {
            let exp_int = Expression::INTEGER(IntegerLiteral{
                token: cur_tok,
                value: integer,
            });
            return Some(exp_int)
        };
        None
    }

    fn parse_boolean(&self) -> Option<Expression> {
        Some(Expression::BOOL(Boolean{token: self.cur_token.clone(), value: self.current_token_is(TokenType::TRUE)}))
    }

    fn parse_string(&self) -> Option<Expression> {
        Some(Expression::STRING(StringLiteral{token: self.cur_token.clone(), value: self.cur_token.clone().literal}))
    }

    fn parse_array(&mut self) -> Option<Expression> {
        let cur_tok = self.cur_token.clone();
        if let Some(elems) = self.parse_expression_list(TokenType::RBRACKET) {
            let arr_exp = Expression::ARRAY(ArrayLiteral{
                token: cur_tok,
                elements: elems,
            });
            return Some(arr_exp)
        }
        None
    }

    fn parse_expression_list(&mut self, tt: TokenType) -> Option<Vec<Expression>> {
        let mut list: Vec<Expression> = Vec::new();
        if self.peek_token_is(tt) {
            self.next_token();
            return Some(list)
        }

        self.next_token();
        if let Some(exp) = self.parse_expression(PrecedenceType::LOWEST) {
            list.push(exp);
        };

        while self.peek_token_is(TokenType::COMMA) {
            self.next_token();
            self.next_token();
            if let Some(exp) = self.parse_expression(PrecedenceType::LOWEST) {
                list.push(exp);
            };
        }
        if !self.expect_peek(tt) {
            return None
        }
        Some(list)
    }

    fn parse_identifier(&self) -> Option<Expression> {
        Some(Expression::IDENT(Identifier{token: self.cur_token.clone(), value: self.cur_token.clone().literal}))
    }

    fn parse_function(&mut self) -> Option<Expression> {
        let cur_tok = self.cur_token.clone();
        if !self.expect_peek(TokenType::LPAREN) {
            return None
        }

        let params = self.parse_function_parameters().unwrap();
        if !self.expect_peek(TokenType::LBRACE) {
            return None
        }
        if let Some(body) = self.parse_block_statement() {
            let func_exp = Expression::FUNC(FunctionLiteral{
                token: cur_tok,
                parameters: params,
                body: body,
            });
            return Some(func_exp)
        }
        None
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<Expression>> {
        let mut idents: Vec<Expression> = Vec::new();
        if self.peek_token_is(TokenType::RPAREN) {
            self.next_token();
            return Some(idents)
        }
        self.next_token();
        let ident = Expression::IDENT(Identifier{
            token: self.cur_token.clone(),
            value: self.cur_token.clone().literal
        });
        idents.push(ident);

        while self.peek_token_is(TokenType::COMMA) {
            self.next_token();
            self.next_token();
            let ident = Expression::IDENT(Identifier{
                token: self.cur_token.clone(),
                value: self.cur_token.clone().literal
            });
            idents.push(ident);
        }

        if !self.expect_peek(TokenType::RPAREN) {
            return None
        }
        Some(idents)
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let cur_tok = self.cur_token.clone();
        self.next_token();

        if let Some(ret_val) = self.parse_expression(PrecedenceType::LOWEST) {
            while !self.current_token_is(TokenType::SEMICOLON) {
                self.next_token();
            }
            let rtn_stmt = Statement::RETURN(ReturnStatement{
                token: cur_tok,
                return_value: Some(Box::new(ret_val)),
            });
            return Some(rtn_stmt)
        };
        None
    }

    fn parse_call_expression(&mut self, func: Expression) -> Option<Expression> {
        let cur_tok = self.cur_token.clone();
        if let Some(arguments) = self.parse_expression_list(TokenType::RPAREN) {
            let call_exp = Expression::CALL(CallExpression{
                token: cur_tok,
                function: Box::new(func),
                arguments: arguments
            });
            return Some(call_exp)
        }
        None
    }

    fn parse_call_arguments(&mut self) -> Option<Vec<Expression>> {
        let mut args: Vec<Expression> = Vec::new();
        if self.peek_token_is(TokenType::RPAREN) {
            self.next_token();
            return Some(args)
        }
        self.next_token();

        if let Some(arg) = self.parse_expression(PrecedenceType::LOWEST) {
            args.push(arg);
        }

        while self.peek_token_is(TokenType::COMMA) {
            self.next_token();
            self.next_token();
            if let Some(arg) = self.parse_expression(PrecedenceType::LOWEST) {
                args.push(arg);
            }
        }

        if !self.expect_peek(TokenType::RPAREN) {
            return None
        }
        Some(args)
    }

    fn parse_group_expression(&mut self) -> Option<Expression> {
        self.next_token();
        let exp = self.parse_expression(PrecedenceType::LOWEST);

        if !self.expect_peek(TokenType::RPAREN) {
            return None
        }
        exp
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        let cur_tok = self.cur_token.clone();
        if !self.expect_peek(TokenType::LPAREN) {
            return None
        }
        self.next_token();
        
        let exp_cond = self.parse_expression(PrecedenceType::LOWEST);
        if self.peek_token_is(TokenType::RPAREN) || self.peek_token_is(TokenType::LBRACE) {
            return None
        }

        let exp_cons = self.parse_block_statement();
        let peek_tok = self.peek_token.clone();
        if peek_tok.token == TokenType::ELSE {
            self.next_token();
            if !self.expect_peek(TokenType::LBRACE) {
                return None
            }
            let exp_alt = self.parse_block_statement();
            let if_exp = Expression::IF(IfExpression{
                token: cur_tok,
                condition: Box::new(exp_cond.unwrap()),
                consequence: Box::new(exp_cons.unwrap()),
                alternative: exp_alt,
            });
            return Some(if_exp)
        }
        None
    }

    fn parse_block_statement(&mut self) -> Option<Statement> {
        let cur_tok = self.cur_token.clone();
        let mut statements: Vec<Statement> = Vec::new();
        self.next_token();

        while !self.current_token_is(TokenType::RBRACE) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }
        let block = Statement::BLOCK_STMT(BlockStatement{
            token: cur_tok,
            statements: statements,
        });
        Some(block)
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

    #[allow(dead_code)]
    fn no_prefix_parse_fn_error(&mut self) {
        let msg = format!("no prefix parse function for {:?} found", self.cur_token.token);
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