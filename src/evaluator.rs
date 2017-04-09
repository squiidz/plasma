use ast::*;
use types::{self, Program};
use object::{self, Object, ObjectType, Objecter};
use environment::*;

pub fn eval<'a>(node: &'a NodeType, mut env: &mut Environment) -> Option<Object> {
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
                    }
                    None
                }
                Expression::INFIX(ref infix) => {
                    // left and right should not be unwraped
                    let left = eval(&NodeType::Expression(*infix.clone().left), env);
                    let right = eval(&NodeType::Expression(*infix.clone().right), env);
                    return eval_infix_expression(&infix.operator, left.unwrap(), right.unwrap());
                }
                Expression::IDENT(ref ident) => return eval_identifier(ident, env),
                Expression::INTEGER(ref int) => {
                    return Some(Object::INTEGER(object::Integer { value: int.value }))
                }
                Expression::BOOL(ref bo) => return native_boolean_object(bo.value),
                Expression::STRING(ref str_lit) => {
                    return Some(Object::STRING(object::Str { value: str_lit.clone().value }))
                }
                Expression::FUNC(ref func) => {
                    return Some(Object::FUNCTION(object::Func {
                                                     parameters: func.parameters.clone(),
                                                     body: func.body.clone(),
                                                     // Env should be a ref instead of a clone
                                                     env: env.clone(),
                                                 }));
                }
                _ => panic!("INVALID EXPRESSION"),
            }
        }
        NodeType::Statement(ref stmt) => {
            match *stmt {
                Statement::VAR(ref var_stmt) => {
                    if let Some(val) = eval(&NodeType::Expression(*var_stmt
                                                                       .clone()
                                                                       .value
                                                                       .unwrap()),
                                            env) {
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
                Statement::RETURN(ref rtn) => {
                    match rtn.return_value {
                        Some(ref value) => {
                            if let Some(exp_val) = Object::expression_mapping(*value.clone()) {
                                return Some(Object::RETURN_VAL(object::Return {
                                                                   value: Box::new(exp_val),
                                                               }));
                            }
                            None
                        }
                        None => None,
                    }
                }
                _ => unimplemented!(),
            }
        }
        _ => panic!("INVALID EXPRESSION"),
    }
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
            }
            None => continue,
        }
    }
    result
}

fn eval_prefix_expression(op: &str, right: Object) -> Option<Object> {
    match op {
        "!" => eval_bang_operator(right),
        "-" => eval_minus_prefix_operator(right),
        _ => None,
    }
}

fn eval_infix_expression(op: &str, left: Object, right: Object) -> Option<Object> {
    if left.obj_type() == ObjectType::INTEGER && right.obj_type() == ObjectType::INTEGER {
        return eval_integer_infix(op, left, right);
    } else if left.obj_type() == ObjectType::STRING && right.obj_type() == ObjectType::STRING {
        return eval_string_infix(op, left, right);
    } else if op == "==" {
        return native_boolean_object(left == right);
    } else if op == "!=" {
        return native_boolean_object(left != right);
    } else if left.obj_type() != right.obj_type() {
        unimplemented!()
    }
    return unimplemented!();
}

fn eval_identifier(node: &types::Identifier, env: &mut Environment) -> Option<Object> {
    if let Some(v) = env.get(&node.value) {
        return Some(v);
    }
    None
}

fn eval_integer_infix(op: &str, left: Object, right: Object) -> Option<Object> {
    let left_value = match left {
        Object::INTEGER(v) => v.value,
        _ => return None,
    };
    let right_value = match right {
        Object::INTEGER(v) => v.value,
        _ => return None,
    };

    match op {
        "+" => return Some(Object::INTEGER(object::Integer { value: left_value + right_value })),
        "-" => return Some(Object::INTEGER(object::Integer { value: left_value - right_value })),
        "*" => return Some(Object::INTEGER(object::Integer { value: left_value * right_value })),
        "/" => return Some(Object::INTEGER(object::Integer { value: left_value / right_value })),
        "<" => return native_boolean_object(left_value < right_value),
        ">" => return native_boolean_object(left_value > right_value),
        "==" => return native_boolean_object(left_value == right_value),
        "!=" => return native_boolean_object(left_value != right_value),
        _ => return None,
    }
}

fn eval_string_infix(op: &str, left: Object, right: Object) -> Option<Object> {
    if op != "+" {
        return None;
    }
    let left_value = match left {
        Object::STRING(v) => v.value,
        _ => return None,
    };
    let right_value = match right {
        Object::STRING(v) => v.value,
        _ => return None,
    };
    return Some(Object::STRING(object::Str { value: left_value + &right_value }));
}

fn eval_bang_operator(right: Object) -> Option<Object> {
    if let Object::BOOL(b) = right {
        let v = match b {
            object::Boolean::True => Object::BOOL(object::Boolean::False),
            object::Boolean::False => Object::BOOL(object::Boolean::True),
        };
        return Some(v);
    } else {
        let v = match right {
            Object::NULL => Object::BOOL(object::Boolean::True),
            _ => Object::BOOL(object::Boolean::False),
        };
        return Some(v);
    }
}

fn eval_minus_prefix_operator(right: Object) -> Option<Object> {
    if right.obj_type() != ObjectType::INTEGER {
        return None;
    }
    let value = match right {
        Object::INTEGER(v) => v.value,
        _ => return None,
    };
    return Some(Object::INTEGER(object::Integer { value: -value }));
}

fn native_boolean_object(input: bool) -> Option<Object> {
    if input {
        return Some(Object::BOOL(object::Boolean::True));
    }
    Some(Object::BOOL(object::Boolean::False))
}

fn is_error(obj: &Object) -> bool {
    obj.obj_type() == ObjectType::ERROR
}
