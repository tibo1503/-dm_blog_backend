
use rocket::serde::{json::Json};
use rocket::http::Status;

use serialization_struct::user::User;
use crate::{request_guard, serialization_struct};

#[get("/")]
pub fn get_users(_user: request_guard::user::User) -> Result<Json<Vec<User>>, Status> {
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
pub fn get_user(id: u64) -> Result<Json<User>, Status> {
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