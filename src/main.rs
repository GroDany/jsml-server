#![warn(clippy::all, clippy::perf)]

use std::{io::Error, sync::Mutex};

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use clap::Parser;

mod database;
mod jsml_error;
mod logger;
mod panel;
mod routes;
mod source;
mod state;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path of the source json file
    #[arg(short, long)]
    source: String,

    /// Port default: 4242
    #[arg(short, long, default_value_t = 4242)]
    port: usize,

    /// Identifier default _id
    #[arg(long, default_value_t = String::from("id"))]
    id: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let state = state::State::new(&args)?;
    let state = web::Data::new(Mutex::new(state));
    let Ok(port) = u16::try_from(args.port) else {
        return Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid port"));
    };

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(middleware::NormalizePath::trim())
            .wrap(cors)
            .app_data(state.clone())
            .service(panel::hello)
            .service(panel::routes)
            .service(routes::get_all)
            .service(routes::get_one)
            .service(routes::put_one)
            .service(routes::post_one)
            .service(routes::patch_one)
            .service(routes::delete)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
