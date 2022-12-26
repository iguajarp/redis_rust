use std::{collections::HashMap, time::{Instant, Duration}};

pub struct Cache {
    data: HashMap<String, Entry>,
}

struct Entry {
    t: Option<Instant>,
    value: String,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        let entry = Entry {t: None, value};
        self.data.insert(key, entry);
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        match self.data.get(key.as_str()) {
            Some(entry) => {
                if let Some(t) = &entry.t {
                    if Instant::now() > t.clone() {
                        self.data.remove(key.as_str());
                        return None;
                    }
                }

                Some(entry.value.clone())
            }
            None => None,
        }
    }

    pub fn set_px(&mut self, key: String, value: String, px: u64) {
        let entry = Entry {
            t: Some(Instant::now() + Duration::from_millis(px)),
            value,
        };
        self.data.insert(key, entry);
    }
}