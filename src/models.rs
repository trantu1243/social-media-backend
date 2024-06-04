use chrono::{DateTime, Utc};
use diesel::prelude::*;
use super::schema::{users, posts};

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
    pub id: i32,
    pub name: String,
    pub about: Option<String>,
    pub avatar: Option<String>,
    pub background: Option<String>,
    pub photo: Option<Vec<Option<String>>>,
    pub postid: Option<Vec<Option<i32>>>,
    pub followerid: Option<Vec<Option<i32>>>,
    pub followingid: Option<Vec<Option<i32>>>
}

#[derive(Queryable, Selectable, serde::Serialize, serde::Deserialize)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub userid: Option<i32>,
    pub name: Option<String>,
    pub avatar_user: Option<String>,
    pub content: Option<String>,
    pub post_date: Option<DateTime<Utc>>,
    pub interact_date: Option<DateTime<Utc>>,
    pub image: Option<Vec<Option<String>>>,
    pub likeid: Option<Vec<Option<i32>>>,
    pub commentid: Option<Vec<Option<i32>>>,
    pub shareid: Option<Vec<Option<i32>>>,
    pub secret: Option<bool>,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub userid: Option<i32>,
    pub name: Option<String>,
    pub avatar_user: Option<String>,
    pub content: Option<String>,
    pub image: Option<Vec<Option<String>>>,
}

#[derive(serde::Deserialize)]
pub struct DataId {
    pub id: i32
}