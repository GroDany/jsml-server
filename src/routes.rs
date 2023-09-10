use actix_web::{delete, get, http::StatusCode, patch, post, put, web, HttpResponse, Responder};
use serde_json::Value;
use std::sync::{Arc, Mutex};

use crate::{
    logger::{LogEntry, RouteEntry},
    state::State,
};

// TODO: Delete unwraps !

#[get("/{route}")]
async fn get_all(path: web::Path<String>, data: web::Data<Mutex<State>>) -> impl Responder {
    let route = path.into_inner();
    let mut data = data.lock().unwrap();
    let mut log = RouteEntry::new(&format!("localhost:{}/{route}", data.port));
    let result = data.query(&route);
    match result {
        Ok(response) => {
            log.update(StatusCode::OK);
            data.entries.push(Arc::new(log));
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            log.update(StatusCode::NOT_FOUND);
            data.entries.push(Arc::new(log));
            return HttpResponse::NotFound().body(format!("Error: {e}"));
        }
    }
}

#[get("/{route}/{id}")]
async fn get_one(
    path: web::Path<(String, String)>,
    data: web::Data<Mutex<State>>,
) -> impl Responder {
    let (route, id) = path.into_inner();
    let mut data = data.lock().unwrap();
    let mut log = RouteEntry::new(&format!("localhost:{}/{route}/{id}", data.port));
    let result = data.get(&route, &id);
    match result {
        Ok(response) => {
            log.update(StatusCode::OK);
            data.entries.push(Arc::new(log));
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            log.update(StatusCode::NOT_FOUND);
            data.entries.push(Arc::new(log));
            return HttpResponse::NotFound().body(format!("Error: {e}"));
        }
    }
}

#[put("/{route}/{id}")]
async fn put_one(
    path: web::Path<(String, String)>,
    data: web::Data<Mutex<State>>,
    body: web::Json<Value>,
) -> impl Responder {
    let (route, id) = path.into_inner();
    let mut data = data.lock().unwrap();
    let mut log = RouteEntry::new(&format!("localhost:{}/{route}/{id}", data.port));
    let result = data.put(&route, &id, &body);
    match result {
        Ok(response) => {
            log.update(StatusCode::OK);
            data.entries.push(Arc::new(log));
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            log.update(StatusCode::NOT_FOUND);
            data.entries.push(Arc::new(log));
            return HttpResponse::NotFound().body(format!("Error: {e}"));
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
    let mut data = data.lock().unwrap();
    let mut log = RouteEntry::new(&format!("localhost:{}/{route}/{id}", data.port));
    let result = data.patch(&route, &id, &body);
    match result {
        Ok(response) => {
            log.update(StatusCode::OK);
            data.entries.push(Arc::new(log));
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            log.update(StatusCode::NOT_FOUND);
            data.entries.push(Arc::new(log));
            return HttpResponse::NotFound().body(format!("Error: {e}"));
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
    let mut data = data.lock().unwrap();
    let mut log = RouteEntry::new(&format!("localhost:{}/{route}", data.port));
    let result = data.post(&route, &body);
    match result {
        Ok(response) => {
            log.update(StatusCode::OK);
            data.entries.push(Arc::new(log));
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            log.update(StatusCode::NOT_FOUND);
            data.entries.push(Arc::new(log));
            return HttpResponse::NotFound().body(format!("Error: {e}"));
        }
    }
}

#[delete("/{route}/{id}")]
async fn delete(
    path: web::Path<(String, String)>,
    data: web::Data<Mutex<State>>,
) -> impl Responder {
    let (route, id) = path.into_inner();
    let mut data = data.lock().unwrap();
    let mut log = RouteEntry::new(&format!("localhost:{}/{route}/{id}", data.port));
    let result = data.delete(&route, &id);
    match result {
        Ok(response) => {
            log.update(StatusCode::OK);
            data.entries.push(Arc::new(log));
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            log.update(StatusCode::NOT_FOUND);
            data.entries.push(Arc::new(log));
            return HttpResponse::NotFound().body(format!("Error: {e}"));
        }
    }
}
