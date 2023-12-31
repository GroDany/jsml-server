use itertools::Itertools;
use serde_json::{json, Value};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{jsml_error::JsmlError, routes::QueryParams};

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

    pub fn query<'a>(
        &'a self,
        route: &str,
        query: &QueryParams,
    ) -> Result<Vec<&'a Value>, JsmlError> {
        let Some(collection) = self.database.get(route) else {
            return Err(JsmlError::new(&format!("collection {route} not found")));
        };
        let mut response: Vec<&Value> = vec![];
        if let Some(page) = query.page {
            let limit = match query.limit {
                Some(limit) => limit,
                None => 10,
            };
            for key in collection.keys().sorted().skip(limit * page).take(limit) {
                if Self::match_query(&query, &collection[key]) {
                    response.push(&collection[key]);
                }
            }
        } else {
            for key in collection.keys().sorted() {
                if Self::match_query(&query, &collection[key]) {
                    response.push(&collection[key]);
                }
            }
        }
        Ok(response)
    }

    pub fn get(&self, route: &str, id: &str) -> Result<&Value, JsmlError> {
        let Some(collection) = self.database.get(route) else {
            return Err(JsmlError::new(&format!("collection {route} not found")));
        };
        let Some(response) = collection.get(id) else {
            return Err(JsmlError::new(&format!("item {route}/{id} not found")));
        };
        Ok(response)
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
            let body = json!(body);
            col.insert(id.to_string(), body.clone());
            return Ok(body);
        } else {
            let id = Uuid::new_v4().to_string();
            body.insert(self.id_key.clone(), json!(id));
            let body = json!(body);
            col.insert(id.to_string(), body.clone());
            return Ok(body);
        }
    }

    pub fn serialize_all(&self) -> HashMap<String, Vec<Value>> {
        let mut response = HashMap::<String, Vec<Value>>::new();
        for collection in self.database.iter() {
            response.insert(
                collection.0.to_string(),
                Vec::from_iter(collection.1.values().map(|x| x.clone())),
            );
        }
        response
    }

    fn match_query(query: &QueryParams, value: &Value) -> bool {
        query
            .filters
            .keys()
            .all(|key| match Self::get_filtered_field(value, key) {
                Some(val) => match val {
                    Value::String(val) => query.filters[key].contains(&val.to_string()),
                    Value::Number(val) => query.filters[key].contains(&val.to_string()),
                    Value::Bool(val) => query.filters[key].contains(&val.to_string()),
                    _ => false,
                },
                None => false,
            })
    }

    fn get_filtered_field(value: &Value, key: &String) -> Option<Value> {
        let keys = key.split('.');
        let mut value = value;
        for (_, key) in keys.enumerate() {
            let tmp = value.get(key);
            value = match tmp {
                Some(val) => val,
                None => return None,
            };
        }
        Some(value.clone())
    }
}
