use std::collections::HashMap;

use object::*;

#[derive(Debug, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
    out: Box<Option<Environment>>,
}

impl Environment {
    pub fn get(&mut self, name: &str) -> Option<Object> {
        if let Some(obj) = self.store.get(name) {
            return Some(obj.clone())
        } else if let Some(ref mut out_env) = *self.out {
            return out_env.get(name)
        }
        None
    }
    pub fn set(&mut self, name: &str, obj: Object) -> Option<Object> {
        self.store.insert(name.to_owned(), obj)
    }
}