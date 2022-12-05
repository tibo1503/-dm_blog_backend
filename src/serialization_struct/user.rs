use rocket::serde::{Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i64,
    pub pseudo: String,
    pub about: String,

    pub inscription_date: String,
    pub last_connection_date: String
}