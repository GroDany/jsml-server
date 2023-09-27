use std::sync::Arc;

use serde_json::Value;

use crate::database::Database;
use crate::jsml_error::JsmlError;
use crate::logger::LogEntry;
use crate::source::Source;
use crate::Args;

pub struct State {
    pub port: usize,
    pub database: Database,
    entries: Vec<Arc<dyn LogEntry>>,
    source: Source,
}

impl State {
    pub fn new(args: &Args) -> Result<Self, JsmlError> {
        let mut source = Source::new(&args.source);
        let database = Database::new(&args.id, &source.process()?)?;
        Ok(Self {
            port: args.port,
            database,
            source,
            entries: vec![],
        })
    }

    pub fn query(
        &self,
        route: &str,
        page: Option<usize>,
        limit: Option<usize>,
    ) -> Result<Value, JsmlError> {
        self.database.query(route, page, limit)
    }

    pub fn get(&self, route: &str, id: &str) -> Result<Value, JsmlError> {
        self.database.get(route, id)
    }

    pub fn delete(&mut self, route: &str, id: &str, flush: bool) -> Result<(), JsmlError> {
        let result = self.database.delete(route, id);
        match result {
            Ok(_) => {
                if flush {
                    self.source.write_all(&self.database)?;
                }
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn put(
        &mut self,
        route: &str,
        id: &str,
        body: &Value,
        flush: bool,
    ) -> Result<Value, JsmlError> {
        let result = self.database.put(route, id, body);
        match result {
            Ok(res) => {
                if flush {
                    self.source.write_all(&self.database)?;
                }
                Ok(res)
            }
            Err(e) => Err(e),
        }
    }

    pub fn patch(
        &mut self,
        route: &str,
        id: &str,
        body: &Value,
        flush: bool,
    ) -> Result<Value, JsmlError> {
        let result = self.database.patch(route, id, body);
        match result {
            Ok(res) => {
                if flush {
                    self.source.write_all(&self.database)?;
                }
                Ok(res)
            }
            Err(e) => Err(e),
        }
    }

    pub fn post(&mut self, route: &str, body: &Value, flush: bool) -> Result<Value, JsmlError> {
        let result = self.database.post(route, body);
        match result {
            Ok(res) => {
                if flush {
                    self.source.write_all(&self.database)?;
                }
                Ok(res)
            }
            Err(e) => Err(e),
        }
    }

    pub fn log(&mut self, entry: impl LogEntry + 'static) {
        self.entries.push(Arc::new(entry));
    }
}
