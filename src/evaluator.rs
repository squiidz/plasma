use ast::*;
use environment::Environment;
use types::{self, Program};
use object::{self, Object, ObjectType, Objecter};

pub fn eval<'a>(node: &'a NodeType, mut env: &mut Environment) -> Option<Object> {
    match *node {
        NodeType::Program(ref prog) => eval_program(prog, env),
        NodeType::Expression(ref exp) => eval_expression_type(exp, env),
        NodeType::Statement(ref stmt) => eval_statement_type(stmt, env),
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

fn eval_expression_type(exp: &Expression, env: &mut Environment) -> Option<Object> {
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
            if let Some(left) = eval(&NodeType::Expression(*infix.clone().left), env) {
                if let Some(right) = eval(&NodeType::Expression(*infix.clone().right), env) {
                    return eval_infix_expression(&infix.operator, left, right);
                }
            }
            None
        }
        Expression::IDENT(ref ident) => return eval_identifier(ident, env),
        Expression::INTEGER(ref int) => {
            return Some(Object::INTEGER(object::Integer { value: int.value }))
        }
        Expression::BOOL(ref bo) => return native_boolean_object(bo.value),
        Expression::STRING(ref str_lit) => {
            return Some(Object::STRING(object::Str { value: str_lit.clone().value }))
        }
        Expression::IF(ref if_exp) => return eval_if_expression(if_exp.clone(), env),
        Expression::FUNC(ref func) => {
            return Some(Object::FUNCTION(object::Func {
                                             parameters: func.parameters.clone(),
                                             body: func.body.clone(),
                                             // Env should be a ref instead of a clone
                                             env: env.clone(),
                                         }));
        }
        Expression::CALL(ref call) => {
            if let Some(func) = eval(&NodeType::Expression(*call.function.clone()), env) {
                if let Some(args) = eval_expression(call.clone().arguments, env) {
                    return apply_function(func, args);
                }
            }
            None
        }
        _ => {
            println!("{:?}", exp);
            return Some(Object::NULL);
        }
    }
}

fn eval_statement_type(stmt: &Statement, env: &mut Environment) -> Option<Object> {
    match *stmt {
        Statement::VAR(ref var_stmt) => {
            if let Some(val) = eval(&NodeType::Expression(*var_stmt.clone().value.unwrap()), env) {
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
        Statement::BLOCK_STMT(ref blk_stmt) => return eval_block(blk_stmt.clone(), env),
        Statement::RETURN(ref rtn) => {
            if let Some(rtn_val) = rtn.return_value.clone() {
                if let Some(value) = eval(&NodeType::Expression(*rtn_val), env) {
                    let exp_val = Object::from(value);
                    return Some(Object::RETURN_VAL(object::Return { value: Box::new(exp_val) }));
                }
            }
            None
        }
    }
}

fn eval_expression(exps: Vec<Expression>, env: &mut Environment) -> Option<Vec<Object>> {
    let mut result: Vec<Object> = Vec::new();

    for exp in exps {
        if let Some(evaluated) = eval(&NodeType::Expression(exp), env) {
            result.push(evaluated)
        }
    }
    Some(result)
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
        return eval_string_infix(op, left, right);
    }
    return unimplemented!();
}

fn eval_if_expression(if_exp: types::IfExpression, env: &mut Environment) -> Option<Object> {
    if let Some(condition) = eval(&NodeType::Expression(*if_exp.condition), env) {
        if is_truthy(condition) {
            return eval(&NodeType::Statement(*if_exp.consequence), env);
        } else if let Some(alt) = if_exp.alternative {
            return eval(&NodeType::Statement(alt), env);
        }
    }
    None
}

fn eval_block(block: types::BlockStatement, env: &mut Environment) -> Option<Object> {
    let mut result = Object::NULL;

    for stmt in block.statements {
        if let Some(res) = eval(&NodeType::Statement(stmt), env) {
            result = res;
            if result.obj_type() == ObjectType::RETURN_VAL ||
               result.obj_type() == ObjectType::ERROR {
                return Some(result);
            }
        }
    }
    Some(result)
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
        "^" => return Some(Object::INTEGER(object::Integer { value: left_value.pow(right_value as u32) })),
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
        _ => left.inspect(),
    };
    let right_value = match right {
        Object::STRING(v) => v.value,
        _ => right.inspect(),
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

fn apply_function(func: Object, args: Vec<Object>) -> Option<Object> {
    match func {
        Object::FUNCTION(fun) => {
            if let Some(mut ext_env) = extend_function_env(fun.clone(), &args) {
                if let Some(evaluated) = eval(&NodeType::Statement(fun.body), &mut ext_env) {
                    return unwrap_return_value(evaluated);
                }
            }
            None
        }
        Object::BUILTIN(ref blt_in) => unimplemented!(),
        _ => return Some(Object::ERROR(object::Error { message: "Invalid Type".to_owned() })),
    }
}

fn extend_function_env(func: object::Func, args: &[Object]) -> Option<Environment> {
    let mut new_env = func.env.new_enclosed();
    for (id, param) in func.parameters.iter().enumerate() {
        // params.to_string() is not the correct key, should be param.value
        new_env.set(&param.to_string(), args[id].clone());
    }
    Some(new_env)
}

fn unwrap_return_value(obj: Object) -> Option<Object> {
    if let Object::RETURN_VAL(rtn_val) = obj {
        return Some(*rtn_val.value);
    }
    Some(obj)
}

fn is_truthy(obj: Object) -> bool {
    match obj {
        Object::NULL => false,
        Object::BOOL(b) => {
            match b {
                object::Boolean::True => true,
                object::Boolean::False => false,
            }
        }
        _ => true,
    }
}

fn is_error(obj: &Object) -> bool {
    obj.obj_type() == ObjectType::ERROR
}
