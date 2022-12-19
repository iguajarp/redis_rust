use std::{collections::HashMap};

pub struct Cache {
    data: HashMap<String, String>,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            data: HashMap::new(),
        }
    }
}