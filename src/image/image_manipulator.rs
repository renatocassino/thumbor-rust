use opencv::core::{Mat, Size};
use opencv::prelude::MatTraitConstManual; // to get method `.size()` must have this use

use crate::{url_props::{UrlProps}, service::image::ImageWithType, calc};

pub mod direction {
    pub const HORIZONTAL: i32 = 1;
    pub const VERTICAL: i32 = 0;
}

pub fn flip(image: &Mat, direction: i32) -> Mat {
    let mut flipped_image = Mat::default();
    opencv::core::flip(&image, &mut flipped_image, direction).unwrap();
    flipped_image
}

pub fn flip_horizontal(image: &Mat) -> Mat {
    flip(image, direction::HORIZONTAL)
}

pub fn flip_vertical(image: &Mat) -> Mat {
    flip(image, direction::VERTICAL)
}

pub fn resize(img: &ImageWithType, url_props: &UrlProps) -> Mat {
    let original_size = img.image.size().unwrap();
    let new_aspect = calc::get_new_size_respecting_aspect_ratio(
        original_size,
        Size {
            width: url_props.width,
            height: url_props.height,
        }
    );

    let mut resized_image = Mat::default();
    opencv::imgproc::resize(
        &img.image,
        &mut resized_image,
        new_aspect,
        0.0,
        0.0,
        opencv::imgproc::INTER_AREA
    ).unwrap();

    resized_image
}

pub fn crop(resized_image: &Mat, url_props: &UrlProps, original_size: opencv::core::Size) -> Mat {
    let new_aspect = calc::get_new_size_respecting_aspect_ratio(
        original_size,
        Size {
            width: url_props.width,
            height: url_props.height,
        }
    );

    let halign = url_props.alignment.halign.as_str();
    let x = match halign {
        "left" => 0,
        "center" => (new_aspect.width - url_props.width) / 2,
        "right" => new_aspect.width - url_props.width,
        &_ => panic!("Invalid valign: {}", halign)
    };

    let valigh = url_props.alignment.valign.as_str();
    let y = match valigh {
        "top" => 0,
        "middle" => (new_aspect.height - url_props.height) / 2,
        "bottom" => new_aspect.height - url_props.height,
        &_ => panic!("Invalid valign: {}", valigh)
    };

    Mat::roi(resized_image, opencv::core::Rect {
        x,
        y,
        width: url_props.width,
        height: url_props.height,
    }).unwrap()
}
