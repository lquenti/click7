/*
* TODOs:
* - [ ] Add Asserts at start
*   - [ ] Assert that all numbers have the same height
* - [ ] Better image gen
*   - [ ] more padding
*   - [ ] grey border
* - [ ] Add clippy
*   - [ ] Concigure max number of digits
* - [ ] Add logging
*   - [ ] ALso replace all prints
* - [ ] CI
* - [ ] Proper error management with anyhow+thiserror
 */

mod img_gen;


fn main() -> Result<(), image::ImageError> {
    img_gen::init_lazy_static();
    if !img_gen::all_same_size() {
        panic!("Not all images have the same height!");
    }


    let num: u64 = std::env::args().nth(1).unwrap().parse().unwrap();
    let new_img = img_gen::generate_image(num, 6);
    let _ = new_img.save("./new.png");

    Ok(())
}
