use opencv::core::Mat;

async fn load_image_from_url(filename: &mut String) -> Result<Mat, Box<dyn std::error::Error>> {
    if filename.starts_with("http://") || filename.starts_with("https://") {
        let resp = reqwest::get(filename.to_string()).await?;
        if resp.status().is_success() {
            let image_data = resp.bytes().await?.to_vec();
            let mut mat = Mat::from_slice(&image_data)?;
            let img = opencv::imgcodecs::imdecode(&mut mat, opencv::imgcodecs::IMREAD_COLOR)?;
            return Ok(img);
        }
    }
    Ok(Mat::default())
}

fn load_image_from_file(filename: &str) -> Result<Mat, opencv::Error> {
    let path = format!("./src/images/{}", filename);
    let img = opencv::imgcodecs::imread(&path, opencv::imgcodecs::IMREAD_COLOR)?;
    Ok(img)
}

pub async fn get_image(filename: &str)-> Mat {
    if filename.starts_with("http://") || filename.starts_with("https://") {
        let img = load_image_from_url(&mut filename.to_string()).await.unwrap();
        return img;
    }

    let img = load_image_from_file(filename).unwrap();
    return img;
}