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
mod aws_s3;

use models::{Login, NewUser};
use rocket::{form::Form, fs::TempFile, http::{Method, Status}, response::status::Custom, serde::json::{Json, Value}};
use serde_json::json;
use respositories::users::UserRespository;
use rocket_cors::AllowedOrigins;
use authorization::BearerToken;
use tokio::io::AsyncReadExt;
use crate::aws_s3::AwsS3;

#[database("social_media")]
struct DbConn(diesel::PgConnection);

#[post("/login", format = "json", data = "<user>")]
async fn sign_in(db: DbConn, user: Json<Login>) -> Result<Value, Custom<Value>> {
    db.run(|c|{
        let result = UserRespository::handle_login(c, user.into_inner());
        result
        .map(|user| json!(user))
        .map_err(|e| Custom(Status::InternalServerError, json!({"error": e.to_string()})))
    }).await
}

#[post("/register", format = "json", data = "<new_user>")]
async fn register(db: DbConn, new_user: Json<NewUser>) -> Result<Value, Custom<Value>> {
   db.run(|c| {
        let result = UserRespository::create_user(c, new_user.into_inner());
        result
        .map(|user| json!(user))
        .map_err(|e| Custom(Status::InternalServerError, json!({"error": e.to_string()})))
    })
    .await
}

#[get("/auth")]
async fn authorize(db: DbConn, _auth: BearerToken) -> Result<Value, Custom<Value>> {
    db.run(move |c|{
        let result = BearerToken::get_user(&_auth, c);
        result
        .map(|user| json!(user))
        .map_err(|e| Custom(Status::InternalServerError, json!({"error": e.to_string()})))
    }).await
}

#[get("/user/<id>")]
async fn user_from_id(db: DbConn, id: i32) -> Result<Value, Custom<Value>>{
    db.run(move |c|{
        let result = UserRespository::get_user_info(c, id);
        result
        .map(|user| json!(user))
        .map_err(|e| Custom(Status::InternalServerError, json!({"error": e.to_string()})))
    }).await
}

#[derive(FromForm)]
struct FileUploadForm<'a> {
    file: TempFile<'a>,
    r#type: &'a str,
    id: &'a str,
}

#[post("/upload/avatar", data = "<data>")]
async fn upload_avatar(db: DbConn, _auth: BearerToken, data: Form<FileUploadForm<'_>>) -> Result<Value, Custom<Value>> {
    let file = &data.file;
    let extension =  data.r#type;
    let id = data.id.to_string();
    let mut bytes = vec![];
    if let Ok(mut file) = file.open().await {
        let _ = file.read_to_end(&mut bytes).await;
    }
    let res = AwsS3::upload_s3(bytes, extension).await;

    match res {
        Ok(url) => {
            db.run(move |c| {
                UserRespository::save_avatar(c, id, url)
                    .map(|user| json!(user))
                    .map_err(|e| Custom(Status::InternalServerError, json!({"error": e.to_string()})))
            })
            .await
        },
        Err(_) => Err(Custom(Status::InternalServerError, json!({"error": "Failed to upload to S3"}))),
    }
}

#[post("/upload/background", data = "<data>")]
async fn upload_background(db: DbConn, _auth: BearerToken, data: Form<FileUploadForm<'_>>) -> Result<Value, Custom<Value>> {
    let file = &data.file;
    let extension =  data.r#type;
    let id = data.id.to_string();
    let mut bytes = vec![];
    if let Ok(mut file) = file.open().await {
        let _ = file.read_to_end(&mut bytes).await;
    }
    let res = AwsS3::upload_s3(bytes, extension).await;

    match res {
        Ok(url) => {
            db.run(move |c| {
                UserRespository::save_background(c, id, url)
                    .map(|user| json!(user))
                    .map_err(|e| Custom(Status::InternalServerError, json!({"error": e.to_string()})))
            })
            .await
        },
        Err(_) => Err(Custom(Status::InternalServerError, json!({"error": "Failed to upload to S3"}))),
    }
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
        authorize,
        user_from_id,
        upload_avatar,
        upload_background
    ])
    .register("/", catchers![
        not_found,
        unprocessable_entity
    ])
    .attach(DbConn::fairing())
    .attach(cors)
}