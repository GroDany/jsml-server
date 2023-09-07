use serde_json::{json, Value};
use std::{collections::HashMap, error, fmt};

#[derive(Debug)]
pub struct DatabaseError {
    pub details: String,
}

impl DatabaseError {
    pub fn new(msg: &str) -> Self {
        Self {
            details: msg.to_string(),
        }
    }
}

impl error::Error for DatabaseError {
    fn description(&self) -> &str {
        &self.details
    }
}

// impl<T: error::Error + Send + Sync + 'static> From<T> for DatabaseError {
//     fn from(e: T) -> Self {
//         Self {
//             details: e.to_string(),
//         }
//     }
// }

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl From<DatabaseError> for std::io::Error {
    fn from(err: DatabaseError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, err.details.as_str())
    }
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
            return Err(DatabaseError::new("Error: invalid file content"));
        };

        for (key, value) in data {
            let Some(collection) = value.as_array() else {
                return Err(DatabaseError::new("Error: invalid file content"));
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
                return Err(DatabaseError::new(
                    format!("No field named: '{}'", id).as_str(),
                ));
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
