use rocket::serde::{json::Json};
use rocket::http::Status;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    let api_url = "/api/v1_dev".to_string();

    rocket::build()
        //.mount(format!("{}{}", api_url, "/"), routes![])
        //.mount(format!("{}{}", api_url, "/auth"), routes![])
        .mount(format!("{}{}", api_url, "/user"), routes![get_users, get_user])
        .mount(format!("{}{}", api_url, "/article_tag"), routes![get_article_tags, get_article_tag])
        .mount(format!("{}{}", api_url, "/article"), routes![get_articles, get_article])
}
mod serialization_struct;

// Auth

// User
use serialization_struct::user::User;

#[get("/")]
fn get_users() -> Result<Json<Vec<User>>, Status> {
    Result::Ok(Json(vec![
        User {
            id: Option::Some(1),
            pseudo: Option::Some("Dofe".to_string()),
            ..Default::default()
        },
        User {
            id: Option::Some(2),
            pseudo: Option::Some("Mac_Bro0k".to_string()),
            ..Default::default()
        }
    ]))
}

#[get("/<id>")]
fn get_user(id: u64) -> Result<Json<User>, Status> {
    match id { 
        1 => Result::Ok(Json(User {
            id: Option::Some(1),
            pseudo: Option::Some("Dofe".to_string()),
            ..Default::default()
        })),
        2 => Result::Ok(Json(User {
            id: Option::Some(2),
            pseudo: Option::Some("Mac_Bro0k".to_string()),
            ..Default::default()
        })),
        _ => Result::Err(Status::NotFound)
    }
}

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

