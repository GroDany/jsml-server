use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use clap::Parser;

mod indexer;
mod parser;
mod routes;

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
    let mut data = parser::Source::from(args.source.as_str());
    if let Err(e) = data.process() {
        eprintln!("Error: {e}");
        std::process::exit(1)
    }
    let Ok(index) = indexer::Index::new(args.id.as_str(), &data.source) else {
        eprintln!("Error: invalid file format");
        std::process::exit(1)
    };

    let data = web::Data::new(Mutex::new(index));

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(data.clone())
            .service(routes::hello)
            .service(routes::get_all)
            .service(routes::get_one)
            .service(routes::put_one)
            .service(routes::post_one)
            .service(routes::patch_one)
    })
    .bind(("127.0.0.1", args.port as u16))?
    .run()
    .await
}
