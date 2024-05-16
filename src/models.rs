use diesel::prelude::*;
use super::schema::users;

#[derive(serde::Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub name: String,
    pub about: Option<String>,
    pub avatar: Option<String>,
    pub background: Option<String>,
    pub photo: Option<Vec<Option<String>>>,
    pub post_id: Option<Vec<Option<i32>>>,
    pub follower_id: Option<Vec<Option<i32>>>,
    pub following_id: Option<Vec<Option<i32>>>,
    pub like_id: Option<Vec<Option<i32>>>,
    pub comment_id: Option<Vec<Option<i32>>>,
    pub share_id: Option<Vec<Option<i32>>>,
    pub notifications: Option<Vec<Option<i32>>>,
    pub checknotification: Option<bool>,
    pub blocklist: Option<Vec<Option<i32>>>
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub name: String,
}

#[derive(serde::Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String
}

#[derive(serde::Serialize, Queryable)]
pub struct SafeUser {
    id: i32,
    name: String,
    about: Option<String>,
    avatar: Option<String>,
    background: Option<String>,
    photo: Option<Vec<Option<String>>>,
    postid: Option<Vec<Option<i32>>>,
    followerid: Option<Vec<Option<i32>>>,
    followingid: Option<Vec<Option<i32>>>
}

#[derive(serde::Serialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub user_id: Option<i32>,
    pub name: Option<String>,
    pub avatar_user: Option<String>,
    pub content: Option<String>,
    pub post_date: Option<chrono::NaiveDateTime>,
    pub interact_date: Option<chrono::NaiveDateTime>,
    pub image: Option<String>,
    pub like_id: Option<Vec<Option<i32>>>,
    pub comment_id: Option<Vec<Option<i32>>>,
    pub share_id: Option<Vec<Option<i32>>>,
}