use std::{fs::File, io::{BufReader, Read}};
use image::GenericImageView;
use opencv::{core::{Mat, Rect}, prelude::MatTrait};
use opencv::core::Vector;

use actix_web::{get, web::{self, Query}, App, HttpServer, Responder, http::{StatusCode, header}, HttpResponse, HttpRequest};
use serde::Deserialize;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("Content-Type", "text/html"))
        .body(format!("Hello {}!", name))
}

#[derive(Deserialize)]
struct QueryParams {
    w: i32,
    h: i32,
}

#[get("/file/{name}")]
async fn file(name: web::Path<String>, params: web::Query<QueryParams>) -> HttpResponse {
    let path = "./src/images/big.jpg";
    let file = File::open(path).unwrap();

    let mut buf_reader = BufReader::new(file);
    let mut contents = Vec::new();
    buf_reader.read_to_end(&mut contents).unwrap();

    let img = image::load_from_memory(&contents).unwrap();
    let cropped_image = img.resize_to_fill(params.w.try_into().unwrap(), params.h.try_into().unwrap(), image::imageops::FilterType::Gaussian);
    cropped_image.save("sun-cropped.jpg").unwrap();

    let file2 = File::open("sun-cropped.jpg").unwrap();
    let mut buf_reader2 = BufReader::new(file2);
    let mut contents_crop = Vec::new();
    buf_reader2.read_to_end(&mut contents_crop).unwrap();

    HttpResponse::Ok()
        .append_header(("Content-Type", "image/jpeg"))
        .body(contents_crop)
}

#[get("/file-cv/{name}")]
async fn file_cv(name: web::Path<String>, params: web::Query<QueryParams>) -> HttpResponse {
    let path = "./src/images/big.jpg";
    let mut img = opencv::imgcodecs::imread(
        path,
        opencv::imgcodecs::IMREAD_COLOR,
    ).unwrap();

    let cropped_image = Mat::roi(&img, Rect {
        x: 0,
        y: 0,
        width: params.w,
        height: params.h,
    }).unwrap();

    let mut out_vector: Vector<u8> = Vector::new();
    opencv::imgcodecs::imencode(".jpg", &cropped_image, &mut out_vector, &Vector::new()).expect("Encode image");

    HttpResponse::Ok()
        .append_header(("Content-Type", "image/jpeg"))
        .body(out_vector.to_vec())
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let n_workers = num_cpus::get() * 2;
    println!("Starting server with {} workers", n_workers);
    HttpServer::new(|| {
        App::new().service(greet).service(file).service(file_cv)
    })
    .workers(n_workers)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
