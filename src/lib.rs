extern crate postgres;
extern crate chrono;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate query_builder;

use chrono::prelude::*;
use query_builder::*;
use postgres::rows::Row;

type PostgresConn = r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>;

#[derive(Debug)]
pub struct Question {
    pub id: i32,
    pub body: String,
    pub ip_address: String,
    pub created_at: DateTime<Local>,
    pub hidden: bool,
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
            "INSERT INTO questions (body, ip_address) VALUES ($1, $2) RETURNING id, created_at, hidden",
            &[&body, &ip_address]
        ).unwrap();
        let row = rows.iter().next().unwrap();

        Question {
            id: row.get("id"),
            body: body,
            ip_address: ip_address,
            created_at: row.get("created_at"),
            hidden: row.get("hidden"),
        }
    }

    pub fn all_questions(&self) -> Vec<Question> {
        let query = self.questions_select_query();
        let rows = self.conn.query(&query.as_string(), &[]).unwrap();
        rows.into_iter().map(|row| self.row2question(row)).collect::<Vec<Question>>()
    }

    pub fn find_question(&self, id: i32) -> Option<Question> {
        let mut query = self.questions_select_query();
        query.limit(1);
        query.whre.push(WhereClause::new("id", Value::Int(id), None));
        let rows = self.conn.query(&query.as_string(), &[]).unwrap();
        rows.into_iter().next().map(|row| self.row2question(row))
    }

    pub fn hide_question(&self, id: i32) {
        let mut query = self.questions_update_query();
        query.whre.push(WhereClause::new("id", Value::Int(id), None));
        query.set.insert("hidden", Value::Bool(true));
        self.conn.execute(&query.as_string(), &[]).unwrap();
    }

    fn questions_update_query(&self) -> UpdateQuery {
        UpdateQuery::update("questions")
    }

    fn questions_select_query(&self) -> SelectQuery {
        SelectQuery::select(&["id", "body", "hidden", "ip_address", "created_at"]).from("questions")
    }

    fn row2question(&self, row: Row) -> Question {
        Question {
            id: row.get("id"),
            body: row.get("body"),
            ip_address: row.get("ip_address"),
            created_at: row.get("created_at"),
            hidden: row.get("hidden"),
        }
    }
}
