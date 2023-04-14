use opencv::{core::Mat};
use reqwest::{get, header::CONTENT_TYPE};
use mime_guess::MimeGuess;

pub struct ImageWithType {
    pub mime_type: String,
    pub image: Mat,
}

async fn load_image_from_url(filename: &mut String) -> Result<ImageWithType, Box<dyn std::error::Error>> {
    let resp = get(filename.to_string()).await?;    

    if resp.status().is_success() {
        let headers = resp.headers().clone();
        let content_type = headers
            .get(CONTENT_TYPE)
            .and_then(|header| header.to_str().ok())
            .unwrap_or("type/jpeg");

        let image_data = resp.bytes().await?.clone().to_vec();
        let mat = Mat::from_slice(&image_data)?;
        let img = opencv::imgcodecs::imdecode(&mat, opencv::imgcodecs::IMREAD_COLOR)?;

        return Ok(ImageWithType { image: img, mime_type: content_type.to_string() })
    }
    Ok(ImageWithType { image: Mat::default(), mime_type: "type/jpeg".to_string() })
}

fn load_image_from_file(filename: &str) -> Result<ImageWithType, opencv::Error> {
    let path = format!("./src/images/{}", filename);
    let img = opencv::imgcodecs::imread(&path, opencv::imgcodecs::IMREAD_COLOR)?;
    let mime = MimeGuess::from_path(path).first_or_octet_stream();

    println!("Mime: {}", mime);
    Ok(ImageWithType { image: img, mime_type: mime.to_string() })
}

pub async fn get_image(filename: &str) -> ImageWithType {
    if filename.starts_with("http://") || filename.starts_with("https://") {
        let img = load_image_from_url(&mut filename.to_string()).await.unwrap();
        return img;
    }

    load_image_from_file(filename).unwrap()
}