use token::Token;
use types::*;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expression {
    IDENT(Identifier),
    BOOL(Boolean),
    INTEGER(IntegerLiteral),
    STRING(StringLiteral),
    ARRAY(ArrayLiteral),
    PREFIX(PrefixExpression),
    INFIX(InfixExpression),
    IF(IfExpression),
    FUNC(FunctionLiteral),
    CALL(CallExpression),
}

impl Expression {
    pub fn to_string(&self) -> String {
        self.to_string()
    }

    pub fn token_literal(&self) -> String {
        self.token_literal()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Statement {
    LET(LetStatement),
    EXPR_STMT(ExpressionStatement),
    BLOCK_STMT(BlockStatement),
    RETURN(ReturnStatement),
}

impl Statement {
    pub fn to_string(&self) -> String {
        self.to_string()
    }

    pub fn token_literal(&self) -> String {
        self.token_literal()
    }
}

#[derive(Debug, Clone)]
pub enum NodeType {
    Expression(Expression),
    Statement(Statement),
    Program(Program),
}

#[derive(Debug, Clone)]
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
    
    pub fn to_string(&self) -> String {
        match self.node_type {
            NodeType::Statement(ref stmt) => { stmt.to_string() },
            NodeType::Expression(ref expr) => { expr.to_string() },
            _ => { unimplemented!() }
        }
    }
}