use rocket_db_pools::Database;

#[macro_use] extern crate rocket;

mod cors;
mod request;
mod request_guard;
mod database;
use database::Blog;

//use request::*;
pub const TOKEN: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
pub const TOKEN_COOKIE_FIELD: &str = "token";

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();

    let api_url = "/api/v1_dev".to_string();

    rocket::build()
        .attach(cors::CORS)
        .attach(Blog::init())
        .mount(format!("{}{}", api_url, "/"), routes![request::auth::login, request::auth::logout])
        .mount(format!("{}{}", api_url, "/user"), routes![request::user::get_users, request::user::get_user])
        .mount(format!("{}{}", api_url, "/article_tag"), routes![request::tags::get_article_tags, request::tags::get_article_tag])
        .mount(format!("{}{}", api_url, "/article"), routes![request::article::get_articles, request::article::get_article])
}
mod serialization_struct;

