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
* - [ ] Get a health check API endpoint returning 200
* - [ ] Find out how to return an Image
* - [ ] Create a debug endpoint where one can query any number and get it
* - [ ] Create a `/counter/<ID>` endpoint that always returns the same number
* 
* ## sqlite
* todo
 */

mod cli;
mod img_gen;

use axum::{Router, routing};
use clap::Parser;

use crate::cli::Args;

#[tokio::main]
async fn main() {
    /* Parse CLI args */
    let args = Args::parse();

    /* init image generator */
    img_gen::init_lazy_static();
    if !img_gen::all_same_size() {
        panic!("Not all images have the same height!");
    }

    /* Create route */
    let app = Router::new()
        .route("/health_check", routing::get(|| async {"Ok"}));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
