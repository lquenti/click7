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
*
* ## REST API
* - [ ] Find out a Rust framework to use
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

use clap::Parser;

use crate::cli::Args;

fn main() -> Result<(), image::ImageError> {
    /* Parse CLI args */
    let args = Args::parse();

    /* init image generator */
    img_gen::init_lazy_static();
    if !img_gen::all_same_size() {
        panic!("Not all images have the same height!");
    }

    let new_img = img_gen::generate_image(args.number, args.digits, args.padding, args.border);
    let _ = new_img.save("./new.png");

    Ok(())
}
