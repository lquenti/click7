use std::collections::HashMap;

use image::{ImageBuffer, ImageError, ImageFormat, Rgba, RgbaImage, ImageEncoder};
use lazy_static::lazy_static;

const BLACK: Rgba<u8> = Rgba([0_u8, 0_u8, 0_u8, 255_u8]);
const GREY: Rgba<u8> = Rgba([128_u8, 128_u8, 128_u8, 255_u8]);

const DIGIT_BYTES: [&[u8]; 10] = [
    include_bytes!("../assets/0.png"),
    include_bytes!("../assets/1.png"),
    include_bytes!("../assets/2.png"),
    include_bytes!("../assets/3.png"),
    include_bytes!("../assets/4.png"),
    include_bytes!("../assets/5.png"),
    include_bytes!("../assets/6.png"),
    include_bytes!("../assets/7.png"),
    include_bytes!("../assets/8.png"),
    include_bytes!("../assets/9.png"),
];

lazy_static! {
    static ref DIGIT_IMAGES: HashMap<u8, RgbaImage> = {
        let mut map = HashMap::new();
        for (i, &bytes) in DIGIT_BYTES.iter().enumerate() {
            map.insert(i as u8, load_png(bytes).unwrap());
        }
        map
    };
}

fn load_png(bytes: &[u8]) -> Result<RgbaImage, ImageError> {
    let img = image::load_from_memory_with_format(bytes, ImageFormat::Png)?;
    Ok(img.into_rgba8())
}

pub fn init_lazy_static() {
    // ensure that all are loaded
    for (_, image) in DIGIT_IMAGES.iter() {
        let _width = image.width();
    }
}

pub fn all_same_size() -> bool {
    let mut heights = DIGIT_IMAGES.iter().map(|(_, image)| image.height());

    let first = heights.next().unwrap();
    heights.all(|height| height == first)
}

pub fn generate_image(n: u32, max_digits: u8, padding: u32, border: u32) -> RgbaImage {
    let max_number: u64 = 10_u64.pow(max_digits.into()) - 1;

    // convert number into digits
    let mut digits = vec![];
    if (n as u64) > max_number {
        // If too large, just put nines there
        println!("Overflow: {} > {}", n, max_number);
        digits = vec![9_u8; max_digits as usize];
    } else {
        // cut off last number, add
        let mut curr = n;
        while curr != 0 {
            let last_digit = (curr % 10) as u8;
            digits.push(last_digit);
            curr /= 10;
        }
        digits.reverse();

        // Fill with zeros on the left
        let missing_zeros = (max_digits as i32) - (digits.len() as i32);
        if missing_zeros > 0 {
            let mut zerovec = vec![0; missing_zeros as usize];
            zerovec.extend(digits.iter());
            digits = zerovec;
        }
    }

    // Convert digits to images
    let digits: Vec<&RgbaImage> = digits
        .into_iter()
        .map(|n| DIGIT_IMAGES.get(&n).unwrap())
        .collect();

    /*
     * How it works:
     * - First a gray background for the border
     * - Then overlay a smaller black one for the padding
     * - Then, with padding in mind, put the digits on there
     */
    let black_background_height = digits[0].height() + 2 * padding;
    let digit_width: u32 = digits.iter().map(|image| image.width()).sum();
    let padding_width: u32 = (digits.len() as u32 + 1) * padding;
    let black_background_width = digit_width + padding_width;

    let grey_background_height = black_background_height + 2 * border;
    let grey_background_width = black_background_width + 2 * border;

    let mut grey_background: RgbaImage =
        ImageBuffer::from_pixel(grey_background_width, grey_background_height, GREY);
    let black_blackground: RgbaImage =
        ImageBuffer::from_pixel(black_background_width, black_background_height, BLACK);

    // Overlay the black packground for the number padding
    image::imageops::overlay(
        &mut grey_background,
        &black_blackground,
        border as i64,
        border as i64,
    );

    // add the digits
    let mut offset: u32 = border + padding;
    for digit in digits {
        image::imageops::overlay(
            &mut grey_background,
            digit,
            offset as i64,
            (border + padding) as i64,
        );
        offset += digit.width() + padding;
    }

    grey_background
}

pub fn save_to_png(img: RgbaImage) -> Result<Vec<u8>, ImageError> {
    let mut bytes: Vec<u8> = vec![];

    image::codecs::png::PngEncoder::new(&mut bytes).write_image(
        &img,
        img.width(),
        img.height(),
        image::ColorType::Rgba8,
    )?;
    Ok(bytes)
}
