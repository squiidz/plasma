use ast::*;
use object::*;
use environment::*;

fn eval(node: NodeType, mut env: &mut Environment) -> Object {
    match node.clone() {
        NodeType::Program(prog) => {
            unimplemented!()
        }
        NodeType::Expression(exp) => {
            match exp {
                Expression::PREFIX(prefix) => unimplemented!(),
                _ => unimplemented!(),
            }
        },
        NodeType::Statement(stmt) => {
            match stmt {
                Statement::VAR(var_stmt) => {
                    let val = eval(node, env);
                    if is_error(&val) {
                        return val
                    }
                    env.set(var_stmt.name.value.as_str(), val).unwrap()
                },
                Statement::EXPR_STMT(exp_stmt) => {
                    return eval(node, env)
                },
                _ => unimplemented!(),
            }
        }
    }
}

fn is_error(obj: &Object) -> bool {
    obj.obj_type() == ObjectType::ERROR
}