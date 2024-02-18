/*
* TODOs:
* - [x] Add Asserts at start
*   - [x] Assert that all numbers have the same height
* - [ ] Better image gen
*   - [ ] more padding
*   - [ ] grey border
* - [ ] Add clap
*   - [ ] Concigure max number of digits
* - [ ] Add logging
*   - [ ] ALso replace all prints
* - [ ] CI
* - [ ] Proper error management with anyhow+thiserror
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

    let new_img = img_gen::generate_image(args.number, args.digits);
    let _ = new_img.save("./new.png");

    Ok(())
}
