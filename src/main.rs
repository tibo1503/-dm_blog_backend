use rocket::serde::{json::Json};
use rocket::http::Status;

#[macro_use] extern crate rocket;

mod cors;
mod request;
mod request_guard;
use request::*;
pub const TOKEN: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
pub const TOKEN_COOKIE_FIELD: &str = "token";

#[launch]
fn rocket() -> _ {
    let api_url = "/api/v1_dev".to_string();

    rocket::build()
        .attach(cors::CORS)
        .mount(format!("{}{}", api_url, "/"), routes![auth::login, auth::logout])
        .mount(format!("{}{}", api_url, "/user"), routes![user::get_users, user::get_user])
        .mount(format!("{}{}", api_url, "/article_tag"), routes![get_article_tags, get_article_tag])
        .mount(format!("{}{}", api_url, "/article"), routes![get_articles, get_article])
}
mod serialization_struct;

// Article tags
use serialization_struct::tags::Tag;

#[get("/")]
fn get_article_tags() -> Result<Json<Vec<Tag>>, Status> {
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
        }
    ];

    Result::Ok(Json(tags))
}

#[get("/<id>")]
fn get_article_tag(id: u64) -> Result<Json<Tag>, Status> {
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
        _ => Result::Err(Status::NotFound)
    }
}

// Articles
use serialization_struct::article::Article;

#[get("/")]
fn get_articles() -> Result<Json<Vec<Article>>, Status> {
    let articles = vec![
        Article {
            id: 1,

            author_id: 1,
            author_pseudo: "Dofe".to_string(),
        
            title: "Why use Rust ?".to_string(),
            content: "Security".to_string()
        },
        Article {
            id: 2,

            author_id: 1,
            author_pseudo: "Dofe".to_string(),
        
            title: "What is Wasm ?".to_string(),
            content: "Any \"alternative\" for JS".to_string()
        }
    ];

    Result::Ok(Json(articles))
}

#[get("/<id>")]
fn get_article(id: u64) -> Result<Json<Article>, Status> {
    match id {
        1 => Result::Ok(Json(Article {
            id: 1,

            author_id: 1,
            author_pseudo: "Dofe".to_string(),
        
            title: "Why use Rust ?".to_string(),
            content: "Security".to_string()
        })),
        2 => Result::Ok(Json(Article {
            id: 2,

            author_id: 1,
            author_pseudo: "Dofe".to_string(),
        
            title: "What is Wasm ?".to_string(),
            content: "Any \"alternative\" of JS".to_string()
        })),
        _ => Result::Err(Status::NotFound)
    }
}

// Comment
//use serialization_struct::comment::Comment;

