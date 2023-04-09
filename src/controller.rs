use actix_web::HttpRequest;
use opencv::prelude::MatTraitConstManual; // to get method `.size()` must have this use
use opencv::{core::{Mat}};
use opencv::core::Vector;
use crate::{calc, service::image::get_image};
use actix_web::{get, web::{self}, HttpResponse};

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("Content-Type", "text/html"))
        .body(format!("Hello {}!", name))
}

#[get("/{key}/{width:\\d+}x{height:\\d+}/{smart:(smart/)?}{filename:.*}")]
pub async fn file_cv(req: HttpRequest) -> HttpResponse {
    let _key = req.match_info().get("key").unwrap();
    let width = req.match_info().get("width").unwrap().parse::<i32>().unwrap();
    let height = req.match_info().get("height").unwrap().parse::<i32>().unwrap();
    let _smart: bool = req.match_info().get("smart").unwrap() == "smart/";
    let filename = req.match_info().get("filename").unwrap();

    let img = get_image(filename).await;

    let original_size = img.size().unwrap();
    let new_size = opencv::core::Size { width: width, height: height };
    let new_size_with_aspect = calc::get_new_size_respecting_aspect_ratio(original_size, new_size);

    let mut resized_image = Mat::default();
    opencv::imgproc::resize(&img, &mut resized_image, new_size_with_aspect, 0.0, 0.0, opencv::imgproc::INTER_AREA).unwrap();

    let cropped_image = Mat::roi(&resized_image, opencv::core::Rect {
        x: ((new_size_with_aspect.width - width) / 2) as i32,
        y: ((new_size_with_aspect.height - height) / 2) as i32,
        width: width,
        height: height,
    }).unwrap();

    let mut out_vector: Vector<u8> = Vector::new();
    opencv::imgcodecs::imencode(".jpg", &cropped_image, &mut out_vector, &Vector::new()).expect("Encode image");

    HttpResponse::Ok()
        .append_header(("Content-Type", "image/jpeg"))
        .body(out_vector.to_vec())
}
