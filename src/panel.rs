use std::sync::Mutex;

use actix_web::{get, web, HttpResponse, Responder};
use html_to_string_macro::html;
use serde_json::Value;

use crate::{routes::QueryParams, state::State};

fn display_routes(r: Vec<&str>) -> String {
    let mut result = String::new();
    for route in r {
        let item = html!(
            <h2
                { format!("hx-get=\"/jsml_{route}\"") }
                hx-trigger="click"
                hx-target="#collection"
                style="padding: 0 10%; text-align: center"
            >
                { route }
            </h2>
        );
        result = format!("{result}\n{item}");
    }
    result
}

fn display_item(i: &Value) -> String {
    i.to_string()
}

fn display_items(r: &Value) -> String {
    let Some(r) = r.as_array() else {
        return "Error: invalid data format".to_string();
    };
    let mut result = String::new();
    for data in r.iter() {
        let item = html!(
            <p style="text-align: center">
                { display_item(data) }
            </p>
        );
        result = format!("{result}\n{item}");
    }
    result
}

#[get("/jsml_{route}")]
pub async fn routes(path: web::Path<String>, data: web::Data<Mutex<State>>) -> impl Responder {
    let route = path.into_inner();
    let Ok(items) = data.lock().expect("Internal Error").query(&route, &QueryParams::default()) else {
       return HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(format!("Error {route} not found"));
    };
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(display_items(&items))
}

#[get("/")]
pub async fn hello(data: web::Data<Mutex<State>>) -> impl Responder {
    let Ok(data) = data.lock() else {
        return HttpResponse::InternalServerError().body("Internal Server Error");
    };
    let mut r = vec![];
    for key in data.database.database.keys() {
        r.push(key.as_str());
    }
    let hello = html!(
        <html>
            <head>
                <title>"JSML Server"</title>
                <script
                    src="https://unpkg.com/htmx.org@1.9.5"
                    integrity="sha384-xcuj3WpfgjlKF+FXhSQFQ0ZNr39ln+hwjN3npfM9VBnUskLolQAcN80McRIVOPuO"
                    crossorigin="anonymous"
                >
                </script>
            </head>
            <body>
                <h1 style="text-align: center">
                    "JSML Panel (localhost:"{ data.port }")"
                </h1>
                <div style="display: flex; flex-direction: row; margin 10%; justify-content: center">
                    { display_routes(r) }
                </div>
                <div
                    style="display: flex; flex-direction: column; justify-content: center"
                    id="collection"
                >
                </div>
            </body>
        </html>
    );

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(hello)
}
