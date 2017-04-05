use token::Token;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Expression {
    IDENT {token: Token, value: String},
    BOOL {token: Token, value: bool},
    INTEGER {token: Token, value: i64},
    STRING {token: Token, value: String},
    ARRAY {token: Token, elems: Vec<Box<Expression>>},
    PREFIX {token: Token, operator: char, right: Option<Box<Expression>>},
    INFIX {token: Token, operator: char, left: Option<Box<Expression>>, right: Option<Box<Expression>>},
    IF {token: Token, condition: Box<Expression>, consequence: Statement, alter: Option<Statement>},
    FUNC {token: Token, params: Vec<Box<Expression>>, body: Statement},
    CALL {token: Token, func: Box<Expression>, args: Vec<Box<Expression>>},
}

impl Expression {
    fn to_string(self) -> String {
        match self {
            Expression::IDENT{token, value} => {
                format!("{:?} {}", token.token, value)
            },
            Expression::BOOL{token, value} => {
                format!("{:?} {}", token.token, value)
            },
            Expression::INTEGER{token, value} => {
                format!("{:?} {}", token.token, value)
            },
            Expression::STRING{token, value} => {
                format!("{:?} {}", token.token, value)
            },
            _ => unimplemented!(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Statement {
    LET {token: Token, name: String, value: Option<Box<Expression>>},
    EXPR_STMT {token: Token, expr: Option<Box<Expression>>},
    BLOCK_STMT {token: Token, statements: Vec<Box<Statement>>},
    RETURN {token: Token, value: Option<Box<Expression>>},
}

impl Statement {
    pub fn to_string(self) -> String {
        match self {
            Statement::LET{token, name, value} => {
                format!("{:?} {} {:?}", token.token, name, value)
            },
            Statement::RETURN{token, value} => {
                format!("{:?} {:?}", token.token, value)
            } 
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum NodeType {
    Expression(Expression),
    Statement(Statement),
    Program(Program),
}

#[derive(Debug)]
pub struct Node {
    node_type: NodeType,
    value: String,
}

#[allow(dead_code)]
impl Node {
    pub fn new(nt: NodeType, value: &str) -> Node {
        Node{node_type: nt, value: value.to_owned()}
    }
    
    pub fn new_statement(stmt: Statement, value: &str) -> Node {
        Node{node_type: NodeType::Statement(stmt), value: value.to_owned()} 
    }
    
    pub fn new_expression(expr: Expression, value: &str) -> Node {
        Node{node_type: NodeType::Expression(expr), value: value.to_owned()} 
    }
    
    pub fn to_string(self) -> String {
        match self.node_type {
            NodeType::Statement(stmt) => { stmt.to_string() },
            NodeType::Expression(expr) => { expr.to_string() },
            _ => { unimplemented!() }
        }
    }
}