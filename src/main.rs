use image::{DynamicImage, GenericImageView, GenericImage, Rgba};

fn encode_message(img: &DynamicImage, message: &str) -> DynamicImage {
    let mut img = img.clone();
    let bytes = message.as_bytes();
    let mut byte_iter = bytes.iter();

    let mut pixel_count = 0;

    'outer: for y in 0..img.height() {
        for x in 0..img.width() {
            if let Some(&byte) = byte_iter.next() {
                for bit in 0..8 {
                    let mut rgba = img.get_pixel(x, y).0;
                    rgba[0] = (rgba[0] & !1) | ((byte >> bit) & 1); // Modify LSB of Red channel
                    img.put_pixel(x, y, Rgba(rgba));

                    pixel_count += 1;
                    if pixel_count % 8 == 0 {
                        continue 'outer;
                    }
                }
            } else {
                break 'outer;
            }
        }
    }

    println!("Encoded {} characters in {} pixels.", message.len(), pixel_count);
    img
}


fn decode_message(img: &DynamicImage, length: usize) -> String {
    let mut message_bytes = Vec::new();
    let mut current_byte = 0u8;
    let mut bit_count = 0;

    let mut pixel_count = 0;

    'outer: for y in 0..img.height() {
        for x in 0..img.width() {
            if pixel_count >= 8 * length {
                break 'outer;
            }

            let pixel = img.get_pixel(x, y).0;
            let bit = (pixel[0] & 1) << bit_count;
            current_byte |= bit;
            bit_count += 1;

            if bit_count == 8 {
                message_bytes.push(current_byte);
                println!("Decoded byte: {}, corresponding char: {}", current_byte, current_byte as char);
                current_byte = 0;
                bit_count = 0;
            }

            pixel_count += 1;
        }
    }

    String::from_utf8_lossy(&message_bytes).to_string()
}

fn main() {
    let img = image::open("rust-logo.png").unwrap();
    let secret_message = "hey how long can I encode ? :)";
    
    let encoded_img = encode_message(&img, secret_message);
    encoded_img.save("rust-logo-encoded.png").unwrap();

    let decoded_message = decode_message(&encoded_img, secret_message.len());
    println!("Decoded Message: {}", decoded_message);
}
