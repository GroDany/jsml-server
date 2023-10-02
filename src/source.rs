use serde_json::Value;
use std::{fs::File, io::Read};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

use crate::database::Database;

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

    pub fn write_all(&self, db: &Database) -> std::io::Result<()> {
        let path = self.path.clone();
        let serialized = db.serialize_all();
        tokio::spawn(async move {
            let Ok(res) = serde_json::to_string_pretty(&serialized) else {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid data  in database"));
            };
            let mut file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&path)
                .await?;
            file.write_all(res.as_bytes()).await
        });
        Ok(())
    }
}
