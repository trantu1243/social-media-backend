use diesel::{ ExpressionMethods, PgConnection, QueryResult, RunQueryDsl, TextExpressionMethods};
use diesel::QueryDsl;
use social_media_backend::schema::posts;
use crate::authorization::BearerToken;
use crate::models::SearchUser;
use crate::schema::comments;
use crate::{models::{Login, NewUser, User, SafeUser}, schema::users};
use scrypt::{ScryptParams, scrypt_simple, scrypt_check};
use crate::jwt::JWTtoken;

#[derive(serde::Serialize)]
pub struct LoginRes {
    user: SafeUser,
    token: String
}

#[derive(serde::Deserialize)]
pub struct InfoUser {
    pub name: String,
    pub about: String
}

#[derive(serde::Deserialize)]
pub struct PasswordInput {
    pub password: String,
    pub new_password: String,
    pub confirm_password: String
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
                    users::name,
                    users::about,
                    users::avatar,
                    users::background,
                    users::photo,
                    users::postid,
                    users::followerid,
                    users::followingid
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
                            users::name,
                            users::about,
                            users::avatar,
                            users::background,
                            users::photo,
                            users::postid,
                            users::followerid,
                            users::followingid
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

    pub fn get_user_info(c: &mut PgConnection, id: i32) -> QueryResult<SafeUser> {
        let safe_user = users::table.select((
            users::id,
            users::name,
            users::about,
            users::avatar,
            users::background,
            users::photo,
            users::postid,
            users::followerid,
            users::followingid
        ))
        .find(id)
        .first::<SafeUser>(c)?;
    Ok(safe_user)
    }

    pub fn save_avatar(c: &mut PgConnection, id: i32, url: String) -> QueryResult<String> {     
        let result = diesel::update(users::table.find(id)).set(users::avatar.eq(url.clone()))
        .execute(c)?;
        diesel::update(posts::table.filter(posts::userid.eq(id))).set(posts::avatar_user.eq(url.clone()))
        .execute(c)?;
        diesel::update(comments::table.filter(comments::userid.eq(id))).set(comments::avatar_user.eq(url))
        .execute(c)?;
        Ok(result.to_string())
    }

    pub fn save_background(c: &mut PgConnection, id: i32, url: String) -> QueryResult<String> {
        let result = diesel::update(users::table.find(id)).set(users::background.eq(url))
        .execute(c)?;
        Ok(result.to_string())
    }

    pub fn search_from_name(c: &mut PgConnection, search_name: String) -> QueryResult<Vec<SearchUser>> {
        users::table.select((
            users::id,
            users::name,
            users::about,
            users::avatar,
            users::followerid
        ))
        .filter(users::name.like(format!("%{}%", search_name)))
        .limit(5)
        .load::<SearchUser>(c)
    }

    pub fn update_user_info(c: &mut PgConnection, _auth: BearerToken, info_user: InfoUser) -> QueryResult<String> {
        let user = BearerToken::get_user(&_auth, c)?;
        let result = diesel::update(users::table.find(user.id)).set((
            users::name.eq(info_user.name),
            users::about.eq(info_user.about)
        )).execute(c)?;
        Ok(result.to_string())
    }

    pub fn change_password(c: &mut PgConnection, _auth: BearerToken, password_input: PasswordInput) -> QueryResult<String> {
        let password = password_input.password.clone();
        let user = BearerToken::get_user(&_auth, c)?;
        let origin_password = users::table.select(users::password).find(user.id).first::<String>(c)?;
        match scrypt_check(&password, &origin_password) {
            Ok(_) => {
                if password_input.new_password == password_input.confirm_password {
                    let params = ScryptParams::new(8, 4, 1).unwrap();
                    let hasded_password = scrypt_simple(&password_input.new_password, &params).unwrap();
                    let result = diesel::update(users::table.find(user.id)).set(
                        users::password.eq(hasded_password)
                    ).execute(c)?;
                    Ok(result.to_string())
                } else {
                    Err(diesel::result::Error::BrokenTransactionManager)
                }
            },
            Err(_) => Err(diesel::result::Error::BrokenTransactionManager)
        }
    }
}