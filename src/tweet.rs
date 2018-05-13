use image;
use tokio_core::reactor::Core;
use egg_mode;
use egg_mode::media::{UploadBuilder, media_types};
use egg_mode::tweet::DraftTweet;
use std::path::Path;
use uuid::Uuid;
use std::fs::File;
use std::env;
use std::io::Read;
use std::thread;

fn get_twitter_access_token() -> egg_mode::Token {
    let con_token = egg_mode::KeyPair::new(env::var("TWITTER_CONSUMER_KEY").unwrap(), env::var("TWITTER_CONSUMER_SECRET").unwrap());
    let access_token = egg_mode::KeyPair::new(env::var("TWITTER_ACCESS_TOKEN").unwrap(), env::var("TWITTER_ACCESS_SECRET").unwrap());

    egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    }
}

pub fn tweet_answer(answer: String, question_image: image::RgbImage) {
    thread::spawn(move || {
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let token = get_twitter_access_token();

        let tmp_filepath = format!("/tmp/{}.jpg", Uuid::new_v4());
        let tmp_filepath = Path::new(&tmp_filepath);
        question_image.save(&tmp_filepath).unwrap();

        let mut image_buf = vec![];
        File::open(&tmp_filepath).unwrap().read_to_end(&mut image_buf).unwrap();
        let builder = UploadBuilder::new(image_buf, media_types::image_jpg());
        let media_handle = core.run(builder.call(&token, &handle)).unwrap();

        let draft = DraftTweet::new(answer)
                        .media_ids(&[media_handle.id]);
        core.run(draft.send(&token, &handle)).unwrap();
    });
}