use token::{Token};
use ast::*;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Box<Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            let head_stmt = self.statements[0].token_literal();
            return head_stmt;
        }
        "".to_owned()
    }

    fn to_string(&self) -> String {
        let mut out = String::new();
        for s in &self.statements {
            out.push_str(s.to_string().as_str());
        }
        out
    }

    fn node_type(&self) -> NodeType {
        NodeType::Program(self.clone())
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn to_string(&self) -> String {
        self.value.to_owned()
    }
        
    fn node_type(&self) -> NodeType {
        NodeType::Expression(Expression::IDENT(self.clone()))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VarStatement {
    pub token: Token,
    pub name:  Identifier,
    pub value: Option<Box<Expression>>,
}

impl Node for VarStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }
    fn to_string(&self) -> String {
        let mut out = String::new();
        out.push_str(&(self.token_literal() + " "));
        out.push_str(&self.name.to_string());
        out.push_str(" = ");
        if let Some(v) = self.value.as_ref() {
            out.push_str(v.to_string().as_str());
        }
        out.push_str(";");
        out.to_owned()
    }

    fn node_type(&self) -> NodeType {
        NodeType::Statement(Statement::VAR(self.clone()))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Box<Expression>>
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn to_string(&self) -> String {
        let mut out = String::new();

        out.push_str(&(self.token_literal() + " "));
        if let Some(v) = self.return_value.as_ref() {
            out.push_str(v.to_string().as_str());
        }

        out.push_str(";");
        out.to_owned()
    }
    
    fn node_type(&self) -> NodeType {
        NodeType::Statement(Statement::RETURN(self.clone()))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<Box<Expression>>
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn to_string(&self) -> String {
        match self.expression.as_ref() {
            Some(v) => { v.to_string() },
            None => { "".to_owned() },
        }
    }

    fn node_type(&self) -> NodeType {
        NodeType::Statement(Statement::EXPR_STMT(self.clone()))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn to_string(&self) -> String {
        self.token.literal.to_owned()
    }
    
    fn node_type(&self) -> NodeType {
        NodeType::Expression(Expression::INTEGER(self.clone()))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<Expression>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn to_string(&self) -> String {
        let mut out = String::new();

        out.push_str("(");
        out.push_str(self.operator.as_str());
        out.push_str(self.right.to_string().as_str());
        out.push_str(")");

        out.to_owned()
    }

    fn node_type(&self) -> NodeType {
        NodeType::Expression(Expression::PREFIX(self.clone()))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub token: Token,
    pub operator: String,
    pub left:  Box<Expression>,
    pub right: Box<Expression>
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn to_string(&self) -> String {
        let mut out = String::new();

        out.push_str("(");
        out.push_str(self.left.to_string().as_str());
        out.push_str(&format!(" {} " , self.operator));
        out.push_str(self.right.to_string().as_str());
        out.push_str(")");

        out.to_owned()
    }
    
    fn node_type(&self) -> NodeType {
        NodeType::Expression(Expression::INFIX(self.clone()))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Boolean {
    pub token: Token,
    pub value: bool
}

impl Node for Boolean {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn to_string(&self) -> String {
        self.token.literal.to_owned()
    }
    
    fn node_type(&self) -> NodeType {
        NodeType::Expression(Expression::BOOL(self.clone()))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Statement>,
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn to_string(&self) -> String {
        let mut out = String::new();

        for s in &self.statements {
            out.push_str(s.to_string().as_str())
        }

        out.to_owned()
    }

    fn node_type(&self) -> NodeType {
        NodeType::Statement(Statement::BLOCK_STMT(self.clone()))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IfExpression {
    pub token: Token,
    pub condition:   Box<Expression>,
    pub consequence: Box<Statement>,
    pub alternative: Option<Statement>
}

impl Node for IfExpression {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn to_string(&self) -> String {
        let mut out = String::new();

        out.push_str("if");
        out.push_str(self.condition.to_string().as_str());
        out.push_str(" ");
        out.push_str(self.consequence.to_string().as_str());

        if let Some(v) = self.alternative.as_ref() {
            out.push_str("else ");
            out.push_str(v.to_string().as_str());
        }

        out.to_owned()
    }

    fn node_type(&self) -> NodeType {
        NodeType::Expression(Expression::IF(self.clone()))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Expression>,
    pub body: Statement
}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn to_string(&self) -> String {
        let mut out = String::new();
        let mut params = Vec::new();

        for p in &self.parameters {
            params.push(p.to_string());
        }
        out.push_str(self.token_literal().as_str());
        out.push_str("(");
        out.push_str(params.join(", ").as_str());
        out.push_str(") ");
        out.push_str(self.body.to_string().as_str());

        out.to_owned()
    }

    fn node_type(&self) -> NodeType {
        NodeType::Expression(Expression::FUNC(self.clone()))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CallExpression {
    pub token: Token,
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>
}

impl Node for CallExpression {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn to_string(&self) -> String {
        let mut out = String::new();
        let mut params = Vec::new();

        for p in &self.arguments {
            params.push(p.to_string());
        }

        out.push_str(self.function.to_string().as_str());
        out.push_str("(");
        out.push_str(params.join(", ").as_str());
        out.push_str(")");

        out.to_owned()
    }

    fn node_type(&self) -> NodeType {
        NodeType::Expression(Expression::CALL(self.clone()))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub token: Token,
    pub value: String
}

impl Node for StringLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn to_string(&self) -> String {
        self.token.literal.to_owned()
    }

    fn node_type(&self) -> NodeType {
        NodeType::Expression(Expression::STRING(self.clone()))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ArrayLiteral {
    pub token: Token,
    pub elements: Vec<Expression>
}

impl Node for ArrayLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn to_string(&self) -> String {
        let mut out = String::new();
        let mut elements = Vec::new();

        for el in &self.elements {
            elements.push(el.to_string());
        }

        out.push_str("[");
        out.push_str(elements.join(", ").as_str());
        out.push_str("]");

        out.to_owned()
    }

    fn node_type(&self) -> NodeType {
        NodeType::Expression(Expression::ARRAY(self.clone()))
    }
}