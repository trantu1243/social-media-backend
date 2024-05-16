use diesel::{ ExpressionMethods, PgConnection, QueryResult, RunQueryDsl};
use diesel::QueryDsl;
use rocket::request::{FromRequest, Request, Outcome};
use rocket::http::Status;
use serde::{Deserialize, Serialize};
use crate::jwt::JWTtoken;
use crate::models::SafeUser;
use crate::schema::users;

#[derive(Debug, Serialize, Deserialize)]
pub struct BearerToken {
    pub username: String,       
    pub exp: usize,          
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BearerToken{
    type Error = ();

    async fn from_request(requset: &'r Request<'_>) ->Outcome<Self, Self::Error> {
        let auth_header = requset.headers().get_one("Authorization");

        if let Some(auth_header) = auth_header {
            if let Some(auth) = Self::from_authorization_header(auth_header) {
                return Outcome::Success(auth);
            }
        }

        Outcome::Error((Status::Unauthorized, ()))
    }
}

impl BearerToken{
    fn from_authorization_header(auth_header: &str)->Option<BearerToken>{
        let split = auth_header.split_whitespace().collect::<Vec<_>>();

        if split.len() != 2 {
            return None;
        }

        if split[0] != "Bearer" {
            return None;
        }

        Self::from_jwt(split[1])
    }

    fn from_jwt(token: &str)->Option<BearerToken>{
        match JWTtoken::verify_jwt(token) {
            Ok(data) => {
                Some(data.claims)
            },
            Err(_) => {
                return None;
            }
        } 
    }

    pub fn get_user(&self, c: &mut PgConnection) -> QueryResult<SafeUser>{
        let safe_user = users::table.select((
                users::id,
                users::name,
                users::about,
                users::avatar,
                users::background,
                users::photo,
                users::postid,
                users::followerid,
                users::followingid,
            )).filter(users::username.eq(self.username.clone()))
            .first::<SafeUser>(c)?;
        Ok(safe_user)
    }  
    
}