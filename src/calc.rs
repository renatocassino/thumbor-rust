/**
 * Returns the aspect ratio of a rectangle.
 * If is less than 1 is portrait, if is greater than 1 is landscape.
 */
pub fn get_aspect_ratio(width: i32, height: i32) -> f32 {
    width as f32 / height as f32
}

/**
 * Returns the new size of a rectangle respecting the aspect ratio.
 * 
 * Calc of the new width: (original_width / original_height) * new_height
 * Calc of the new height: (original_height / original_width) * new_width
 */
pub fn get_new_size_respecting_aspect_ratio(original_size: opencv::core::Size, new_size: opencv::core::Size) -> opencv::core::Size {
    let original_ratio = get_aspect_ratio(original_size.width, original_size.height);
    let new_ratio = get_aspect_ratio(new_size.width, new_size.height);

    let is_wider = original_ratio < new_ratio;

    let (mut new_width, mut new_height) = (new_size.width, new_size.height);
    if is_wider {
        new_height = ((original_size.height as f32 / original_size.width as f32) * new_size.width as f32) as i32;
    } else {
        new_width = ((original_size.width as f32 / original_size.height as f32) * new_size.height as f32) as i32;
    }

    opencv::core::Size { width: new_width, height: new_height }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_aspect_ratio() {
        assert_eq!(get_aspect_ratio(100, 100), 1.0);
        assert_eq!(get_aspect_ratio(100, 50), 2.0);
        assert_eq!(get_aspect_ratio(50, 100), 0.5);
        assert_eq!(get_aspect_ratio(3456, 5184), 0.6666666666666666);
    }

    #[test]
    fn test_get_new_size_respecting_aspect_ratio_in_portrait() {
        let original_size = opencv::core::Size { width: 3456, height: 5184 };
        let new_size = opencv::core::Size { width: 670, height: 390 };
        let new_size_with_aspect = get_new_size_respecting_aspect_ratio(original_size, new_size);
        assert_eq!(new_size_with_aspect.width, 670);
        assert_eq!(new_size_with_aspect.height, 1005);
    }

    #[test]
    fn test_get_new_size_respecting_aspect_ratio_in_landscape() {
        let original_size = opencv::core::Size { width: 5184, height: 3456 };
        let new_size = opencv::core::Size { width: 390, height: 670 };
        let new_size_with_aspect = get_new_size_respecting_aspect_ratio(original_size, new_size);
        assert_eq!(new_size_with_aspect.width, 1005);
        assert_eq!(new_size_with_aspect.height, 670);
    }
}
