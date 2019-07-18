extern crate dotenv;
extern crate chrono;
extern crate uuid;
extern crate base64;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate diesel;
extern crate egg_mode;
extern crate tokio;
extern crate lettre;
extern crate lettre_email;
extern crate htmlescape;
extern crate reing_text2image;
extern crate log;

mod entity;
mod usecase;

fn main() {
    dotenv::dotenv().ok();
}
