use actix_web::HttpRequest;
use opencv::prelude::MatTraitConstManual; // to get method `.size()` must have this use
use opencv::{core::{Mat}};
use opencv::core::Vector;
use crate::security;
use crate::settings::{Settings};
use crate::{calc, service::image::get_image};
use actix_web::{get, HttpResponse};

#[get("/{key}/{width:\\d+}x{height:\\d+}/{smart:(smart/)?}{filename:.*}")]
pub async fn file_cv(req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    Settings::start();

    let width = req.match_info().get("width").unwrap().parse::<i32>().unwrap();
    let height = req.match_info().get("height").unwrap().parse::<i32>().unwrap();
    let _smart: bool = req.match_info().get("smart").unwrap() == "smart/";
    let filename = req.match_info().get("filename").unwrap();

    if !security::is_valid_key(req.uri().to_string()) {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let img = get_image(filename).await;

    let original_size = img.size().unwrap();
    let new_size = opencv::core::Size { width, height };
    let new_size_with_aspect = calc::get_new_size_respecting_aspect_ratio(original_size, new_size);

    let mut resized_image = Mat::default();
    opencv::imgproc::resize(&img, &mut resized_image, new_size_with_aspect, 0.0, 0.0, opencv::imgproc::INTER_AREA).unwrap();

    let cropped_image = Mat::roi(&resized_image, opencv::core::Rect {
        x: ((new_size_with_aspect.width - width) / 2),
        y: ((new_size_with_aspect.height - height) / 2),
        width,
        height,
    }).unwrap();

    let mut out_vector: Vector<u8> = Vector::new();
    opencv::imgcodecs::imencode(".jpg", &cropped_image, &mut out_vector, &Vector::new()).expect("Encode image");

    Ok(HttpResponse::Ok()
        .append_header(("Content-Type", "image/jpeg"))
        .body(out_vector.to_vec()))
}
