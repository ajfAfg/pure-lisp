use crate::syntax::Value;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Environment(HashMap<String, Value>);

impl Environment {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn lookup(&self, name: &String) -> Option<&Value> {
        self.0.get(name)
    }

    pub fn extend(&mut self, name: String, value: Value) -> Option<Value> {
        self.0.insert(name, value)
    }
}
