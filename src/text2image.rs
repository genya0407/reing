use image;
use imageproc::drawing::draw_text_mut;
use image::{Rgb, RgbImage};
use image::ImageBuffer;
use rusttype::{Font, FontCollection, Scale};

#[test]
fn text2image_test() {
    use std::path::Path;
    let img = text2image(String::from("私はその人を常に先生と呼んでいた。\nだからここでもただ先生と書くだけで本名はuchiaけない。これは世間を憚かる遠慮というよりも、その方が私にとって自然だからである。\n私はその人の記憶を呼び起すごとに、すぐ「先生」といいたくなる。\n筆を執っても心持は同じ事である。\nよそよそしい頭文字などはとても使う気にならない。"));
    img.save(Path::new("test.png"));
}

pub fn text2image(text: String) -> image::RgbImage {
    let font = get_font();

    let full_font_size = 22.5;
    let half_font_size = full_font_size / 2.;
    let scale = Scale { x: full_font_size, y: full_font_size };

    let max_text_length = 44;
    let wrapped_text = wrap_text(text, max_text_length);

    let image_width = 600;
    let image_height = (50. + (wrapped_text.len() as f32) * full_font_size + 90.) as u32;
    let mut image = RgbImage::from_pixel(image_width, image_height, Rgb([255,255,255]));
    let x_offset = 50.;
    let y_offset = 50.;

    for (v_index, line) in wrapped_text.into_iter().enumerate() {
        let mut h_index = 0u32;
        for c in line.chars() {
            let x_position = (x_offset + (h_index as f32) * half_font_size) as u32;
            let y_position = (y_offset + (v_index as f32) * full_font_size) as u32;
            draw_text_mut(
                &mut image, Rgb([0, 0, 0]),
                x_position, y_position,
                scale, &font, &c.to_string()
            );

            h_index += char_len(c);
        }
    }

    return image;
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

fn wrap_text(text: String, max_length: u32 /* in ascii character */) -> Vec<String> {
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

fn get_font<'a>() -> Font<'a> {
    let font = Vec::from(include_bytes!("../assets/font/VL-Gothic-Regular.ttf") as &[u8]);
    FontCollection::from_bytes(font).unwrap().into_font().unwrap()
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
