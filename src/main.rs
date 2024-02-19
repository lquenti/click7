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
*   - [ ] Publish that one on crates.io already
*
* ## REST API
* - [x] Find out a Rust framework to use
* - [x] Get a health check API endpoint returning 200
* - [x] Find out how to return an Image
* - [x] Create a debug endpoint where one can query any number and get it
*  - [x] properly refactor it
* - [ ] add a `/` that explains the project in HTML
* - [ ] Create the actual API after the DB is working
*
* ## sqlite
* todo
 */

mod cli;
mod img_gen;
mod routes;

use crate::cli::Args;
use axum::{routing, Router};
use clap::Parser;

#[tokio::main]
async fn main() {
    /* Parse CLI args */
    let _args = Args::parse();

    /* init image generator */
    img_gen::init_lazy_static();
    if !img_gen::all_same_size() {
        panic!("Not all images have the same height!");
    }

    /* Define routes */
    let app = Router::new()
        .route("/health_check", routing::get(routes::health_check))
        .route("/generate/:number", routing::get(routes::generate));

    /* start server */
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
