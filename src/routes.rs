use actix_web::{delete, get, patch, post, put, web, HttpResponse, Responder};
use serde_json::{json, Value};
use std::sync::Mutex;
use uuid::Uuid;

use crate::indexer;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Greetings !")
}

#[get("/{route}")]
async fn get_all(
    path: web::Path<String>,
    data: web::Data<Mutex<indexer::Database>>,
) -> impl Responder {
    let route = path.into_inner();
    let data = data.lock().unwrap();
    let Some(collection) = data.database.get(&route) else {
        return HttpResponse::NotFound().body(format!("collection {route} not found"));
    };

    match &collection.get_all() {
        serde_json::Value::Null => {
            HttpResponse::NotFound().body(format!("collection {route} not found"))
        }
        serde_json::Value::Array(collection) => HttpResponse::Ok().json(collection),
        _ => HttpResponse::InternalServerError().into(),
    }
}

#[get("/{route}/{id}")]
async fn get_one(
    path: web::Path<(String, String)>,
    data: web::Data<Mutex<indexer::Database>>,
) -> impl Responder {
    let (route, id) = path.into_inner();
    let data = data.lock().unwrap();
    let Some(col) = &data.database.get(&route) else {
        return HttpResponse::NotFound().body(format!("collection {route} not found"));
    };
    let Some(item) = col.collection.get(&id) else {
        return HttpResponse::NotFound().body(format!("item {route}/{id} not found"));
    };

    match item {
        serde_json::Value::Null => {
            HttpResponse::NotFound().body(format!("item {route}/{id} not found"))
        }
        item => {
            let Some(item) = item.as_object() else {
                return HttpResponse::NotFound().body(format!("item {route}/{id} not found"));
            };
            HttpResponse::Ok().json(item)
        }
    }
}

#[put("/{route}/{id}")]
async fn put_one(
    path: web::Path<(String, String)>,
    data: web::Data<Mutex<indexer::Database>>,
    body: web::Json<Value>,
) -> impl Responder {
    let (route, id) = path.into_inner();
    let mut data = data.lock().unwrap();
    let Some(col) = data.database.get_mut(&route) else {
        return HttpResponse::NotFound().body(format!("collection {route} not found"));
    };
    let Some(item) = col.collection.get_mut(&id) else {
        return HttpResponse::NotFound().body(format!("item {route}/{id} not found"));
    };
    let Some(body) = body.as_object() else {
        return HttpResponse::InternalServerError().body("Error: invalid body format");
    };

    item.as_object_mut();
    *item = json!(serde_json::Value::Null);
    for (key, value) in body {
        item[key] = value.to_owned();
    }

    match item {
        serde_json::Value::Null => {
            HttpResponse::NotFound().body(format!("item {route}/{id} not found"))
        }
        item => {
            let Some(item) = item.as_object() else {
                return HttpResponse::NotFound().body(format!("item {route}/{id} not found"));
            };
            HttpResponse::Ok().json(item)
        }
    }
}

#[patch("/{route}/{id}")]
async fn patch_one(
    path: web::Path<(String, String)>,
    data: web::Data<Mutex<indexer::Database>>,
    body: web::Json<Value>,
) -> impl Responder {
    let (route, id) = path.into_inner();
    let mut data = data.lock().unwrap();
    let Some(col) = data.database.get_mut(&route) else {
        return HttpResponse::NotFound().body(format!("collection {route} not found"));
    };
    let Some(item) = col.collection.get_mut(&id) else {
        return HttpResponse::NotFound().body(format!("item {route}/{id} not found"));
    };
    let Some(body) = body.as_object() else {
        return HttpResponse::InternalServerError().body("Error: invalid body format");
    };

    item.as_object_mut();
    for (key, value) in body {
        item[key] = value.to_owned();
    }

    match item {
        serde_json::Value::Null => {
            HttpResponse::NotFound().body(format!("item {route}/{id} not found"))
        }
        item => {
            let Some(item) = item.as_object() else {
                return HttpResponse::NotFound().body(format!("item {route}/{id} not found"));
            };
            HttpResponse::Ok().json(item)
        }
    }
}

#[post("/{route}")]
async fn post_one(
    path: web::Path<String>,
    data: web::Data<Mutex<indexer::Database>>,
    body: web::Json<Value>,
) -> impl Responder {
    let route = path.into_inner();
    let mut data = data.lock().unwrap();
    let key = data.id_key.to_string();
    let Some(collection) = data.database.get_mut(&route) else {
        return HttpResponse::NotFound().body(format!("collection {route} not found"));
    };
    let mut body = body.0;
    let id: Option<String> = match &body[&key] {
        Value::Null => {
            let _id = Uuid::new_v4().to_string();
            body[key] = json!(&_id);
            Some(_id)
        }
        Value::String(current_id) => Some(current_id.to_owned()),
        _ => None,
    };
    let Some(id) = id else {
        return HttpResponse::NotFound().body(format!("collection {route} not found"));
    };
    let res = body.clone();
    collection.collection.insert(id, body);

    HttpResponse::Ok().json(res)
}

#[delete("/{route}/{id}")]
async fn delete(
    path: web::Path<(String, String)>,
    data: web::Data<Mutex<indexer::Database>>,
) -> impl Responder {
    let (route, id) = path.into_inner();
    let mut data = data.lock().unwrap();
    let Some(collection) = data.database.get_mut(&route) else {
        return HttpResponse::NotFound().body(format!("collection {route} not found"));
    };
    let result = collection.collection.remove(&id);
    if result == None {
        return HttpResponse::NotFound().body(format!("item {route}/{id} not found"));
    }

    HttpResponse::Ok().into()
}
