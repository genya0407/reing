pub mod schema;

use self::schema::questions;
use chrono::prelude::*;

#[derive(Insertable)]
#[table_name="questions"]
pub struct NewQuestion {
    pub ip_address: String,
    pub body: String,
}

#[derive(Queryable)]
pub struct Question {
    pub id: i32,
    pub body: String,
    pub ip_address: String,
    pub hidden: bool,
    pub created_at: DateTime<Utc>,
}