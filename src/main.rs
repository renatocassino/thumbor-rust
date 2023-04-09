use thumbor_rust::controller;
use actix_web::{App, HttpServer};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let n_workers = num_cpus::get() * 2;
    println!("Starting server with {} workers", n_workers);
    HttpServer::new(|| {
        App::new()
            .service(controller::greet)
            .service(controller::file_cv)
    })
    .workers(n_workers)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
