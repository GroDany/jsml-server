use itertools::Itertools;
use serde_json::{json, Value};
use std::collections::HashMap;
use uuid::Uuid;

use crate::jsml_error::JsmlError;

#[derive(Debug)]
pub struct Database {
    pub id_key: String,
    pub database: HashMap<String, HashMap<String, Value>>,
}

impl Database {
    pub fn new(id_key: &str, data: &Value) -> Result<Self, JsmlError> {
        let mut database = HashMap::new();
        let Some(data) = data.as_object() else {
            return Err(JsmlError::new("Error: invalid file content"));
        };

        for (key, value) in data {
            let Some(collection) = value.as_array() else {
                return Err(JsmlError::new("Error: invalid file content"));
            };
            let mut col = HashMap::new();
            for item in collection.iter() {
                let Value::String(key) = &item[id_key] else {
                return Err(JsmlError::new(
                    &format!("No field named: '{id_key}'"),
                ));
            };
                col.insert(String::from(key), item.clone());
            }
            database.insert(key.to_string(), col);
        }
        Ok(Self {
            id_key: String::from(id_key),
            database,
        })
    }

    pub fn query(&self, route: &str) -> Result<Value, JsmlError> {
        let Some(collection) = self.database.get(route) else {
            return Err(JsmlError::new(&format!("collection {route} not found")));
        };
        let mut response: Vec<&Value> = vec![];
        for key in collection.keys().sorted() {
            response.push(&collection[key]);
        }
        Ok(json!(response))
    }

    pub fn get(&self, route: &str, id: &str) -> Result<Value, JsmlError> {
        let Some(collection) = self.database.get(route) else {
            return Err(JsmlError::new(&format!("collection {route} not found")));
        };
        let Some(response) = collection.get(id) else {
            return Err(JsmlError::new(&format!("item {route}/{id} not found")));
        };
        Ok(json!(response))
    }

    pub fn delete(&mut self, route: &str, id: &str) -> Result<(), JsmlError> {
        let Some(collection) = self.database.get_mut(route) else {
            return Err(JsmlError::new(&format!("collection {route} not found")));
        };
        match collection.remove(id) {
            None => Err(JsmlError::new(&format!("item {route}/{id} not found"))),
            _ => Ok(()),
        }
    }

    pub fn put(&mut self, route: &str, id: &str, body: &Value) -> Result<Value, JsmlError> {
        let Some(col) = self.database.get_mut(route) else {
            return Err(JsmlError::new(&format!("collection {route} not found")));
        };
        let Some(item) = col.get_mut(id) else {
            return Err(JsmlError::new(&format!("item {route}/{id} not found")));
        };
        let Some(body) = body.as_object() else {
            return Err(JsmlError::new("invalid request body"));
        };

        item.as_object_mut();
        let id = &item[&self.id_key].clone();
        *item = json!(serde_json::Value::Null);
        for (key, value) in body {
            item[key] = value.clone();
        }
        item[&self.id_key] = id.clone();
        Ok(item.clone())
    }

    pub fn patch(&mut self, route: &str, id: &str, body: &Value) -> Result<Value, JsmlError> {
        let Some(col) = self.database.get_mut(route) else {
            return Err(JsmlError::new(&format!("collection {route} not found")));
        };
        let Some(item) = col.get_mut(id) else {
            return Err(JsmlError::new(&format!("item {route}/{id} not found")));
        };
        let Some(body) = body.as_object() else {
            return Err(JsmlError::new("invalid request body"));
        };

        item.as_object_mut();
        for (key, value) in body {
            item[key] = value.clone();
        }
        Ok(item.clone())
    }

    pub fn post(&mut self, route: &str, body: &Value) -> Result<Value, JsmlError> {
        let Some(col) = self.database.get_mut(route) else {
            return Err(JsmlError::new(&format!("collection {route} not found")));
        };
        let Some(body) = body.as_object() else {
            return Err(JsmlError::new("invalid request body"));
        };
        let mut body = body.clone();
        if let Some(id) = &body.get(&self.id_key) {
            let Some(id) = id.as_str() else {
                return Err(JsmlError::new("invalid request body"));
            };
            if col.get(id).is_some() {
                return Err(JsmlError::new(&format!("duplicate id: {id}")));
            }
        } else {
            body.insert(self.id_key.clone(), json!(Uuid::new_v4().to_string()));
        }
        col.insert(self.id_key.clone(), json!(body));
        Ok(json!(body))
    }

    pub fn serialize_all(&self) -> Value {
        let mut response = json!({});
        for key in self.database.keys() {
            let mut result: Vec<&Value> = vec![];
            for subkey in self.database[key].keys().sorted() {
                result.push(&self.database[key][subkey]);
            }
            response[key] = json!(result);
        }
        response
    }
}
