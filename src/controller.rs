use actix_web::http::header;
use opencv::prelude::MatTraitConstManual; // to get method `.size()` must have this use
use opencv::{core::{Mat}};
use opencv::core::Vector;
use crate::calc::{new_width_when_respect_aspect_ration, new_height_when_respect_aspect_ration};
use crate::image::image_manipulator;
use crate::url_props::UrlPropsController;
use crate::{security, url_props};
use crate::settings::{Settings};
use crate::{service::image::get_image};
use actix_web::{get, web, Result, HttpRequest, HttpResponse};

#[get("/{key}/{width:-?\\d+}x{height:-?\\d+}{smart:(/smart)?}{halign:(/(left|right|center))?}{valign:(/(top|middle|bottom))?}/{filename:.*}")]
pub async fn file_cv(req: HttpRequest, path: web::Path<UrlPropsController>) -> Result<HttpResponse, actix_web::Error> {
    Settings::start();

    let mut url_props = url_props::build_url_props(path.into_inner());
    if !security::is_valid_key(req.uri().to_string()) {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let img = get_image(&url_props.filename).await;
    let original_size = img.image.size().unwrap();

    if url_props.width == 0 {
        url_props.width = new_width_when_respect_aspect_ration(original_size.width, original_size.height, url_props.height);
    }

    if url_props.height == 0 {
        url_props.height = new_height_when_respect_aspect_ration(original_size.width, original_size.height, url_props.width);
    }

    let mut final_image: Mat;
    let resized_image = image_manipulator::resize(&img, &url_props);
    final_image = image_manipulator::crop(&resized_image, &url_props, original_size);

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
