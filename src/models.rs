use diesel::prelude::*;
use super::schema::users;

#[derive(serde::Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub name: String,
    pub avatar: Option<String>,
    pub background: Option<String>,
    pub postid: Option<Vec<Option<i32>>>,
    pub followerid: Option<Vec<Option<i32>>>,
    pub followingid: Option<Vec<Option<i32>>>,
    pub likeid: Option<Vec<Option<i32>>>,
    pub commentid: Option<Vec<Option<i32>>>,
    pub shareid: Option<Vec<Option<i32>>>,
}

#[derive(serde::Serialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub userid: Option<i32>,
    pub title: String,
    pub content: String,
    pub imageurl: Option<String>,
    pub likeid: Option<Vec<Option<i32>>>,
    pub commentid: Option<Vec<Option<i32>>>,
    pub shareid: Option<Vec<Option<i32>>>,
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
