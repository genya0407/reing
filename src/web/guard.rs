use reing;
use r2d2;
use r2d2_postgres;
use std::ops::Deref;
use std::net::SocketAddr;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

/* Guard Repository */

type PostgresPool = r2d2::Pool<r2d2_postgres::PostgresConnectionManager>;

pub struct Repository(pub reing::Repository);

impl Deref for Repository {
    type Target = reing::Repository;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Repository {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<PostgresPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Repository(reing::Repository::new(conn))),
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