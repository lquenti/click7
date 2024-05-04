/*
* # TODOs:
*
* Too fast moving to bother with GH Issues rn
*
* ## General
* - [ ] Add logging with tracing
* - [ ] Proper error management with anyhow+thiserror
* - [ ] Get cargo audit and cargo outdated running
* - [ ] Write docs
*
*
* ## Image gen
* - [ ] Make the counter smaller
*   - [ ] decrease img size
*   - [ ] decrease padding
*   - [ ] decrease border size
* - [ ] Increase contrast (#1)
*
* ## REST API
* - [x] Find out a Rust framework to use
* - [x] Get a health check API endpoint returning 200
* - [x] Find out how to return an Image
* - [x] Create a debug endpoint where one can query any number and get it
*  - [x] properly refactor it
* - [x] add a `/` that explains the project in HTML
*   - [ ] Add a more beautiful and descriptive website
*     - [ ] Add cfg details to it
* - [x] Create the actual API after the DB is working
*
* ## KV Store
* - [x] Add to project
* - [x] Init DB
* - [x] Add increment value
* - [ ] Make sure it is thread-safe
*   - [ ] describe current state on README
 */

mod cli;
mod img_gen;
mod routes;
mod store;

use std::sync::Arc;

use crate::cli::Args;
use axum::{routing, Extension, Router};
use clap::Parser;

#[tokio::main]
async fn main() {
    /* Parse CLI args */
    let args = Args::parse();

    /* init image generator */
    img_gen::init_lazy_static();
    if !img_gen::all_same_size() {
        panic!("Not all images have the same height!");
    }

    /* load database */
    /* panic intended, if the db doenst load we are fucked */
    let db = store::load_db_if_exist(&args.database).unwrap();

    let db_arc = Arc::new(db);

    /* Define routes */
    let app_with_state = Router::new()
        .route("/generate/:number", routing::get(routes::generate))
        .route("/cnt/:id", routing::get(routes::counter))
        .layer(Extension(args.clone()))
        .layer(Extension(db_arc));

    let app_without_state = Router::new()
        .route("/", routing::get(routes::index))
        .route("/health_check", routing::get(routes::health_check));

    let app = app_with_state.merge(app_without_state);

    /* start server */
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port)).await.unwrap();
    println!("Running at port {}", args.port);
    axum::serve(listener, app).await.unwrap();
}
