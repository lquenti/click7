use std::sync::Arc;

use crate::{cli::Args, img_gen::{generate_image, save_to_png}, store};
use image::ImageEncoder;

const INDEX_HTML: &str = include_str!("../assets/index.html");

use axum::{
    extract::Path,
    http::{header::CONTENT_TYPE, Response, StatusCode},
    response::{Html, IntoResponse}, Extension,
};
use redb::Database;

pub async fn index() -> impl IntoResponse {
    Html(INDEX_HTML)
}

pub async fn health_check() -> impl IntoResponse {
    "Ok"
}

fn create_img_response(args: &Args, number: u32) -> impl IntoResponse {
    let img = generate_image(number, args.digits, args.padding, args.border);
    let res = save_to_png(img);
    match res {
        Ok(bytes) => {
            let response = Response::builder()
                .status(StatusCode::OK)
                .header(CONTENT_TYPE, "image/png")
                .body(())
                .unwrap();
            (response, bytes)
        },
        Err(_) => {
            let response = Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(())
                .unwrap();
            (response, Vec::new())
        }
    }
}

pub async fn generate(Extension(args): Extension<Args>, Path(number): Path<u32>) -> impl IntoResponse {
    create_img_response(&args, number)
}

pub async fn counter(
    Extension(args): Extension<Args>, 
    Extension(db): Extension<Arc<Database>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let number = store::read_and_increment(&db, &id).unwrap();
    create_img_response(&args, number)
}

