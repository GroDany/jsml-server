use std::sync::Mutex;

use actix_web::{get, web, HttpResponse, Responder};
use html_to_string_macro::html;

use crate::state::State;

fn display_routes(routes: Vec<&str>) -> String {
    let mut result = String::new();
    for route in routes.iter() {
        let item = html!(
            <p hx-get="/test" hx-trigger="click"> "localhost:4242/"{route} </p>
        );
        result = format!("{result}\n{item}");
    }
    result
}

#[get("/test")]
pub async fn test(data: web::Data<Mutex<State>>) -> impl Responder {
    let data = data.lock().unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("Test {}", data.port))
}

#[get("/")]
pub async fn hello(data: web::Data<Mutex<State>>) -> impl Responder {
    let data = data.lock().unwrap();
    let mut routes = vec![];
    for key in data.database.database.keys() {
        routes.push(key.as_str());
    }
    let hello = html!(
        <html>
            <head>
                <title>"JSML Server"</title>
                <script
                    src="https://unpkg.com/htmx.org@1.9.5"
                    integrity="sha384-xcuj3WpfgjlKF+FXhSQFQ0ZNr39ln+hwjN3npfM9VBnUskLolQAcN80McRIVOPuO"
                    crossorigin="anonymous">
                </script>
            </head>
            <body>
                <h1>"Welcome to JSML Server Panel !"</h1>
                { display_routes(routes) }
            </body>
        </html>
    );
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(hello.to_string())
}
