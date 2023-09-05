use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Debug)]
pub struct DatabaseError {
    pub details: String,
}

#[derive(Debug)]
pub struct Database {
    pub id_key: String,
    pub database: HashMap<String, Collection>,
}

impl Database {
    pub fn new(id_key: &str, data: &Value) -> Result<Self, DatabaseError> {
        let mut database = HashMap::new();
        let Some(data) = data.as_object() else {
            eprintln!("Error: invalid file format");
            std::process::exit(1)
        };

        for (key, value) in data {
            let Some(collection) = value.as_array() else {
                eprintln!("Error: invalid file format");
                std::process::exit(1)
            };
            let col = Collection::new(id_key, collection);
            match col {
                Ok(col) => {
                    database.insert(key.to_string(), col);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(Self {
            id_key: String::from(id_key),
            database,
        })
    }
}

#[derive(Default, Debug)]
pub struct Collection {
    pub source: Value,
    pub collection: HashMap<String, Value>,
}

impl Collection {
    pub fn new(id: &str, data: &Vec<Value>) -> Result<Self, DatabaseError> {
        let mut collection = HashMap::new();
        for item in data.iter() {
            let Value::String(key) = &item[id] else {
                return Err(DatabaseError { details: format!("No field named: '{}'", id) });
            };
            collection.insert(String::from(key), item.clone());
        }
        Ok(Self {
            source: json!(data),
            collection,
        })
    }

    pub fn get_all(&self) -> Value {
        let mut res: Vec<Value> = vec![];
        for (_, value) in &self.collection {
            res.push(value.clone());
        }
        json!(res)
    }
}
