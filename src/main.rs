#[macro_use] extern crate rocket;
extern crate diesel;
#[macro_use] extern crate rocket_sync_db_pools;

mod models;
mod schema;
mod respositories {
    pub mod users;
}

use models::{Login, NewUser};
use rocket::{http::Status, response::status::Custom, serde::json::{Json, Value}};
use serde_json::json;
use respositories::users::UserRespository;

#[database("social_media")]
struct DbConn(diesel::PgConnection);

#[post("/login", format = "json", data = "<user>")]
async fn sign_in(db: DbConn, user: Json<Login>) -> Result<Value, Custom<Value>> {
    db.run(|c|{
        let result = UserRespository::handle_login(c, user.into_inner());
        result
        .map(|user| json!(user))
        .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[post("/register", format = "json", data = "<new_user>")]
async fn register(db: DbConn, new_user: Json<NewUser>) -> Result<Value, Custom<Value>> {
   db.run(|c| {
        let result = UserRespository::create_user(c, new_user.into_inner());
        result
        .map(|user| json!(user))
        .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found")
}

#[catch(422)]
fn unprocessable_entity() -> Value {
    json!("Unprocessable Entity")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![
        sign_in,
        register
        ])
    .register("/", catchers![
        not_found,
        unprocessable_entity
    ])
    .attach(DbConn::fairing())
}