use base64;
use diesel;
use model;
use r2d2;
use r2d2_diesel;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::env;
use std::ops::Deref;

/* Guard Repository */

type DieselPool = r2d2::Pool<r2d2_diesel::ConnectionManager<diesel::PgConnection>>;

pub struct Repository(pub model::Repository);

impl Deref for Repository {
    type Target = model::Repository;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Repository {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<DieselPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Repository(model::Repository::new(conn))),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

/* Guard Client IP address */

pub struct ClientIP(pub String);

impl ClientIP {
    pub fn address(self) -> String {
        self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ClientIP {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let ip_address = match request.client_ip() {
            Some(ip_address) => format!("{}", ip_address),
            None => String::from(""),
        };
        Outcome::Success(ClientIP(ip_address))
    }
}

/* Guard BasicAuth */

pub struct BasicAuth();

impl<'a, 'r> FromRequest<'a, 'r> for BasicAuth {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        if let Some(sent_auth_code) = request.headers().get("Authorization").next() {
            let username = env::var("ADMIN_USERNAME").unwrap();
            let password = env::var("ADMIN_PASSWORD").unwrap();
            let b64 = base64::encode(&format!("{}:{}", username, password));
            let valid_auth_code = format!("Basic {}", b64);

            if sent_auth_code == valid_auth_code {
                return Outcome::Success(BasicAuth());
            }
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}

/* Guard Force SSL */
// Depends on heroku (X-Forwarded-Proto)

pub struct ForceSSL();

impl<'a, 'r> FromRequest<'a, 'r> for ForceSSL {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        if let Some(schema) = request.headers().get("X-Forwarded-Proto").next() {
            if schema == String::from("http") {
                return Outcome::Success(ForceSSL());
            }
        }

        Outcome::Forward(())
    }
}
