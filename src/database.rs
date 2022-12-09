use rocket_db_pools::{Database, Connection};

#[derive(Database)]
#[database("blog")]
pub struct Blog(sqlx::MySqlPool);