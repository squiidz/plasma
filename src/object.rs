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

#[derive(Debug, Clone)]
pub enum Object {
    INTEGER(Integer),
    BOOL(Boolean),
    STRING(Str),
    FUNCTION(Func),
    BUILTIN(BuiltIn),
    NULL(Null),
    ERROR(Error),
}

impl Objecter for Object {
    fn obj_type(&self) -> ObjectType {
        match *self {
            Object::INTEGER(ref int) => int.obj_type(),
            Object::BOOL(ref b) => b.obj_type(),
            Object::STRING(ref s) => s.obj_type(),
            Object::FUNCTION(ref f) => f.obj_type(),
            Object::BUILTIN(ref b) => b.obj_type(),
            Object::NULL(ref n) => n.obj_type(),
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
            Object::NULL(ref n) => n.inspect(),
            Object::ERROR(ref e) => e.inspect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Integer {
    pub value: i64
}

impl Objecter for Integer {
    fn obj_type(&self) -> ObjectType {
        ObjectType::INTEGER
    }
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct Boolean {
    value: bool
}

impl Objecter for Boolean {
    fn obj_type(&self) -> ObjectType {
        ObjectType::BOOL
    }
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct Str {
    value: String
}

impl Objecter for Str {
    fn obj_type(&self) -> ObjectType {
        ObjectType::STRING
    }
    fn inspect(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Func {
    parameters: Vec<Expression>,
    body: Statement,
    env: Environment,
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Null();

impl Objecter for Null {
    fn obj_type(&self) -> ObjectType {
        ObjectType::NULL
    }
    fn inspect(&self) -> String {
        "null".to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    message: String
}

impl Objecter for Error {
    fn obj_type(&self) -> ObjectType {
        ObjectType::ERROR
    }
    fn inspect(&self) -> String {
        format!("ERROR: {}", self.message)
    }
}