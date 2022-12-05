use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Article {
    pub id: i64,

    pub author_id: i64,
    pub author_pseudo: String,

    pub title: String,
    pub content: String
}