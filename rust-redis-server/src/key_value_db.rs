use std::collections::HashMap;
use regex::Regex;

pub struct Item {
    pub ttl: u32,
    pub value: String,
}

pub struct KeyValueDB {
    hasmap: HashMap<String, Item>,
}

impl KeyValueDB {
    pub fn new() -> KeyValueDB {
        KeyValueDB { hasmap: HashMap::new() }
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.hasmap.get(&key).map(|item| item.value.clone())
    }

    pub fn set(&mut self, key: String, value: String) {
        let item = Item { ttl: 0, value };
        self.hasmap.insert(key, item);
    }

    pub fn parse_input(&self, input: &str) -> (String, String, String) {
        let re = Regex::new(r"(?i)^\s*(\w+)\s+(\w+)(?:\s+(.*))?\s*$").unwrap();

        if let Some(caps) = re.captures(input) {
            let comando = caps.get(1).map_or("", |m| m.as_str()).to_uppercase();
            let chave = caps.get(2).map_or("", |m| m.as_str()).to_string();
            let valor = caps.get(3).map_or("", |m| m.as_str()).to_string();
            (comando, chave, valor)
        } else {
            ("INVALID".to_string(), "".to_string(), "".to_string())
        }
    }
}
