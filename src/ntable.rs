use super::ast;
use std::collections::HashMap;

pub struct Variable {
    pub name: String,
    pub value: i64,
}

pub struct NameTable {
    map: HashMap<String, Variable>,
}

impl Variable {
    pub fn new(string: &String, value: i64) -> Variable {
        Variable {
            name: string.clone(),
            value: value,
        }
    }
}

impl Clone for Variable {
    fn clone(&self) -> Self {
        Variable {
            name: self.name.clone(),
            value: self.value,
        }
    }
}

impl NameTable {
    pub fn get(&self, key: &String) -> Result<Variable, String> {
        let value = self.map.get(key);
        match value {
            Some(v) => return Ok(v.clone()),
            None => return Err(format!("variable '{}' not found!", key)),
        }
    }
    pub fn set(&mut self, key: String, variable: Variable) {
        self.map.insert(key, variable);
    }
    pub fn new() -> NameTable {
        NameTable {
            map: HashMap::<String, Variable>::new(),
        }
    }
}