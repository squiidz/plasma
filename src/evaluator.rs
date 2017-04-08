use ast::*;
use types::{self, Program};
use object::{self, Object, ObjectType, Objecter};
use environment::*;

pub fn eval<'a>(node: &'a NodeType, mut env: &mut Environment) -> Option<Object> {
    println!("NODE => {:?}", node);
    match *node {
        NodeType::Program(ref prog) => eval_program(prog, env),
        NodeType::Expression(ref exp) => {
            match *exp {
                Expression::PREFIX(ref prefix) => {
                    if let Some(right) = eval(&NodeType::Expression(*prefix.right.clone()), env) {
                        if is_error(&right) {
                            return Some(right);
                        }
                        return eval_prefix_expression(&prefix.operator, right);
                    };
                    None
                }
                _ => unimplemented!(),
            }
        }
        NodeType::Statement(ref stmt) => {
            match *stmt {
                Statement::VAR(ref var_stmt) => {
                    if let Some(val) = eval(node, env) {
                        if is_error(&val) {
                            return Some(val);
                        }
                        return env.set(var_stmt.name.value.as_str(), val);
                    }
                    None
                }
                Statement::EXPR_STMT(ref exp_stmt) => {
                    if let Some(ref expr) = exp_stmt.expression {
                        return eval(&NodeType::Expression(*expr.clone()), env);
                    };
                    None
                }
                _ => unimplemented!(),
            }
        }
        _ => panic!("INVALID EXPRESSION"),
    };
    None
}

fn eval_program(program: &Program, env: &mut Environment) -> Option<Object> {
    let mut result: Option<Object> = None;

    for stmt in &*program.statements {
        result = eval(&NodeType::Statement(*stmt.clone()), env);
        match result {
            Some(ref r) => {
                match *r {
                    Object::RETURN_VAL(ref res) => return Some(*res.value.clone()),
                    Object::ERROR(_) => return Some(r.clone()),
                    _ => continue,
                }
            },
            None => return result,
        }
    }
    result
}

fn eval_prefix_expression(op: &str, right: Object) -> Option<Object> {
    match op {
        "!" => Some(parse_bang_operator(right)),
        "-" => Some(parse_minus_operator(right)),
        _ => None,
    }
}

fn parse_bang_operator(right: Object) -> Object {
    if let Object::BOOL(b) = right {
        match b {
            object::Boolean::True => Object::BOOL(object::Boolean::False),
            object::Boolean::False => Object::BOOL(object::Boolean::True),
        }
    } else {
        match right {
            Object::NULL => Object::BOOL(object::Boolean::True),
            _ => Object::BOOL(object::Boolean::False),
        }
    }
}

fn parse_minus_operator(right: Object) -> Object {
    unimplemented!()
}

fn is_error(obj: &Object) -> bool {
    obj.obj_type() == ObjectType::ERROR
}
