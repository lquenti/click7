/*
* # TODOs:
*
* Too fast moving to bother with GH Issues rn
*
* ## General
* - [ ] Add logging with tracing
*   - [ ] ALso replace all prints
* - [ ] Proper error management with anyhow+thiserror
* - [ ] Get cargo audit and cargo outdated running
*
*
* ## Image gen
* - [ ] Make the counter smaller
*   - [ ] decrease img size
*   - [ ] decrease padding
*   - [ ] decrease border size
* - [ ] Add a standalone binary that just creates the img
*
* ## REST API
* - [x] Find out a Rust framework to use
* - [x] Get a health check API endpoint returning 200
* - [ ] Find out how to return an Image
* - [ ] Create a debug endpoint where one can query any number and get it
* - [ ] Create a `/counter/<ID>` endpoint that always returns the same number
*
* ## sqlite
* todo
 */

mod cli;
mod img_gen;

use axum::{
    http::{header::CONTENT_TYPE, Response, StatusCode},
    routing, Router,
};
use clap::Parser;
use image::ImageEncoder;
use img_gen::generate_image;

use crate::cli::Args;

async fn debug() -> (Response<()>, Vec<u8>) {
    let img = generate_image(1337, 7, 20, 20);
    let mut bytes: Vec<u8> = vec![];

    image::codecs::png::PngEncoder::new(&mut bytes)
        .write_image(&img, img.width(), img.height(), image::ColorType::Rgba8)
        .unwrap();
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "image/png")
        .body(())
        .unwrap();
    (response, bytes)
}

#[tokio::main]
async fn main() {
    /* Parse CLI args */
    let args = Args::parse();

    /* init image generator */
    img_gen::init_lazy_static();
    if !img_gen::all_same_size() {
        panic!("Not all images have the same height!");
    }

    /* Define routes */
    let app = Router::new()
        .route("/health_check", routing::get(|| async { "Ok" }))
        .route("/debug", routing::get(debug));

    /* start server */
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
