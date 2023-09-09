use serde_json::{json, Value};

use crate::database::Database;
use crate::jsml_error::JsmlError;
use crate::source::Source;

#[derive(Debug)]
pub struct State {
    pub database: Database,
    source: Source,
}

impl State {
    pub fn new(path: &str, id_key: &str) -> Result<Self, JsmlError> {
        let mut source = Source::new(path);
        let database = Database::new(id_key, &source.process()?)?;
        Ok(Self { database, source })
    }

    pub fn query(&self, route: &str) -> Result<Value, JsmlError> {
        self.database.query(route)
    }

    pub fn get(&self, route: &str, id: &str) -> Result<Value, JsmlError> {
        self.database.get(route, id)
    }

    pub fn delete(&mut self, route: &str, id: &str) -> Result<(), JsmlError> {
        let result = self.database.delete(&route, &id);
        match result {
            Ok(_) => {
                let content = json!(self.database.serialize_all());
                self.source.write_all(&content)?;
                return Ok(());
            }
            Err(e) => Err(e),
        }
    }

    pub fn put(&mut self, route: &str, id: &str, body: &Value) -> Result<Value, JsmlError> {
        let result = self.database.put(route, id, body);
        match result {
            Ok(res) => {
                let content = json!(self.database.serialize_all());
                self.source.write_all(&content)?;
                return Ok(res);
            }
            Err(e) => Err(e),
        }
    }

    pub fn patch(&mut self, route: &str, id: &str, body: &Value) -> Result<Value, JsmlError> {
        let result = self.database.patch(route, id, body);
        match result {
            Ok(res) => {
                let content = json!(self.database.serialize_all());
                self.source.write_all(&content)?;
                return Ok(res);
            }
            Err(e) => Err(e),
        }
    }

    pub fn post(&mut self, route: &str, body: &Value) -> Result<Value, JsmlError> {
        let result = self.database.post(route, body);
        match result {
            Ok(res) => {
                let content = json!(self.database.serialize_all());
                self.source.write_all(&content)?;
                return Ok(res);
            }
            Err(e) => Err(e),
        }
    }
}
