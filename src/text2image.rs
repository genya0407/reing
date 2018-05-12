use image;
use imageproc::drawing::draw_text_mut;
use image::{Rgb, RgbImage};
use image::ImageBuffer;
use rusttype::{Font, FontCollection, Scale};

pub fn text2image(text: String) {// -> image::RgbImage {
    let font = get_font();

    let half_font_size = 22.5;
    let full_font_size = half_font_size * 2.;
    let scale = Scale { x: full_font_size, y: full_font_size };

    let wrapped_text = wrap_text(text, 40);
}

fn get_font<'a>() -> Font<'a> {
    let font = Vec::from(include_bytes!("../assets/font/VL-Gothic-Regular.ttf") as &[u8]);
    FontCollection::from_bytes(font).unwrap().into_font().unwrap()
}

#[test]
fn test_wrap_text() {
    assert_eq!(
        wrap_text(String::from("1234567890"), 5),
        vec![String::from("12345"), String::from("67890")]
    );
    assert_eq!(
        wrap_text(String::from("あいうえおかきくけこ"), 10),
        vec![String::from("あいうえお"), String::from("かきくけこ")]
    );
    assert_eq!(
        wrap_text(String::from("あいうえおかきくけこ"), 9),
        vec![String::from("あいうえお"), String::from("かきくけこ")]
    );
    assert_eq!(
        wrap_text(String::from("あいうえおaかきくけこ"), 9),
        vec![String::from("あいうえお"), String::from("aかきくけ"), String::from("こ")]
    );
    assert_eq!(
        wrap_text(String::from("あい\nうえおかきく\nけこ"), 10),
        vec![String::from("あい"), String::from("うえおかき"), String::from("く"), String::from("けこ")]
    );
}

pub fn wrap_text(text: String, max_length: u32 /* in ascii character */) -> Vec<String> {
    let mut wrapped_lines = vec![];
    for original_line in text.lines() {
        let mut working_line: Vec<char> = vec![];
        let mut len = 0; // in ascii character
        for c in original_line.chars() {
            if len >= max_length {
                len = 0;
                wrapped_lines.push(working_line.clone().into_iter().collect::<String>());
                working_line = vec![];
            }

            len += char_len(c);
            working_line.push(c)
        }
        if !working_line.is_empty() {
            wrapped_lines.push(working_line.into_iter().collect::<String>())
        }
    }    
    return wrapped_lines;
}

pub fn _text2image(text: String) -> image::RgbImage {
    let font = Vec::from(include_bytes!("../assets/font/VL-Gothic-Regular.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();

    let x_font_size = 5f32;
    let y_font_size = 10f32;
    let scale = Scale { x: x_font_size * 2., y: y_font_size };

    let longest_line_length = text.lines().map(font_ascii_count).max().unwrap() / 2;
    let line_count = text.lines().count();

    let image_width = ((x_font_size * 2.) * longest_line_length as f32) as u32;
    let image_height = (line_count as f32 * y_font_size) as u32;
    let mut image = RgbImage::from_pixel(image_width, image_height, Rgb([255,255,255]));

    for (v_index, line) in text.lines().enumerate() {
        let mut h_index = 0u32;
        for c in line.chars() {
            draw_text_mut(
                &mut image,
                Rgb([0u8, 0u8, 0u8]),
                (h_index as f32 * x_font_size) as u32, (v_index as f32 * y_font_size) as u32,
                scale, &font, &c.to_string()
            );

            if c.is_ascii() {
                h_index += 1;
            } else {
                h_index += 2;
            }
        }
    }

    return image;
}

fn char_len(c: char) -> u32 { // in ascii character
    if c.is_ascii() {
        1
    } else {
        2
    }
}

fn font_ascii_count(text: &str) -> u32 { // in ascii character
    text.chars().map(|c| char_len(c)).sum::<u32>()
}
