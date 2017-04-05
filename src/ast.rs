use token::Token;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Expression {
    IDENT {token: Token, value: String},
    FUNC,
    VAR,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Statement {
    LET {token: Token, name: String, value: Option<Expression>},
    RETURN {token: Token, value: Option<Expression>},
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
            NodeType::Expression(expr) => { unimplemented!() },
            _ => { unimplemented!() }
        }
    }
}