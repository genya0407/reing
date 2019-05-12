use egg_mode;
use egg_mode::media::{UploadBuilder, media_types};
use egg_mode::tweet::DraftTweet;
use tokio::runtime::current_thread::block_on_all;
use std::path::Path;
use uuid::Uuid;
use std::fs::File;
use std::env;
use std::io::Read;
use tokio::prelude::Future;
use super::model;

fn get_twitter_access_token() -> Result<egg_mode::Token, std::env::VarError> {
    let con_token = egg_mode::KeyPair::new(env::var("TWITTER_CONSUMER_KEY")?, env::var("TWITTER_CONSUMER_SECRET")?);
    let access_token = egg_mode::KeyPair::new(env::var("TWITTER_ACCESS_TOKEN")?, env::var("TWITTER_ACCESS_SECRET")?);

    Ok(egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    })
}

pub fn get_twitter_username() -> String {
    match env::var("TWITTER_SCREEN_NAME") {
        Ok(screen_name) => {
            match get_twitter_access_token() {
                Ok(token) => block_on_all(egg_mode::user::show(&screen_name, &token).map(|u| u.clone().name)).unwrap(),
                _ => env::var("PROFILE_USERNAME").unwrap()
            }
        },
        _ => env::var("PROFILE_USERNAME").unwrap()
    }
}

pub fn tweet_answer(answer: model::Answer) {
    let question_image = reing_text2image::TextImage::new(answer.question.body, String::from("Reing"), (0x2c, 0x36, 0x5d));
    let tmp_filepath = format!("/tmp/{}.jpg", Uuid::new_v4());
    let tmp_filepath = Path::new(&tmp_filepath);
    question_image.save_image(&tmp_filepath).unwrap();
    let mut image_buf = vec![];
    File::open(&tmp_filepath).unwrap().read_to_end(&mut image_buf).unwrap();

    let question_url = format!(
        "https://{}/answer/{}",
        env::var("APPLICATION_DOMAIN").unwrap(),
        answer.id
    );
    let tweet_text = format!("{} #reing {}", answer.body, question_url);

    let token = match get_twitter_access_token() {
        Ok(token) => token,
        _ => return
    };
    let builder = UploadBuilder::new(image_buf, media_types::image_jpg());
    let media_handle = block_on_all(builder.call(&token)).unwrap();
    let draft = DraftTweet::new(tweet_text)
                    .media_ids(&[media_handle.id]);
    block_on_all(draft.send(&token)).unwrap();
}
