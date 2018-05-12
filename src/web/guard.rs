use model;
use r2d2;
use r2d2_diesel;
use diesel;
use std::ops::Deref;
use std::net::SocketAddr;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

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
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

/* Guard Client IP address */

pub struct ClientIP(pub SocketAddr);

impl ClientIP {
    pub fn address(&self) -> String {
        match self.0 {
            SocketAddr::V4(v4) => format!("{}", v4.ip()),
            SocketAddr::V6(v6) => format!("{}", v6.ip())
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ClientIP {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match request.remote() {
            Some(socket_addr) => Outcome::Success(ClientIP(socket_addr)),
            None => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}