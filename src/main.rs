use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::http::Status;

#[macro_use] extern crate rocket;

// Auth


// Users


// Tags

// Articles
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

#[launch]
fn rocket() -> _ {
    let api_url = "/api/v1_dev".to_string();

    rocket::build()
        //.mount(format!("{}{}", api_url, "/"), routes![])
        //.mount(format!("{}{}", api_url, "/auth"), routes![])
        //.mount(format!("{}{}", api_url, "/tags"), routes![])
        .mount(format!("{}{}", api_url, "/article"), routes![get_articles, get_article])
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Role {
    id: i64,
    name: String,
    description: String
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct User {
    id: i64,
    pseudo: String,
    about: String,

    inscription_date: String,
    last_connection_date: String
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Tag {
    id: i64,

    name: String,
    description: String
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Article {
    id: i64,

    author_id: i64,
    author_pseudo: String,

    title: String,
    content: String
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Comment {
    id: i64,

    article_id: i64,

    author_id: i64,
    author_pseudo: String,

    content: String
}