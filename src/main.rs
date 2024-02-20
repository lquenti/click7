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
* - [x] add a `/` that explains the project in HTML
*   - [ ] Add a more beautiful and descriptive website
* - [ ] Create the actual API after the DB is working
*
* ## kv store
* - [x] design kv store
*   - Two different possibilites:
*     - Memory only: big HashMap, (de-)serialized on close.
*       - Support an interval where it will be written back periodically
*         - TODO figure out how we can do it without locking
*     - persistent:
*       - redb for persistence
*       - write-back caching with LRU cache
*         - size configurable
*       - force write-back on close
* - [ ] Write on README
* - [ ] Create CLI arguments
*   - [ ] decide on a default
* - [ ] Write trait
* - [ ] impl mem
*   - [ ] TODO decide what to do when its full
* - [ ] impl i/o
*   - [ ] 
 */

mod cli;
mod img_gen;
mod kv_store;
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
        .route("/", routing::get(routes::index))
        .route("/health_check", routing::get(routes::health_check))
        .route("/generate/:number", routing::get(routes::generate));

    /* start server */
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
