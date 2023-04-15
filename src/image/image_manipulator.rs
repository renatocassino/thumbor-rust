use opencv::core::Mat;

pub mod direction {
    pub const HORIZONTAL: i32 = 1;
    pub const VERTICAL: i32 = 0;
}

pub fn flip(image: &Mat, direction: i32) -> Mat {
    let mut flipped_image = Mat::default();
    opencv::core::flip(&image, &mut flipped_image, direction).unwrap();
    return flipped_image;
}

pub fn flip_horizontal(image: &Mat) -> Mat {
    return flip(image, direction::HORIZONTAL);
}

pub fn flip_vertical(image: &Mat) -> Mat {
    return flip(image, direction::VERTICAL);
}