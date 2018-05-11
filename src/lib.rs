extern crate postgres;
extern crate chrono;
extern crate r2d2;
extern crate r2d2_postgres;

use chrono::prelude::*;

type PostgresConn = r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>;

#[derive(Debug)]
pub struct Question {
    pub id: i32,
    pub body: String,
    pub ip_address: String,
    pub created_at: DateTime<Local>
}

pub struct Repository {
    conn: PostgresConn
}

impl Repository {
    pub fn new(conn: PostgresConn) -> Self {
        Self { conn: conn }
    }

    pub fn store_question(&self, body: String, ip_address: String) -> Question {
        let rows = self.conn.query(
            "INSERT INTO questions (body, ip_address) VALUES ($1, $2) RETURNING id, created_at",
            &[&body, &ip_address]
        ).unwrap();
        let row = rows.iter().next().unwrap();

        Question {
            id: row.get("id"),
            body: body,
            ip_address: ip_address,
            created_at: row.get("created_at")
        }
    }
}
