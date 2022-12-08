use rocket::serde::{json::Json};
use rocket::http::Status;

use crate::serialization_struct::article::Article;
use crate::serialization_struct::tags::Tag;

#[get("/")]
pub fn get_articles() -> Result<Json<Vec<Article>>, Status> {
    let articles = vec![
        Article {
            id: 1,

            author_id: 1,
            author_pseudo: "Dofe".to_string(),
        
            title: "Why use Rust ?".to_string(),
            content: "Security".to_string(),

            creation_date: "2022-09-24 22:21:20".to_string(),
            tags: vec![
                Tag {
                    id: 1,
                    name: "Rust".to_string(),
                    description: "Compiled programming language with security oriented syntax compiler with a lot of high level programming language features".to_string(),
                }
            ],
            picture_url: "https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/2048px-Rust_programming_language_black_logo.svg.png".to_string()
        },
        Article {
            id: 2,

            author_id: 1,
            author_pseudo: "Dofe".to_string(),
        
            title: "What is Wasm ?".to_string(),
            content: "Any \"alternative\" for JS".to_string(),

            creation_date: "2022-08-23 21:21:20".to_string(),
            tags: vec![
                Tag {
                    id: 3,
                    name: "WASM".to_string(),
                    description: "Any assembly dedicated to web, but also use into Docker and some other usage".to_string()
                }    
            ],
            picture_url: "https://upload.wikimedia.org/wikipedia/commons/thumb/1/1f/WebAssembly_Logo.svg/2048px-WebAssembly_Logo.svg.png".to_string()
        }
    ];

    Result::Ok(Json(articles))
}

#[get("/<id>")]
pub fn get_article(id: u64) -> Result<Json<Article>, Status> {
    match id {
        1 => Result::Ok(Json(Article {
            id: 1,

            author_id: 1,
            author_pseudo: "Dofe".to_string(),
        
            title: "Why use Rust ?".to_string(),
            content: "Security".to_string(),

            creation_date: "2022-09-24 22:21:20".to_string(),
            tags: vec![
                Tag {
                    id: 1,
                    name: "Rust".to_string(),
                    description: "Compiled programming language with security oriented syntax compiler with a lot of high level programming language features".to_string(),
                }
            ],
            picture_url: "https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/2048px-Rust_programming_language_black_logo.svg.png".to_string()
        })),
        2 => Result::Ok(Json(Article {
            id: 2,

            author_id: 1,
            author_pseudo: "Dofe".to_string(),
        
            title: "What is Wasm ?".to_string(),
            content: "Any \"alternative\" for JS".to_string(),

            creation_date: "2022-08-23 21:21:20".to_string(),
            tags: vec![
                Tag {
                    id: 3,
                    name: "WASM".to_string(),
                    description: "Any assembly dedicated to web, but also use into Docker and some other usage".to_string()
                }    
            ],
            picture_url: "https://upload.wikimedia.org/wikipedia/commons/thumb/1/1f/WebAssembly_Logo.svg/2048px-WebAssembly_Logo.svg.png".to_string()
        })),
        _ => Result::Err(Status::NotFound)
    }
}