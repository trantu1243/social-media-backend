#[macro_use] extern crate rocket;
extern crate diesel;
#[macro_use] extern crate rocket_sync_db_pools;
extern crate dotenv;

mod models;
mod schema;
mod jwt;
mod authorization;
mod respositories {
    pub mod users;
}

use models::{Login, NewUser};
use rocket::{http::{Method, Status}, response::status::Custom, serde::json::{Json, Value}};
use serde_json::json;
use respositories::users::UserRespository;
use rocket_cors::AllowedOrigins;
use authorization::BearerToken;

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

#[get("/auth")]
async fn authorize(db: DbConn, _auth: BearerToken) -> Result<Value, Custom<Value>> {
    db.run(move |c|{
        let result = BearerToken::get_user(&_auth, c);
        result
        .map(|user| json!(user))
        .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
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
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000"]);
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete, Method::Patch].into_iter().map(From::from).collect(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("CORS fairing cannot be created");
    
    rocket::build()
    .mount("/", routes![
        sign_in,
        register,
        authorize
    ])
    .register("/", catchers![
        not_found,
        unprocessable_entity
    ])
    .attach(DbConn::fairing())
    .attach(cors)
}