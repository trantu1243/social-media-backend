use diesel::{ ExpressionMethods, PgConnection, QueryResult, RunQueryDsl};
use diesel::QueryDsl;
use crate::{models::{Login, NewUser, User, SafeUser}, schema::users};
use scrypt::{ScryptParams, scrypt_simple, scrypt_check};
use crate::jwt::JWTtoken;

#[derive(serde::Serialize)]
pub struct LoginRes {
    user: SafeUser,
    token: String
}

pub struct UserRespository;

impl UserRespository {
    pub fn create_user(c: &mut PgConnection, new_user: NewUser) -> QueryResult<LoginRes> {
        let cloned_username = new_user.username.clone();
        let clone_password = new_user.password.clone();

        let params = ScryptParams::new(8, 4, 1).unwrap();
        let hasded_password = scrypt_simple(&clone_password, &params).unwrap();

        let user = NewUser{
            username: new_user.username.clone(),
            password: hasded_password,
            name: new_user.name.clone()
        };
        diesel::insert_into(users::table)
        .values(user)
        .execute(c)?;
        
        let text = new_user.username.clone();
        match JWTtoken::create_jwt(text) {
            Ok(token) => {
                let safe_user = users::table.select((
                    users::id,
                    users::username,
                    users::name,
                    users::avatar,
                    users::background,
                    users::postid,
                    users::followerid,
                    users::followingid,
                    users::likeid,
                    users::commentid,
                    users::shareid,
                    users::notifications,
                    users::checknotification
                )).filter(users::username.eq(cloned_username))
                .first::<SafeUser>(c)?;

                let response = LoginRes{
                    user: safe_user,
                    token: "Bearer ".to_string() + &token
                };
                Ok(response)
            },
            Err(_) =>  Err(diesel::NotFound),
        }
    }

    pub fn handle_login(c: &mut PgConnection, login: Login) -> QueryResult<LoginRes> {
        let username = login.username.clone();
        let password = login.password.clone();
        let user = users::table.filter(users::username.eq(username))
        .first::<User>(c)?;
        let text = login.username.clone();
        match scrypt_check(&password, &user.password) {
            Ok(_) => {
                match JWTtoken::create_jwt(text) {
                    Ok(token) => {
                        let id = user.id.clone();
                        let safe_user = users::table.select((
                            users::id,
                            users::username,
                            users::name,
                            users::avatar,
                            users::background,
                            users::postid,
                            users::followerid,
                            users::followingid,
                            users::likeid,
                            users::commentid,
                            users::shareid,
                            users::notifications,
                            users::checknotification
                        ))
                        .find(id)
                        .first::<SafeUser>(c)?;
                        let response = LoginRes{
                            user: safe_user,
                            token: "Bearer ".to_string() + &token
                        };
                        Ok(response)
                    },
                    Err(_) =>  Err(diesel::NotFound),
                }
            },
            Err(_) => Err(diesel::NotFound)
        }
    }
}