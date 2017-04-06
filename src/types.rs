use token::{Token};
use ast::*;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Box<Statement>>,
}

impl Program {
    pub fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            let head_stmt = self.statements[0].token_literal();
            return head_stmt;
        }
        return "".to_owned();
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();
        for s in self.statements.iter() {
            out.push_str(s.to_string().as_str());
        }
        return out;
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Identifier {
    pub fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    pub fn to_string(&self) -> String {
        self.value.to_owned()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name:  Identifier,
    pub value: Option<Box<Expression>>,
}

impl LetStatement {
    pub fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }
    pub fn to_string(&self) -> String {
        let mut out = String::new();
        out.push_str(&(self.token_literal() + " "));
        out.push_str(&self.name.to_string());
        out.push_str(" = ");
        match self.value.as_ref() {
            Some(v) => out.push_str(v.to_string().as_str()),
            None => {}
        }
        out.push_str(";");
        out.to_owned()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ReturnStatement {
    token: Token,
    return_value: Option<Box<Expression>>
}

impl ReturnStatement {
    pub fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();

        out.push_str(&(self.token_literal() + " "));
        match self.return_value.as_ref() {
            Some(v) => { out.push_str(v.to_string().as_str()) },
            None => {},
        }

        out.push_str(";");
        out.to_owned()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    token: Token,
    expression: Option<Box<Expression>>
}

impl ExpressionStatement {
    pub fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    pub fn to_string(&self) -> String {
        match self.expression.as_ref() {
            Some(v) => { v.to_string() },
            None => { "".to_owned() },
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    token: Token,
    value: i64
}

impl IntegerLiteral {
    pub fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    pub fn to_string(&self) -> String {
        self.token.literal.to_owned()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<Expression>,
}

impl PrefixExpression {
    pub fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();

        out.push_str("(");
        out.push_str(self.operator.as_str());
        out.push_str(self.right.to_string().as_str());
        out.push_str(")");

        out.to_owned()
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

impl InfixExpression {
    pub fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();

        out.push_str("(");
        out.push_str(self.left.to_string().as_str());
        out.push_str(&format!(" {} " , self.operator));
        out.push_str(self.right.to_string().as_str());
        out.push_str(")");

        out.to_owned()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Boolean {
    token: Token,
    value: bool
}

impl Boolean {
    pub fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    pub fn to_string(&self) -> String {
        self.token.literal.to_owned()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BlockStatement {
    token: Token,
    statements: Vec<Box<Statement>>
}

impl BlockStatement {
    pub fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();

        for s in self.statements.iter() {
            out.push_str(s.to_string().as_str())
        }

        out.to_owned()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IfExpression {
    token: Token,
    condition:   Box<Expression>,
    consequence: Box<BlockStatement>,
    alternative: Option<Box<BlockStatement>>
}

impl IfExpression {
    pub fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();

        out.push_str("if");
        out.push_str(self.condition.to_string().as_str());
        out.push_str(" ");
        out.push_str(self.consequence.to_string().as_str());

        match self.alternative.as_ref() {
            Some(v) => {
                out.push_str("else ");
                out.push_str(v.to_string().as_str());
            },
            None => {},
        }

        out.to_owned()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FunctionLiteral {
    token: Token,
    parameters: Vec<Box<Identifier>>,
    body: Box<BlockStatement>
}

impl FunctionLiteral {
    pub fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();
        let mut params = Vec::new();

        for p in self.parameters.iter() {
            params.push(p.to_string());
        }
        out.push_str(self.token_literal().as_str());
        out.push_str("(");
        out.push_str(params.join(", ").as_str());
        out.push_str(") ");
        out.push_str(self.body.to_string().as_str());

        out.to_owned()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CallExpression {
    token: Token,
    function: Box<Expression>,
    arguments: Vec<Box<Expression>>
}

impl CallExpression {
    pub fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();
        let mut params = Vec::new();

        for p in self.arguments.iter() {
            params.push(p.to_string());
        }

        out.push_str(self.function.to_string().as_str());
        out.push_str("(");
        out.push_str(params.join(", ").as_str());
        out.push_str(")");

        out.to_owned()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StringLiteral {
    token: Token,
    value: String
}

impl StringLiteral {
    pub fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    pub fn to_string(&self) -> String {
        self.token.literal.to_owned()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ArrayLiteral {
    token: Token,
    elements: Vec<Box<Expression>>
}

impl ArrayLiteral {
    pub fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();
        let mut elements = Vec::new();

        for el in self.elements.iter() {
            elements.push(el.to_string());
        }

        out.push_str("[");
        out.push_str(elements.join(", ").as_str());
        out.push_str("]");

        out.to_owned()
    }
}