use super::model;
use egg_mode;
use egg_mode::media::{media_types, UploadBuilder};
use egg_mode::tweet::DraftTweet;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tokio::prelude::Future;
use tokio::runtime::current_thread::block_on_all;
use uuid::Uuid;

fn get_twitter_access_token() -> Result<egg_mode::Token, std::env::VarError> {
    let con_token = egg_mode::KeyPair::new(
        env::var("TWITTER_CONSUMER_KEY")?,
        env::var("TWITTER_CONSUMER_SECRET")?,
    );
    let access_token = egg_mode::KeyPair::new(
        env::var("TWITTER_ACCESS_TOKEN")?,
        env::var("TWITTER_ACCESS_SECRET")?,
    );

    Ok(egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    })
}

pub fn get_twitter_username() -> String {
    match env::var("TWITTER_SCREEN_NAME") {
        Ok(screen_name) => match get_twitter_access_token() {
            Ok(token) => {
                block_on_all(egg_mode::user::show(&screen_name, &token).map(|u| u.clone().name))
                    .expect("send request")
            }
            _ => env::var("PROFILE_USERNAME")
                .expect("failed to fetch environment variable PROFILE_USERNAME"),
        },
        _ => env::var("PROFILE_USERNAME")
            .expect("failed to fetch environment variable PROFILE_USERNAME"),
    }
}

pub fn tweet_answer(answer: model::Answer) {
    let question_image = reing_text2image::TextImage::new(
        answer.question.body,
        String::from("Reing"),
        (0x2c, 0x36, 0x5d),
    );
    let tmp_filepath = format!("/tmp/{}.jpg", Uuid::new_v4());
    let tmp_filepath = Path::new(&tmp_filepath);
    question_image
        .save_image(&tmp_filepath)
        .expect("failed to save image");
    let mut image_buf = vec![];
    File::open(&tmp_filepath)
        .expect("failed to open image file")
        .read_to_end(&mut image_buf)
        .expect("failed to read image file");

    let question_url = format!(
        "https://{}/answer/{}",
        env::var("APPLICATION_DOMAIN").expect("failed to fetch environment variable"),
        answer.id
    );
    let tweet_text = format!("{} #reing {}", answer.body, question_url);

    let token = match get_twitter_access_token() {
        Ok(token) => token,
        _ => return,
    };
    let builder = UploadBuilder::new(image_buf, media_types::image_jpg());
    let media_handle = block_on_all(builder.call(&token)).expect("failed to upload media");
    let draft = DraftTweet::new(tweet_text).media_ids(&[media_handle.id]);
    block_on_all(draft.send(&token)).expect("failed to send tweet");
}
