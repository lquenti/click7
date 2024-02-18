/*
* TODOs:
* - [x] Add Asserts at start
*   - [x] Assert that all numbers have the same height
* - [ ] Better image gen
*   - [x] more padding
*   - [ ] grey border
* - [x] Add clap
*   - [x] Concigure max number of digits
* - [ ] Add logging
*   - [ ] ALso replace all prints
* - [x] CI
* - [ ] Proper error management with anyhow+thiserror
* - [ ] Make the counter smaller
*   - [ ] decrease img size
*   - [ ] decrease padding
*   - [ ] decrease border size
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

    let new_img = img_gen::generate_image(args.number, args.digits, args.padding);
    let _ = new_img.save("./new.png");

    Ok(())
}
