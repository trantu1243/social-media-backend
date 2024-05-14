
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey, TokenData};
use chrono::{Utc, Duration};

use dotenv::dotenv;
use std::env;

use crate::authorization::BearerToken;

pub struct JWTtoken;

impl JWTtoken {
    pub fn create_jwt(username: String) -> Result<String, jsonwebtoken::errors::Error> {
        dotenv().ok();
        let now = Utc::now();
        let expiration = now + Duration::days(30);
        let secret_key = env::var("SECRET_KEY").expect("Secret key must be set");
    
        let claims = BearerToken {
            username: username.to_owned(),
            exp: expiration.timestamp() as usize
        };
        encode(&Header::new(Algorithm::HS256), &claims, &EncodingKey::from_secret(&secret_key.as_ref()))
    }

    pub fn verify_jwt(token: &str) -> Result<TokenData<BearerToken>, jsonwebtoken::errors::Error> {
        dotenv().ok();
        let secret_key = env::var("SECRET_KEY").expect("Secret key must be set");
        let decoding_key = DecodingKey::from_secret(secret_key.as_ref());
        decode::<BearerToken>(token, &decoding_key, &Validation::default())
    }
}
