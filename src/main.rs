extern crate lazy_static;

use thumbor_rust::{controller, settings::{conf, Settings}};
use actix_web::{App, HttpServer};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    Settings::start();
    println!("Settings {:?}", conf());
    let n_workers = num_cpus::get() * 2;
    println!("Starting server with {} workers", n_workers);

    HttpServer::new(|| {
        App::new()
            .service(controller::file_cv)
    })
    .workers(n_workers)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
