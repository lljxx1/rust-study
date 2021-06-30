use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Databse {
    store: HashMap<String, String>,
}

impl Databse {
    pub fn new() -> Databse {
        Databse {
            store: HashMap::new()
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: &String) -> Option<&String> {
        let result = self.store.get(key);
        result
    }
}