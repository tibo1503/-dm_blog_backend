use rocket::form::Form;
use rocket::http::{Status, CookieJar, Cookie};
//use crate::token_const::{TOKEN, TOKEN_COOKIE_FIELD};

pub const TOKEN: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
pub const TOKEN_COOKIE_FIELD: &str = "token";

#[derive(FromForm)]
pub struct Login {
    username: String,
    password: String
}

#[post("/login", data = "<login>")]
pub fn login(login: Form<Login>, cookie: &CookieJar<'_>) -> Status {
    if (login.username == "Dofe") && (login.password == "1234") {
/*
        if cookie.get(TOKEN_COOKIE_NAME) != Option::None {
            cookie.remove(Cookie::named(TOKEN_COOKIE_NAME));
        }
*/       
    cookie.add(Cookie::new(TOKEN_COOKIE_FIELD, TOKEN));
        Status::Accepted
    } else {
        Status::NotAcceptable
    }
}

#[post("/logout")]
pub fn logout(cookie: &CookieJar<'_>) {
    cookie.remove(Cookie::named(TOKEN_COOKIE_FIELD));
}