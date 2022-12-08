use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Tag {
    pub id: i64,

    pub name: String,
    pub description: String
}