use image::{RgbaImage, ImageBuffer, ImageFormat};

const ZERO: &[u8] = include_bytes!("../assets/0.png");
const ONE: &[u8] = include_bytes!("../assets/1.png");
const TWO: &[u8] = include_bytes!("../assets/2.png");
const THREE: &[u8] = include_bytes!("../assets/3.png");
const FOUR: &[u8] = include_bytes!("../assets/4.png");
const FIVE: &[u8] = include_bytes!("../assets/5.png");
const SIX: &[u8] = include_bytes!("../assets/6.png");
const SEVEN: &[u8] = include_bytes!("../assets/7.png");
const EIGHT: &[u8] = include_bytes!("../assets/8.png");
const NINE: &[u8] = include_bytes!("../assets/9.png");

fn main() -> Result<(), image::ImageError> {

    let img1 = image::load_from_memory_with_format(TWO, ImageFormat::Png)?.into_rgba8();
    let img2 = image::load_from_memory_with_format(THREE, ImageFormat::Png)?.into_rgba8();

    let width = img1.width() + img2.width();
    let height = img1.height();

    let mut new_img: RgbaImage = ImageBuffer::new(width, height);

    image::imageops::overlay(&mut new_img, &img1, 0, 0);
    image::imageops::overlay(&mut new_img, &img2, img1.width() as i64, 0);

    let _ = new_img.save("./new.png");


    println!("Hello, world!");
    Ok(())
}
