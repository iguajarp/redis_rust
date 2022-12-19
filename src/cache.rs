use std::{collections::HashMap};

pub struct Cache {
    data: HashMap<String, String>,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        self.data.get(key.as_str()).cloned()
    }
}