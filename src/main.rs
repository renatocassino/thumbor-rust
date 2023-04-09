use std::{fs::File, io::{BufReader, Read}};
use std::time::{Duration, Instant};
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
    let mut now = Instant::now();

    let path = "./src/images/big.jpg";
    let img = image::open(path).unwrap();
    println!("{} - Open file", now.elapsed().as_millis());
    now = Instant::now();

    let cropped_image = img.resize_to_fill(params.w.try_into().unwrap(), params.h.try_into().unwrap(), image::imageops::FilterType::Gaussian);
    println!("{} - Resize", now.elapsed().as_millis());
    now = Instant::now();

    cropped_image.save("sun-cropped.jpg").unwrap();
    println!("{} - Save image", now.elapsed().as_millis());
    now = Instant::now();

    let file2 = File::open("sun-cropped.jpg").unwrap();
    println!("{} - Read image again (useless)", now.elapsed().as_millis());
    now = Instant::now();

    let mut buf_reader2 = BufReader::new(file2);
    let mut contents_crop = Vec::new();
    buf_reader2.read_to_end(&mut contents_crop).unwrap();

    println!("{} - Create new buffer", now.elapsed().as_millis());
    HttpResponse::Ok()
        .append_header(("Content-Type", "image/jpeg"))
        .body(contents_crop)
}

#[get("/file-cv/{name}")]
async fn file_cv(name: web::Path<String>, params: web::Query<QueryParams>) -> HttpResponse {
    let mut now = Instant::now();

    let path = "./src/images/big.jpg";
    let mut img = opencv::imgcodecs::imread(
        path,
        opencv::imgcodecs::IMREAD_COLOR,
    ).unwrap();

    /** */
    // let original_size = img.size().unwrap();
    let original_size = opencv::core::Size { width: 3456, height: 5184 };
    let original_ratio = original_size.width as f32 / original_size.height as f32;

    let new_size = opencv::core::Size { width: params.w, height: params.h };
    let new_ratio = new_size.width as f32 / new_size.height as f32;
    
    let is_wider = original_ratio < new_ratio;

    println!("{} - is wider - {:?} original_ration {:?} new ratio", is_wider, original_size, new_size);
    let (mut x_offset, mut y_offset, mut width, mut height) = (0, 0, 0, 0);
    if is_wider {
        width = original_size.width;
        height = (original_size.width as f32 / new_ratio) as i32;
        x_offset = 0;
        y_offset = (original_size.height - height) / 2;
    } else {
        width = (original_size.height as f32 * new_ratio) as i32;
        height = original_size.height;
        x_offset = (original_size.width - width) / 2;
        y_offset = 0;
    }

    let new_size = opencv::core::Size { width, height };

    println!("{:?} - New size", new_size);
    let mut resized_image = Mat::default();
    opencv::imgproc::resize(&img, &mut resized_image, new_size, 0.0, 0.0, opencv::imgproc::INTER_AREA).unwrap();

    let mut cropped_image = Mat::default();
    if is_wider {
        opencv::core::copy_make_border(&resized_image, &mut cropped_image, y_offset, y_offset, x_offset, x_offset, opencv::core::BORDER_CONSTANT, opencv::core::Scalar::default()).unwrap();
    } else {
        opencv::core::copy_make_border(&resized_image, &mut cropped_image, y_offset, y_offset, x_offset, x_offset, opencv::core::BORDER_CONSTANT, opencv::core::Scalar::default()).unwrap();
    }

    let mut out_vector: Vector<u8> = Vector::new();
    opencv::imgcodecs::imencode(".jpg", &cropped_image, &mut out_vector, &Vector::new()).expect("Encode image");

    let mut debug: Vector<u8> = Vector::new();
    opencv::imgcodecs::imencode(".jpg", &resized_image, &mut debug, &Vector::new()).expect("Encode image");
    HttpResponse::Ok()
        .append_header(("Content-Type", "image/jpeg"))
        .body(debug.to_vec())

    // println!("{} - Open file", now.elapsed().as_millis());
    // now = Instant::now();

    // // let new_size = opencv::core::Size { width: params.w, height: params.h };

    // let mut resized_image = Mat::default();
    // opencv::imgproc::resize(&img, &mut resized_image, new_size, 0.0, 0.0, opencv::imgproc::INTER_AREA).unwrap();

    // println!("{} - Crop image", now.elapsed().as_millis());
    // now = Instant::now();

    // let mut out_vector: Vector<u8> = Vector::new();
    // opencv::imgcodecs::imencode(".jpg", &resized_image, &mut out_vector, &Vector::new()).expect("Encode image");

    // println!("{} - Encode image", now.elapsed().as_millis());

    // HttpResponse::Ok()
    //     .append_header(("Content-Type", "image/jpeg"))
    //     .body(out_vector.to_vec())
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
