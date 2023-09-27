use actix_web::{delete, get, http::StatusCode, patch, post, put, web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::Value;
use std::sync::Mutex;

use crate::{
    logger::{LogEntry, RouteEntry},
    state::State,
};

#[derive(Debug, Deserialize)]
struct QueryParams {
    limit: Option<usize>,
    page: Option<usize>,
}

#[get("/{route}")]
async fn get_all(
    path: web::Path<String>,
    query: web::Query<QueryParams>,
    data: web::Data<Mutex<State>>,
) -> impl Responder {
    let route = path.into_inner();
    let Ok(mut data) = data.lock() else {
        return HttpResponse::InternalServerError().body("Internal Server Error");
    };
    let result = data.query(&route, query.page, query.limit);
    let mut log = RouteEntry::new(&format!("localhost:{}/{route}", data.port));
    match result {
        Ok(response) => {
            log.update(StatusCode::OK);
            data.log(log);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            log.update(StatusCode::NOT_FOUND);
            data.log(log);
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
    let Ok(mut data) = data.lock() else {
        return HttpResponse::InternalServerError().body("Internal Server Error");
    };
    let mut log = RouteEntry::new(&format!("localhost:{}/{route}/{id}", data.port));
    let result = data.get(&route, &id);
    match result {
        Ok(response) => {
            log.update(StatusCode::OK);
            data.log(log);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            log.update(StatusCode::NOT_FOUND);
            data.log(log);
            HttpResponse::NotFound().body(format!("Error: {e}"))
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
    let Ok(mut data) = data.lock() else {
        return HttpResponse::InternalServerError().body("Internal Server Error");
    };
    let mut log = RouteEntry::new(&format!("localhost:{}/{route}/{id}", data.port));
    let flush = false;
    let result = data.put(&route, &id, &body, flush);
    match result {
        Ok(response) => {
            log.update(StatusCode::OK);
            data.log(log);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            log.update(StatusCode::NOT_FOUND);
            data.log(log);
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
    let mut log = RouteEntry::new(&format!("localhost:{}/{route}/{id}", data.port));
    let flush = false;
    let result = data.patch(&route, &id, &body, flush);
    match result {
        Ok(response) => {
            log.update(StatusCode::OK);
            data.log(log);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            log.update(StatusCode::NOT_FOUND);
            data.log(log);
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
    let mut log = RouteEntry::new(&format!("localhost:{}/{route}", data.port));
    let flush = false;
    let result = data.post(&route, &body, flush);
    match result {
        Ok(response) => {
            log.update(StatusCode::OK);
            data.log(log);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            log.update(StatusCode::NOT_FOUND);
            data.log(log);
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
    let mut log = RouteEntry::new(&format!("localhost:{}/{route}/{id}", data.port));
    let flush = false;
    let result = data.delete(&route, &id, flush);
    match result {
        Ok(response) => {
            log.update(StatusCode::OK);
            data.log(log);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            log.update(StatusCode::NOT_FOUND);
            data.log(log);
            HttpResponse::NotFound().body(format!("Error: {e}"))
        }
    }
}
