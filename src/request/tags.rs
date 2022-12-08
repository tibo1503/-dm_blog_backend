// Article tags
use rocket::serde::{json::Json};
use rocket::http::Status;

use crate::serialization_struct;

use serialization_struct::tags::Tag;

#[get("/")]
pub fn get_article_tags() -> Result<Json<Vec<Tag>>, Status> {
    let tags = vec![
        Tag {
            id: 1,
            name: "Rust".to_string(),
            description: "Compiled programming language with security oriented syntax compiler with a lot of high level programming language features".to_string(),
        },
        Tag {
            id: 2,
            name: "C++".to_string(),
            description: "Compiled programming language mainly know to be OOP (Oriented Object Programming)".to_string()
        },
        Tag {
            id: 3,
            name: "WASM".to_string(),
            description: "Any assembly dedicated to web, but also use into Docker and some other usage".to_string()
        }
    ];

    Result::Ok(Json(tags))
}

#[get("/<id>")]
pub fn get_article_tag(id: u64) -> Result<Json<Tag>, Status> {
    match id {
        1 => Result::Ok(Json(Tag {
            id: 1,
            name: "Rust".to_string(),
            description: "Compiled programming language with security oriented syntax compiler with a lot of high level programming language features".to_string(),
        })),
        2 => Result::Ok(Json(Tag {
            id: 2,
            name: "C++".to_string(),
            description: "Compiled programming language mainly know to be OOP (Oriented Object Programming)".to_string()
        })),
        3 => Result::Ok(Json(Tag {
            id: 3,
            name: "WASM".to_string(),
            description: "Any assembly dedicated to web, but also use into Docker and some other usage".to_string()
        })),
        _ => Result::Err(Status::NotFound)
    }
}