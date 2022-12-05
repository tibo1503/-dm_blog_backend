use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::http::Status;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    let api_url = "/api/v1_dev".to_string();

    rocket::build()
        //.mount(format!("{}{}", api_url, "/"), routes![])
        //.mount(format!("{}{}", api_url, "/auth"), routes![])
        .mount(format!("{}{}", api_url, "/article_tags"), routes![get_article_tags, get_article_tag])
        .mount(format!("{}{}", api_url, "/article"), routes![get_articles, get_article])
}

// Role
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Role {
    id: i64,
    name: String,
    description: String
}

// Auth


// Users
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct User {
    id: i64,
    pseudo: String,
    about: String,

    inscription_date: String,
    last_connection_date: String
}

// Article tags
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Tag {
    id: i64,

    name: String,
    description: String
}

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
        id => Result::Err(Status::NotFound)
    }
}

// Articles
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Article {
    id: i64,

    author_id: i64,
    author_pseudo: String,

    title: String,
    content: String
}

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
        id => Result::Err(Status::NotFound)
    }
}

// Comment
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Comment {
    id: i64,

    article_id: i64,

    author_id: i64,
    author_pseudo: String,

    content: String
}

