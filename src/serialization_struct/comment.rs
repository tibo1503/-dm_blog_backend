use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Comment {
    pub id: i64,

    pub article_id: i64,

    pub author_id: i64,
    pub author_pseudo: String,

    pub content: String
}