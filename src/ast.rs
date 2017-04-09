use types::*;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
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

impl Node for Expression {
    fn to_string(&self) -> String {
        match *self {
            Expression::IDENT(ref ident) => ident.to_string(),
            Expression::BOOL(ref b) => b.to_string(),
            Expression::INTEGER(ref int) => int.to_string(),
            Expression::STRING(ref string) => string.to_string(),
            Expression::ARRAY(ref arr) => arr.to_string(),
            Expression::PREFIX(ref pre) => pre.to_string(),
            Expression::INFIX(ref inf) => inf.to_string(),
            Expression::IF(ref if_exp) => if_exp.to_string(),
            Expression::FUNC(ref func) => func.to_string(),
            Expression::CALL(ref call) => call.to_string(),
        }
    }

    fn token_literal(&self) -> String {
        match *self {
            Expression::IDENT(ref ident) => ident.token_literal(),
            Expression::BOOL(ref b) => b.token_literal(),
            Expression::INTEGER(ref int) => int.token_literal(),
            Expression::STRING(ref string) => string.token_literal(),
            Expression::ARRAY(ref arr) => arr.token_literal(),
            Expression::PREFIX(ref pre) => pre.token_literal(),
            Expression::INFIX(ref inf) => inf.token_literal(),
            Expression::IF(ref if_exp) => if_exp.token_literal(),
            Expression::FUNC(ref func) => func.token_literal(),
            Expression::CALL(ref call) => call.token_literal(),
        }
    }

    fn node_type(&self) -> NodeType {
        match *self {
            Expression::IDENT(ref ident) => ident.node_type(),
            Expression::BOOL(ref b) => b.node_type(),
            Expression::INTEGER(ref int) => int.node_type(),
            Expression::STRING(ref string) => string.node_type(),
            Expression::ARRAY(ref arr) => arr.node_type(),
            Expression::PREFIX(ref pre) => pre.node_type(),
            Expression::INFIX(ref inf) => inf.node_type(),
            Expression::IF(ref if_exp) => if_exp.node_type(),
            Expression::FUNC(ref func) => func.node_type(),
            Expression::CALL(ref call) => call.node_type(),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VAR(VarStatement),
    EXPR_STMT(ExpressionStatement),
    BLOCK_STMT(BlockStatement),
    RETURN(ReturnStatement),
}

impl Node for Statement {
    fn to_string(&self) -> String {
        match *self {
            Statement::VAR(ref let_stmt) => let_stmt.to_string(),
            Statement::EXPR_STMT(ref expr_stmt) => expr_stmt.to_string(),
            Statement::BLOCK_STMT(ref blk_stmt) => blk_stmt.to_string(),
            Statement::RETURN(ref rtn_stmt) => rtn_stmt.to_string(),
        }
    }

    fn token_literal(&self) -> String {
        match *self {
            Statement::VAR(ref let_stmt) => let_stmt.token_literal(),
            Statement::EXPR_STMT(ref expr_stmt) => expr_stmt.token_literal(),
            Statement::BLOCK_STMT(ref blk_stmt) => blk_stmt.token_literal(),
            Statement::RETURN(ref rtn_stmt) => rtn_stmt.token_literal(),
        }
    }

    fn node_type(&self) -> NodeType {
        match *self {
            Statement::VAR(ref let_stmt) => let_stmt.node_type(),
            Statement::EXPR_STMT(ref expr_stmt) => expr_stmt.node_type(),
            Statement::BLOCK_STMT(ref blk_stmt) => blk_stmt.node_type(),
            Statement::RETURN(ref rtn_stmt) => rtn_stmt.node_type(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Expression(Expression),
    Statement(Statement),
    Program(Program),
}

pub trait Node {
    fn to_string(&self) -> String;
    fn token_literal(&self) -> String;
    fn node_type(&self) -> NodeType;
}

// #[allow(dead_code)]
// impl Node {
//     pub fn new(nt: NodeType, value: &str) -> Node {
//         Node{node_type: nt, value: value.to_owned()}
//     }

//     pub fn new_statement(stmt: Statement, value: &str) -> Node {
//         Node{node_type: NodeType::Statement(stmt), value: value.to_owned()}
//     }

//     pub fn new_expression(expr: Expression, value: &str) -> Node {
//         Node{node_type: NodeType::Expression(expr), value: value.to_owned()}
//     }

//     pub fn to_string(&self) -> String {
//         match self.node_type {
//             NodeType::Statement(ref stmt) => { stmt.to_string() },
//             NodeType::Expression(ref expr) => { expr.to_string() },
//             _ => { unimplemented!() }
//         }
//     }
// }
