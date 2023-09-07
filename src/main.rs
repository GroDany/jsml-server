use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use clap::Parser;

mod database;
mod routes;
mod source;
mod state;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
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
    let mut data = source::Source::from(args.source.as_str());
    if let Err(e) = data.process() {
        eprintln!("Error: {e}");
        std::process::exit(1)
    }
    let Ok(database) = database::Database::new(args.id.as_str(), &data.source) else {
        eprintln!("Error: invalid file content");
        std::process::exit(1)
    };

    let database = web::Data::new(Mutex::new(database));

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(database.clone())
            .service(routes::hello)
            .service(routes::get_all)
            .service(routes::get_one)
            .service(routes::put_one)
            .service(routes::post_one)
            .service(routes::patch_one)
            .service(routes::delete)
    })
    .bind(("127.0.0.1", args.port as u16))?
    .run()
    .await
}
