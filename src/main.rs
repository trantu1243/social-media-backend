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
    pub mod posts;
    pub mod comments;
    pub mod friend_request;
    pub mod notifications;
}
mod aws_s3;

use models::{CommentInput, DataId, Login, NewUser, SearchName};
use rocket::{form::Form, fs::TempFile, http::{Method, Status}, response::status::Custom, serde::json::{Json, Value}};
use serde_json::json;
use respositories::{comments::CommentRespository, friend_request::{FriendRequestRespository, FriendRqInput}, notifications::NotificationRespository, posts::PostUploadForm, users::{InfoUser, PasswordInput, UserRespository}};
use respositories::posts::PostResponsitory;
use rocket_cors::AllowedOrigins;
use authorization::BearerToken;
use crate::aws_s3::AwsS3;

#[database("social_media")]
pub struct DbConn(diesel::PgConnection);

#[post("/login", format = "json", data = "<user>")]
async fn sign_in(db: DbConn, user: Json<Login>) -> Result<Value, Custom<Value>> {
    db.run(|c|{
        let result = UserRespository::handle_login(c, user.into_inner());
        result
        .map(|user| json!({"status": "success", "user": user}))
        .map_err(|e| Custom(Status::InternalServerError, json!({"status": "failed", "error": e.to_string()})))
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
async fn user_from_id(db: DbConn, _auth: BearerToken, id: i32) -> Result<Value, Custom<Value>>{
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
}

#[post("/upload/avatar", data = "<data>")]
async fn upload_avatar(db: DbConn, _auth: BearerToken, data: Form<FileUploadForm<'_>>) -> Result<Value, Custom<Value>> {
    let res = AwsS3::handle_file_s3(&data.file, data.r#type).await;
    match res {
        Ok(url) => {
            db.run(move |c| {
                let user = BearerToken::get_user(&_auth, c);
                match user {
                    Ok(user) => UserRespository::save_avatar(c, user.id, url)
                        .map(|user| json!(user))
                        .map_err(|e| Custom(Status::InternalServerError, json!({"error": e.to_string()}))),
                    Err(_) => Err(Custom(Status::InternalServerError, json!({"error": "Failed to upload to S3"})))
                }
            })
            .await
        },
        Err(_) => Err(Custom(Status::InternalServerError, json!({"error": "Failed to upload to S3"}))),
    }
}

#[post("/upload/background", data = "<data>")]
async fn upload_background(db: DbConn, _auth: BearerToken, data: Form<FileUploadForm<'_>>) -> Result<Value, Custom<Value>> {
    let res = AwsS3::handle_file_s3(&data.file, data.r#type).await;
    match res {
        Ok(url) => {
            db.run(move |c| {
                let user = BearerToken::get_user(&_auth, c);
                match user {
                    Ok(user) => UserRespository::save_background(c, user.id, url)
                        .map(|user| json!(user))
                        .map_err(|e| Custom(Status::InternalServerError, json!({"error": e.to_string()}))),
                    Err(_) => Err(Custom(Status::InternalServerError, json!({"error": "Failed to upload to S3"})))
                }
            })
            .await
        },
        Err(_) => Err(Custom(Status::InternalServerError, json!({"error": "Failed to upload to S3"}))),
    }
}

#[post("/upload/post", data = "<data>")]
async fn upload_post(db: DbConn, _auth: BearerToken, data: Form<PostUploadForm<'_>>) -> Result<Value, Custom<Value>> {
    let res = PostResponsitory::create_post(db, _auth, data).await;
    match res {
        Ok(post) => Ok(json!(post)),
        Err(_) => Err(Custom(Status::InternalServerError, json!({"error": "Failed to upload post"})))
    }
}

#[post("/upload/secret", data = "<data>")]
async fn upload_secret_post(db: DbConn, _auth: BearerToken, data: Form<PostUploadForm<'_>>) -> Result<Value, Custom<Value>> {
    let res = PostResponsitory::create_secret_post(db, _auth, data).await;
    match res {
        Ok(post) => Ok(json!(post)),
        Err(_) => Err(Custom(Status::InternalServerError, json!({"error": "Failed to upload post"})))
    }
}

#[get("/post/<id>")]
async fn post_from_id(db: DbConn, _auth: BearerToken, id: i32) -> Result<Value, Custom<Value>> {
    db.run(move |c|{
        let res = PostResponsitory::get_post_from_id(c, id);
        res
        .map(|post| json!(post))
        .map_err(|e| Custom(Status::InternalServerError, json!({"error": e.to_string()})))
    }).await 
}

#[post("/post/like", data = "<data>")]
async fn like_post(db: DbConn, _auth: BearerToken, data: Json<DataId>) -> Result<Value, Custom<Value>> {
    let res = PostResponsitory::handle_like(db, _auth, data.id).await;
    match res {
        Ok(res) => Ok(json!(res)),
        Err(_) => Err(Custom(Status::InternalServerError, json!({"error": "Failed to like"})))
    }
} 

#[post("/upload/comment", data="<data>")]
async fn upload_comment(db: DbConn, _auth: BearerToken, data: Json<CommentInput>) -> Result<Value, Custom<Value>> {
    db.run(|c|{
        let res = CommentRespository::create_comment(c, _auth, data.into_inner());
        res
        .map(|res| json!(res))
        .map_err(|e| Custom(Status::InternalServerError, json!({"error": e.to_string()})))
    }).await
}

#[get("/comment/<id>")]
async fn comments_from_post_id(db: DbConn, _auth: BearerToken, id: i32) -> Result<Value, Custom<Value>> {
    db.run(move |c|{
        let res = CommentRespository::get_comments(c, _auth, id);
        res
        .map(|res| json!(res))
        .map_err(|e| Custom(Status::InternalServerError, json!({"error": e.to_string()})))
    }).await 
}

#[post("/add-friend", data="<data>")]
async fn add_friend(db: DbConn, _auth: BearerToken, data: Json<FriendRqInput>) -> Result<Value, Custom<Value>> {
    db.run(|c: &mut diesel::PgConnection|{
        let res = FriendRequestRespository::create_request(c, _auth, data.into_inner());
        res
        .map(|res| json!(res))
        .map_err(|e| Custom(Status::InternalServerError, json!({"error": e.to_string()})))
    }).await
}

#[get("/check/add-friend/<id>")]
async fn check_add_friend(db: DbConn, _auth: BearerToken, id: i32) -> Result<Value, Custom<Value>> {
    db.run(move |c|{
        let res = FriendRequestRespository::check_request(c, _auth, id);
        res
        .map(|res| json!(res))
        .map_err(|e| Custom(Status::InternalServerError, json!({"error": e.to_string()})))
    }).await 
}

#[get("/friend-request")]
async fn friend_request(db: DbConn, _auth: BearerToken) -> Result<Value, Custom<Value>> {
    db.run(|c: &mut diesel::PgConnection|{
        let res = FriendRequestRespository::get_friend_request(c, _auth);
        res
        .map(|res| json!(res))
        .map_err(|e| Custom(Status::InternalServerError, json!({"error": e.to_string()})))
    }).await
}

#[post("/confirm-request/<id>")]
async fn confirm_add_friend(db: DbConn, _auth: BearerToken, id: i32) -> Result<Value, Custom<Value>> {
    db.run(move |c|{
        let res = FriendRequestRespository::confirm_request(c, _auth, id);
        res
        .map(|res| json!(res))
        .map_err(|e| Custom(Status::InternalServerError, json!({"error": e.to_string()})))
    }).await 
}

#[post("/delete-friend/<id>")]
async fn delete_friend(db: DbConn, _auth: BearerToken, id: i32) -> Result<Value, Custom<Value>> {
    db.run(move |c|{
        let res = FriendRequestRespository::delete_request(c, _auth, id);
        res
        .map(|res| json!(res))
        .map_err(|e| Custom(Status::InternalServerError, json!({"error": e.to_string()})))
    }).await 
}

#[get("/posts/following")]
async fn get_following_post(db: DbConn, _auth: BearerToken) -> Result<Value, Custom<Value>> {
    db.run(move |c|{
        let res = PostResponsitory::get_posts(c, _auth);
        res
        .map(|res| json!(res))
        .map_err(|e| Custom(Status::InternalServerError, json!({"error": e.to_string()})))
    }).await 
}

#[get("/posts/secret")]
async fn get_secret_post(db: DbConn, _auth: BearerToken) -> Result<Value, Custom<Value>> {
    db.run(move |c|{
        let res = PostResponsitory::get_secret_posts(c, _auth);
        res
        .map(|res| json!(res))
        .map_err(|e| Custom(Status::InternalServerError, json!({"error": e.to_string()})))
    }).await 
}

#[post("/search/user", format = "json", data="<search>")]
async fn search_user_from_name(db: DbConn, _auth: BearerToken, search: Json<SearchName>) -> Result<Value, Custom<Value>> {
    db.run(move |c|{
        let res = UserRespository::search_from_name(c, search.search_name.clone());
        res
        .map(|res| json!(res))
        .map_err(|e| Custom(Status::InternalServerError, json!({"error": e.to_string()})))
    }).await
}

#[get("/friends")]
async fn get_friends(db: DbConn, _auth: BearerToken) -> Result<Value, Custom<Value>> {
    db.run(move |c|{
        let res = FriendRequestRespository::get_friends_of_user(c, _auth);
        res
        .map(|res| json!(res))
        .map_err(|e| Custom(Status::InternalServerError, json!({"error": e.to_string()})))
    }).await 
}

#[post("/update/user-info", format = "json", data="<info_user>")]
async fn update_user(db: DbConn, _auth: BearerToken, info_user: Json<InfoUser>) -> Result<Value, Custom<Value>> {
    db.run(move |c|{
        let res = UserRespository::update_user_info(c, _auth, info_user.into_inner());
        res
        .map(|res| json!(res))
        .map_err(|e| Custom(Status::InternalServerError, json!({"error": e.to_string()})))
    }).await
}

#[post("/update/change-password", format = "json", data="<password_input>")]
async fn handle_change_password(db: DbConn, _auth: BearerToken, password_input: Json<PasswordInput>) -> Result<Value, Custom<Value>> {
    db.run(move |c|{
        let res = UserRespository::change_password(c, _auth, password_input.into_inner());
        res
        .map(|res| json!(res))
        .map_err(|e| Custom(Status::InternalServerError, json!({"error": e.to_string()})))
    }).await
}

#[get("/notifications")]
async fn get_notifications(db: DbConn, _auth: BearerToken) -> Result<Value, Custom<Value>> {
    db.run(move |c|{
        let res = NotificationRespository::get_notifications(c, _auth);
        res
        .map(|res| json!(res))
        .map_err(|e| Custom(Status::InternalServerError, json!({"error": e.to_string()})))
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
        authorize,
        user_from_id,
        upload_avatar,
        upload_background,
        upload_post,
        post_from_id,
        like_post,
        upload_comment,
        comments_from_post_id,
        add_friend,
        check_add_friend,
        confirm_add_friend,
        delete_friend,
        get_following_post,
        upload_secret_post,
        get_secret_post,
        search_user_from_name,
        friend_request,
        get_friends,
        update_user,
        handle_change_password,
        get_notifications
    ])
    .register("/", catchers![
        not_found,
        unprocessable_entity
    ])
    .attach(DbConn::fairing())
    .attach(cors)
}