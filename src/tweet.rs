use super::model;
use egg_mode;
use egg_mode::tweet::DraftTweet;
use std::env;
use tokio::prelude::Future;
use tokio::runtime::current_thread::block_on_all;

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
    let draft = DraftTweet::new(tweet_text);
    block_on_all(draft.send(&token)).expect("failed to send tweet");
}
