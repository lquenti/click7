use std::collections::HashMap;

use image::{RgbaImage, ImageBuffer, ImageFormat, ImageError};
use lazy_static::lazy_static;


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
    let mut heights = DIGIT_IMAGES.iter()
        .map(|(_, image)| image.height());

    let first = heights.next().unwrap();
    heights.all(|height| height == first)
}

pub fn generate_image(n: u64, max_digits: u8) -> RgbaImage {
    let max_number: u64 = (10 as u64).pow(max_digits.into()) - 1;

    // convert number into digits
    let mut digits = vec![];
    if n > max_number {
        // If too large, just put nines there
        println!("Overflow: {} > {}", n, max_number);
        for _ in 0..max_digits {
            digits.push(9 as u8);
        }
    } else {
        // cut off last number, add
        let mut digits_without_prefix = vec![];
        let mut curr = n;
        while curr != 0 {
            let last_digit = (curr % 10) as u8;
            digits_without_prefix.push(last_digit);
            curr /= 10;
        }
        digits_without_prefix.reverse();
        
        // Fill with zeros on the left
        let missing_zeros = (max_digits as i32) - (digits_without_prefix.len() as i32);
        if missing_zeros > 0 {
            let mut zerovec = vec![0; missing_zeros as usize];
            // TODO: HOW DO I HERE GET DIGITS AS ZEROVEC + DIGITS AS ONE VECTOR
            // like
            // digits = zerovec + digits
            zerovec.extend(digits_without_prefix.iter());
            digits = zerovec;
        }
    }

    // Convert digits to images
    let digits: Vec<&RgbaImage> = digits.into_iter()
        .map(|n| DIGIT_IMAGES.get(&n).unwrap())
        .collect();

    // Create buffer
    let height: u32 = digits[0].height();
    let total_width: u32 = digits.iter()
        .map(|image| image.width())
        .sum();
    let mut image: RgbaImage = ImageBuffer::new(total_width, height);

    // add the digits
    let mut offset: u32 = 0;
    for digit in digits {
        image::imageops::overlay(&mut image, digit, offset as i64, 0);
        offset += digit.width();
    }

    image
}
