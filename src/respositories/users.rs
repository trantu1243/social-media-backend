use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, PgConnection, QueryResult, RunQueryDsl};
use crate::{models::{Login, NewUser, User}, schema::users};

use scrypt::{ScryptParams, scrypt_simple, scrypt_check};

pub struct UserRespository;

impl UserRespository {
    pub fn create_user(c: &mut PgConnection, new_user: NewUser) -> QueryResult<User> {
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
        let result = users::table.filter(users::username.eq(cloned_username))
        .first::<User>(c);
        result
    }

    pub fn handle_login(c: &mut PgConnection, login: Login) -> QueryResult<User> {
        let username = login.username.clone();
        let password = login.password.clone();
        let user = users::table.filter(users::username.eq(username))
        .first::<User>(c)?;
        match scrypt_check(&password, &user.password) {
            Ok(_) => Ok(user),
            Err(_) => Err(diesel::NotFound)
        }
    }
}