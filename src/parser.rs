use serde_json::{json, Value};
use std::{fs::File, io::Read};

#[derive(Debug, Clone)]
pub struct Source {
    pub path: String,
    pub source: Value,
}

impl Source {
    pub fn from(path: &str) -> Self {
        Self {
            path: String::from(path),
            source: json!(null),
        }
    }

    pub fn process(&mut self) -> std::io::Result<()> {
        let mut file = File::open(&self.path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        self.source = serde_json::from_str(&content)?;
        Ok(())
    }
}
