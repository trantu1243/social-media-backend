
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};

use dotenv::dotenv;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    username: String,       
    exp: usize,          
}

pub struct JWTtoken;

impl JWTtoken {
    pub fn create_jwt(username: String) -> Result<String, jsonwebtoken::errors::Error> {
        dotenv().ok();
        let now = Utc::now();
        let expiration = now + Duration::days(30);
        let secret_key = env::var("SECRET_KEY").expect("Secret key must be set");
    
        let claims = Claims {
            username: username.to_owned(),
            exp: expiration.timestamp() as usize
        };
        encode(&Header::new(Algorithm::HS256), &claims, &EncodingKey::from_secret(&secret_key.as_ref()))
    }
}
