use rocket::serde::{json::Json};
use rocket::http::Status;

#[macro_use] extern crate rocket;

mod cors;

#[launch]
fn rocket() -> _ {
    let api_url = "/api/v1_dev".to_string();

    rocket::build()
        .attach(cors::CORS)
        .mount(format!("{}{}", api_url, "/"), routes![login, logout])
        .mount(format!("{}{}", api_url, "/user"), routes![get_users, get_user])
        .mount(format!("{}{}", api_url, "/article_tag"), routes![get_article_tags, get_article_tag])
        .mount(format!("{}{}", api_url, "/article"), routes![get_articles, get_article])
}
mod serialization_struct;

// Auth
use rocket::form::Form;
use rocket::http::{CookieJar, Cookie};
//use crate::token_const::{TOKEN, TOKEN_COOKIE_FIELD};
mod token_const;

#[derive(FromForm)]
struct Login {
    username: String,
    password: String
}

#[post("/login", data = "<login>")]
fn login(login: Form<Login>, cookie: &CookieJar<'_>) -> Status {
    if (login.username == "Dofe") && (login.password == "1234") {
/*
        if cookie.get(TOKEN_COOKIE_NAME) != Option::None {
            cookie.remove(Cookie::named(TOKEN_COOKIE_NAME));
        }
*/       
        cookie.add(Cookie::new("token", token_const::TOKEN));
        Status::Accepted
    } else {
        Status::NotAcceptable
    }
}

#[post("/logout")]
fn logout(cookie: &CookieJar<'_>) {
    cookie.remove(Cookie::named(token_const::TOKEN_COOKIE_FIELD));
}

// User
use serialization_struct::user::User;
mod request_guard;
//use crate::{TOKEN, TOKEN_COOKIE_NAME}

#[get("/")]
fn get_users(_user: request_guard::user::User) -> Result<Json<Vec<User>>, Status> {
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

