use std::collections::HashMap;

use object::*;

#[derive(Debug, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
    out: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment{
            store: HashMap::new(),
            out: None,
        }
    }

    pub fn new_enclosed(&mut self) {
        let env = Environment::new();
        self.out = Some(Box::new(env));
        //Some(*self.out.unwrap())
    }

    pub fn get(&mut self, name: &str) -> Option<Object> {
        if let Some(obj) = self.store.get(name) {
            return Some(obj.clone())
        } else if let Some(ref mut out_env) = self.out {
            return out_env.get(name)
        }
        None
    }

    pub fn set(&mut self, name: &str, obj: Object) -> Option<Object> {
        self.store.insert(name.to_owned(), obj)
    }
}