use std::{fs::File, io::{BufReader, Read}};

use actix_web::{get, web, App, HttpServer, Responder, http::{StatusCode, header}, HttpResponse};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("Content-Type", "text/html"))
        .body(format!("Hello {}!", name))
}

#[get("/file/{name}")]
async fn file(name: web::Path<String>) -> HttpResponse {
    let path = "./src/images/sun.jpg";
    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = Vec::new();
    buf_reader.read_to_end(&mut contents).unwrap();

    HttpResponse::Ok()
        .append_header(("Content-Type", "image/jpeg"))
        .body(contents)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let n_workers = num_cpus::get() * 2;
    println!("Starting server with {} workers", n_workers);
    HttpServer::new(|| {
        App::new().service(greet).service(file)
    })
    .workers(n_workers)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
