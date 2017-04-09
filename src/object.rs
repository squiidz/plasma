use ast::*;
use environment::*;

pub trait Objecter {
    fn obj_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
}

#[derive(Debug, PartialEq)]
pub enum ObjectType {
    INTEGER,
    STRING,
    BOOL,
    FUNCTION,
    BUILTIN,
    NULL,
    RETURN_VAL,
    ERROR,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    INTEGER(Integer),
    BOOL(Boolean),
    STRING(Str),
    FUNCTION(Func),
    BUILTIN(BuiltIn),
    RETURN_VAL(Return),
    NULL,
    ERROR(Error),
}

impl Object {
    pub fn expression_mapping(exp: Expression) -> Option<Object> {
        match exp {
            Expression::INTEGER(v) => Some(Object::INTEGER(Integer { value: v.value })),
            _ => None,
        }
    }
}

impl Objecter for Object {
    fn obj_type(&self) -> ObjectType {
        match *self {
            Object::INTEGER(ref int) => int.obj_type(),
            Object::BOOL(ref b) => b.obj_type(),
            Object::STRING(ref s) => s.obj_type(),
            Object::FUNCTION(ref f) => f.obj_type(),
            Object::BUILTIN(ref b) => b.obj_type(),
            Object::RETURN_VAL(ref val) => val.obj_type(),
            Object::NULL => ObjectType::NULL,
            Object::ERROR(ref e) => e.obj_type(),
        }
    }

    fn inspect(&self) -> String {
        match *self {
            Object::INTEGER(ref int) => int.inspect(),
            Object::BOOL(ref b) => b.inspect(),
            Object::STRING(ref s) => s.inspect(),
            Object::FUNCTION(ref f) => f.inspect(),
            Object::BUILTIN(ref b) => b.inspect(),
            Object::RETURN_VAL(ref val) => val.inspect(),
            Object::NULL => "null".to_owned(),
            Object::ERROR(ref e) => e.inspect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Integer {
    pub value: i64,
}

impl Objecter for Integer {
    fn obj_type(&self) -> ObjectType {
        ObjectType::INTEGER
    }
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Boolean {
    True,
    False,
}

impl Objecter for Boolean {
    fn obj_type(&self) -> ObjectType {
        ObjectType::BOOL
    }
    fn inspect(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Str {
    pub value: String,
}

impl Objecter for Str {
    fn obj_type(&self) -> ObjectType {
        ObjectType::STRING
    }
    fn inspect(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Func {
    pub parameters: Vec<Expression>,
    pub body: Statement,
    pub env: Environment,
}

impl Objecter for Func {
    fn obj_type(&self) -> ObjectType {
        ObjectType::FUNCTION
    }
    fn inspect(&self) -> String {
        let mut buff = String::new();

        buff.push_str("function");
        buff.push('(');
        for p in self.parameters.clone() {
            buff.push_str(format!("{}, ", p.to_string()).as_str());
        }
        buff.push_str(") {\n");
        buff.push_str(self.body.to_string().as_str());
        buff.push_str("\n}");

        buff
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuiltIn {
    func: fn(Vec<String>) -> Object,
}

impl Objecter for BuiltIn {
    fn obj_type(&self) -> ObjectType {
        ObjectType::BUILTIN
    }
    fn inspect(&self) -> String {
        "Builtin function".to_owned()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    pub value: Box<Object>,
}

impl Objecter for Return {
    fn obj_type(&self) -> ObjectType {
        ObjectType::RETURN_VAL
    }
    fn inspect(&self) -> String {
        self.value.inspect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    message: String,
}

impl Objecter for Error {
    fn obj_type(&self) -> ObjectType {
        ObjectType::ERROR
    }
    fn inspect(&self) -> String {
        format!("ERROR: {}", self.message)
    }
}
