use actix_web::HttpRequest;
use actix_web::http::header;
use opencv::prelude::MatTraitConstManual; // to get method `.size()` must have this use
use opencv::{core::{Mat}};
use opencv::core::Vector;
use crate::image::image_manipulator;
use crate::security;
use crate::settings::{Settings};
use crate::{calc, service::image::get_image};
use actix_web::{get, HttpResponse};

#[get("/{key}/{width:-?\\d+}x{height:-?\\d+}/{smart:(smart/)?}{filename:.*}")]
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

    let original_size = img.image.size().unwrap();
    let new_size = opencv::core::Size { width: i32::abs(width), height: i32::abs(height) };
    let new_size_with_aspect = calc::get_new_size_respecting_aspect_ratio(original_size, new_size);

    let mut final_image: Mat;
    let mut resized_image = Mat::default();
    opencv::imgproc::resize(&img.image, &mut resized_image, new_size_with_aspect, 0.0, 0.0, opencv::imgproc::INTER_AREA).unwrap();

    let cropped_image = Mat::roi(&resized_image, opencv::core::Rect {
        x: ((new_size_with_aspect.width - new_size.width) / 2),
        y: ((new_size_with_aspect.height - new_size.height) / 2),
        width: new_size.width,
        height: new_size.height,
    }).unwrap();

    final_image = cropped_image;

    if width < 0 {
        final_image = image_manipulator::flip_horizontal(&final_image);
    }

    if height < 0 {
        final_image = image_manipulator::flip_vertical(&final_image);
    }

    let mut out_vector: Vector<u8> = Vector::new();
    opencv::imgcodecs::imencode(".jpg", &final_image, &mut out_vector, &Vector::new()).expect("Encode image");

    Ok(HttpResponse::Ok()
        .content_type(img.mime_type.as_str())
        .insert_header(header::CacheControl(vec![header::CacheDirective::MaxAge(3600)]))
        .body(out_vector.to_vec()))
}
