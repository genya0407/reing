use egg_mode;
use tokio_core::reactor::Core;

fn get_twitter_access_token() -> egg_mode::Token {
    let con_token = egg_mode::KeyPair::new(env::var("TWITTER_CONSUMER_KEY").unwrap(), env::var("TWITTER_CONSUMER_SECRET").unwrap());
    let access_token = egg_mode::KeyPair::new(env::var("TWITTER_ACCESS_TOKEN").unwrap(), env::var("TWITTER_ACCESS_SECRET").unwrap());

    egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    }
}

/*
fn tweet() -> String {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let token = get_twitter_access_token();
    let rustlang = core.run(egg_mode::user::show("rustlang", &token, &handle)).unwrap();

    format!("{} (@{})", rustlang.name, rustlang.screen_name)
}
*/