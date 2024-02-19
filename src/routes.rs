use crate::img_gen::generate_image;
use image::ImageEncoder;

use axum::{
    extract::Path,
    http::{header::CONTENT_TYPE, Response, StatusCode},
    response::IntoResponse,
};

pub async fn health_check() -> impl IntoResponse {
    "Ok"
}

pub async fn generate(Path(number): Path<u32>) -> impl IntoResponse {
    let img = generate_image(number, 7, 20, 20);
    let mut bytes: Vec<u8> = vec![];

    let res = image::codecs::png::PngEncoder::new(&mut bytes).write_image(
        &img,
        img.width(),
        img.height(),
        image::ColorType::Rgba8,
    );
    let status_code = match res {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };

    let response = Response::builder()
        .status(status_code)
        .header(CONTENT_TYPE, "image/png")
        .body(())
        .unwrap();

    (response, bytes)
}
