use image;
use imageproc::rect::Rect;
use imageproc::drawing::{draw_text_mut, draw_filled_rect_mut};
use image::{Rgb, RgbImage};
use rusttype::{Font, FontCollection, Scale};

pub fn text2image(text: String) -> image::RgbImage {
    let font = get_vl_gothic();

    let full_font_size = 22.5;
    let half_font_size = full_font_size / 2.;
    let scale = Scale { x: full_font_size, y: full_font_size };

    let max_text_length = 44;
    let wrapped_text = wrap_text(text, max_text_length);

    let text_height = (wrapped_text.len() as f32) * full_font_size;
    let minimum_image_height = 344;
    let image_width = 600;
    let image_height = (50. + text_height + 90.) as u32;
    let (image_height, text_start_at_x, text_start_at_y) = if image_height >= minimum_image_height {
        (image_height, 50., 50.)
    } else {
        (minimum_image_height, 50., 50. + ((minimum_image_height - 140 - text_height as u32)/2) as f32)
    };
    let mut image = RgbImage::from_pixel(image_width, image_height, Rgb([0x2c, 0x36, 0x5d]));
    let rect = Rect::at(20, 20).of_size(image_width - 40, image_height - 80);
    draw_filled_rect_mut(&mut image, rect, Rgb([255, 255, 255]));

    for (v_index, line) in wrapped_text.into_iter().enumerate() {
        let mut h_index = 0u32;
        for c in line.chars() {
            let x_position = (text_start_at_x + (h_index as f32) * half_font_size) as u32;
            let y_position = (text_start_at_y + (v_index as f32) * full_font_size) as u32;
            draw_text_mut(
                &mut image, Rgb([0, 0, 0]),
                x_position, y_position,
                scale, &font, &c.to_string()
            );

            h_index += char_len(c);
        }
    }

    let logo_scale = Scale { x: 25., y: 25. };
    let logo_x_position = 30;
    let logo_y_position = image_height - 45;
    let tanuki_font = get_tanuki_magic();
    draw_text_mut(
        &mut image, Rgb([255, 255, 255]),
        logo_x_position, logo_y_position,
        logo_scale, &tanuki_font, "Reing"
    );

    return image;
}

#[test]
fn text2image_test() {
    use std::path::Path;
    let original_text = vec![
        "私はその人を常に先生と呼んでいた。",
        "だからここでもただ先生と書くだけで本名は打ち明けない。",
        "これは世間を憚かる遠慮というよりも、その方が私にとって自然だからである。",
        "私はその人の記憶を呼び起すごとに、すぐ「先生」といいたくなる。",
        "筆を執っても心持は同じ事である。",
        "よそよそしい頭文字などは",
        "とても使う気にならない。",
        "私が先生と知り合いになったのは鎌倉である。",
        "その時私はまだ若々しい書生であった。",
        "暑中休暇を利用して海水浴に行った友達からぜひ来いという端書を受け取ったので、私は多少の金を工面して、出掛ける事にした。",
        "私は金の工面に二、三日を費やした。",
        "ところが私が鎌倉に着いて三日と経たないうちに、私を呼び寄せた友達は、急に国元から帰れという電報を受け取った。",
        "電報には母が病気だからと断ってあったけれども友達はそれを信じなかった。",
        "友達はかねてから国元にいる親たちに勧まない結婚を強いられていた。"
    ];
    for i in 0..original_text.len() {
        let img = text2image(original_text[0..i].join("\n"));
        img.save(Path::new(&format!("test_images/test{}.png", i))).unwrap();
    }
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

fn get_vl_gothic<'a>() -> Font<'a> {
    let font = Vec::from(include_bytes!("../assets/font/VL-Gothic-Regular.ttf") as &[u8]);
    FontCollection::from_bytes(font).unwrap().into_font().unwrap()
}

fn get_tanuki_magic<'a>() -> Font<'a> {
    let font = Vec::from(include_bytes!("../assets/font/TanukiMagic.ttf") as &[u8]);
    FontCollection::from_bytes(font).unwrap().into_font().unwrap()    
}

fn char_len(c: char) -> u32 { // in ascii character
    if c.is_ascii() {
        1
    } else {
        2
    }
}
