use serde_json::Value;
use std::{
    fs::File,
    io::{BufWriter, Read, Write},
};

#[derive(Debug)]
pub struct Source {
    pub path: String,
}

impl Source {
    pub fn new(path: &str) -> Self {
        Self {
            path: String::from(path),
        }
    }

    pub fn process(&mut self) -> std::io::Result<Value> {
        let mut file = File::open(&self.path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let source = serde_json::from_str(&content)?;
        Ok(source)
    }

    pub fn write_all(&self, content: &Value) -> std::io::Result<()> {
        let file = File::create(&self.path)?;
        let mut writer = BufWriter::new(&file);
        serde_json::to_writer_pretty(&mut writer, &content)?;
        writer.flush()?;
        Ok(())
    }
}
