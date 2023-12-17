use actix_web::{delete, get, http::StatusCode, patch, post, put, web, HttpResponse, Responder};
use serde_json::Value;
use std::{collections::HashMap, sync::Mutex};

use crate::{
    logger::{LogEntry, RouteEntry},
    state::State,
};

#[derive(Default, Debug)]
pub struct QueryParams {
    pub page: Option<usize>,
    pub limit: Option<usize>,
    pub filters: HashMap<String, Vec<String>>,
}

impl QueryParams {
    fn new(query: HashMap<String, String>) -> Self {
        let mut res = Self::default();
        query.keys().for_each(|key| match key.as_str() {
            "_page" => {
                if let Ok(page) = query[key].parse::<usize>() {
                    res.page = Some(page);
                } else {
                    res.page = None;
                }
            }
            "_limit" => {
                if let Ok(limit) = query[key].parse::<usize>() {
                    res.limit = Some(limit);
                } else {
                    res.limit = None;
                }
            }
            _ => {
                let values: Vec<String> = query[key].split(',').map(String::from).collect();
                res.filters.insert(key.clone(), values);
            }
        });
        res
    }
}

#[get("/{route}")]
async fn get_all(
    path: web::Path<String>,
    query: web::Query<HashMap<String, String>>,
    data: web::Data<Mutex<State>>,
) -> impl Responder {
    let route = path.into_inner();
    let Ok(data) = data.lock() else {
        return HttpResponse::InternalServerError().body("Internal Server Error");
    };
    let query = QueryParams::new(query.into_inner());
    let result = data.query(&route, &query);
    let mut log = RouteEntry::new(&format!("GET - localhost:{}/{route}", data.port));
    match result {
        Ok(response) => {
            log.update(StatusCode::OK);
            // // data.log(Arc::new(log));
            if let Ok(response) = serde_json::to_string_pretty(&response) {
                HttpResponse::Ok().body(response)
            } else {
                HttpResponse::InternalServerError().body("Internal Server Error")
            }
        }
        Err(e) => {
            log.update(StatusCode::NOT_FOUND);
            // // data.log(log);
            HttpResponse::NotFound().body(format!("Error: {e}"))
        }
    }
}

#[get("/{route}/{id}")]
async fn get_one(
    path: web::Path<(String, String)>,
    data: web::Data<Mutex<State>>,
) -> impl Responder {
    let (route, id) = path.into_inner();
    let Ok(data) = data.lock() else {
        return HttpResponse::InternalServerError().body("Internal Server Error");
    };
    let mut log = RouteEntry::new(&format!("GET - localhost:{}/{route}/{id}", data.port));
    let result = data.get(&route, &id).cloned();
    match result {
        Ok(response) => {
            // // data.log(Arc::new(log));
            if let Ok(response) = serde_json::to_string_pretty(&response) {
                log.update(StatusCode::OK);
                HttpResponse::Ok().body(response)
            } else {
                log.update(StatusCode::INTERNAL_SERVER_ERROR);
                HttpResponse::InternalServerError().body("Internal Server Error")
            }
        }
        Err(e) => {
            // // data.log(Arc::new(log));
            log.update(StatusCode::NOT_FOUND);
            HttpResponse::NotFound().body(format!("Error: {e}"))
        }
    }
}

// TODO: reduce unnecessary alloc in mutation requests
#[put("/{route}/{id}")]
async fn put_one(
    path: web::Path<(String, String)>,
    data: web::Data<Mutex<State>>,
    body: web::Json<Value>,
) -> impl Responder {
    let (route, id) = path.into_inner();
    let Ok(mut data) = data.lock() else {
        return HttpResponse::InternalServerError().body("Internal Server Error");
    };
    let mut log = RouteEntry::new(&format!("localhost:{}/{route}/{id}", data.port));
    let result = data.put(&route, &id, &body, false);
    match result {
        Ok(response) => {
            log.update(StatusCode::OK);
            // // data.log(&log);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            log.update(StatusCode::NOT_FOUND);
            // // data.log(&log);
            HttpResponse::NotFound().body(format!("Error: {e}"))
        }
    }
}

#[patch("/{route}/{id}")]
async fn patch_one(
    path: web::Path<(String, String)>,
    data: web::Data<Mutex<State>>,
    body: web::Json<Value>,
) -> impl Responder {
    let (route, id) = path.into_inner();
    let Ok(mut data) = data.lock() else {
        return HttpResponse::InternalServerError().body("Internal Server Error");
    };
    let mut log = RouteEntry::new(&format!("PATCH - localhost:{}/{route}/{id}", data.port));
    let result = data.patch(&route, &id, &body, false);
    match result {
        Ok(response) => {
            log.update(StatusCode::OK);
            // data.log(&log);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            log.update(StatusCode::NOT_FOUND);
            // data.log(&log);
            HttpResponse::NotFound().body(format!("Error: {e}"))
        }
    }
}

#[post("/{route}")]
async fn post_one(
    path: web::Path<String>,
    data: web::Data<Mutex<State>>,
    body: web::Json<Value>,
) -> impl Responder {
    let route = path.into_inner();
    let Ok(mut data) = data.lock() else {
        return HttpResponse::InternalServerError().body("Internal Server Error");
    };
    let mut log = RouteEntry::new(&format!("POST - localhost:{}/{route}", data.port));
    let result = data.post(&route, &body, true);
    match result {
        Ok(response) => {
            log.update(StatusCode::OK);
            // data.log(&log);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            log.update(StatusCode::NOT_FOUND);
            // data.log(&log);
            HttpResponse::NotFound().body(format!("Error: {e}"))
        }
    }
}

#[delete("/{route}/{id}")]
async fn delete(
    path: web::Path<(String, String)>,
    data: web::Data<Mutex<State>>,
) -> impl Responder {
    let (route, id) = path.into_inner();
    let Ok(mut data) = data.lock() else {
        return HttpResponse::InternalServerError().body("Internal Server Error");
    };
    let mut log = RouteEntry::new(&format!("DELETE - localhost:{}/{route}/{id}", data.port));
    let result = data.delete(&route, &id, false);
    match result {
        Ok(response) => {
            log.update(StatusCode::OK);
            // data.log(&log);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            log.update(StatusCode::NOT_FOUND);
            // data.log(&log);
            HttpResponse::NotFound().body(format!("Error: {e}"))
        }
    }
}
