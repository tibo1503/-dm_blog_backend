
use std::default;

use rocket::serde::{json::Json};
use rocket::http::Status;

use crate::database::Blog;
use rocket_db_pools::Connection;
use rocket_db_pools::sqlx::Row;

use serialization_struct::user::User;
use crate::{request_guard, serialization_struct};

use crate::database;

#[get("/")]
pub async fn get_users(mut db: Connection<Blog>, _user: request_guard::user::User) -> Result<Json<Vec<User>>, Status> {
    let query_result = sqlx::query("SELECT id, pseudo, about FROM user_blog")
        .fetch_all(&mut *db).await
        .and_then(|e| Ok (
            e.iter().map(|r| 
                User {
                    id: Option::Some(r.try_get(0).unwrap()),
                    pseudo: Option::Some(r.try_get(1).unwrap()),
                    about: match r.try_get(2) {
                        Result::Ok(x) => Option::Some(x),
                        Result::Err(_) => Option::None
                    },
                    ..Default::default()
                }
            ).collect::<Vec<User>>()
        )).ok();

    match query_result {
        Option::Some(x) => Result::Ok(Json(x)),
        Option::None => Result::Err(Status::NotFound)
    }
}

#[get("/<id>")]
pub async fn get_user(mut db: Connection<Blog>, id: u64) -> Result<Json<User>, Status> {
    let result = sqlx::query("SELECT id, pseudo, about FROM user_blog WHERE id = ?").bind(id)
        .fetch_one(&mut *db).await
        .and_then(|r| Ok(User {
            id: Option::Some(r.try_get(0)?),
            pseudo: Option::Some(r.try_get(1)?),
            about: match r.try_get(2) {
                Result::Ok(x) => Option::Some(x),
                Result::Err(_) => Option::None
            },
            ..Default::default()
        })).ok();


    match result {
        Option::Some(x) => Result::Ok(Json(x)),
        Option::None => Result::Err(Status::NotFound)
    }
}