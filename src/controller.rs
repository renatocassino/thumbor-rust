use actix_web::http::header;
use opencv::prelude::MatTraitConstManual; // to get method `.size()` must have this use
use opencv::{core::{Mat}};
use opencv::core::Vector;
use crate::image::image_manipulator;
use crate::url_props::UrlPropsController;
use crate::{security, url_props};
use crate::settings::{Settings};
use crate::{calc, service::image::get_image};
use actix_web::{get, web, Result, HttpRequest, HttpResponse};

#[get("/{key}/{width:-?\\d+}x{height:-?\\d+}{smart:(/smart)?}{halign:(/(left|right|center))?}{valign:(/(top|middle|bottom))?}/{filename:.*}")]
pub async fn file_cv(req: HttpRequest, path: web::Path<UrlPropsController>) -> Result<HttpResponse, actix_web::Error> {
    Settings::start();

    let url_props = url_props::build_url_props(path.into_inner());
    if !security::is_valid_key(req.uri().to_string()) {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let img = get_image(&url_props.filename).await;

    let original_size = img.image.size().unwrap();
    let new_size = opencv::core::Size { width: url_props.width, height: url_props.height };
    let new_aspect = calc::get_new_size_respecting_aspect_ratio(original_size, new_size);

    let mut final_image: Mat;
    let mut resized_image = Mat::default();
    opencv::imgproc::resize(
        &img.image,
        &mut resized_image,
        new_aspect,
        0.0,
        0.0,
        opencv::imgproc::INTER_AREA
    ).unwrap();

    final_image = Mat::roi(&resized_image, opencv::core::Rect {
        x: ((new_aspect.width - new_size.width) / 2),
        y: ((new_aspect.height - new_size.height) / 2),
        width: new_size.width,
        height: new_size.height,
    }).unwrap();

    if url_props.flip.horizontal {
        final_image = image_manipulator::flip_horizontal(&final_image);
    }

    if url_props.flip.vertical {
        final_image = image_manipulator::flip_vertical(&final_image);
    }

    let mut out_vector: Vector<u8> = Vector::new();
    opencv::imgcodecs::imencode(".jpg", &final_image, &mut out_vector, &Vector::new()).expect("Encode image");

    Ok(HttpResponse::Ok()
        .content_type(img.mime_type.as_str())
        .insert_header(header::CacheControl(vec![header::CacheDirective::MaxAge(3600)]))
        .body(out_vector.to_vec()))
}
