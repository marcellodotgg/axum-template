use serde::{Deserialize, Serialize};
use std::{env, time::{SystemTime, UNIX_EPOCH}};
use serde::de::DeserializeOwned;
use jsonwebtoken::{decode, encode, errors::Error, DecodingKey, EncodingKey, Header, Validation};

pub struct Jwt {}

impl Jwt {
    pub fn encode<T: Serialize>(value: T) -> Result<String, Error> {
        let secret = env::var("JWT_SECRET").expect("JWT Secret was not set");

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;
        let expiration = now + (60 * 60 * 24 * 365);
        let claims = Claims {
            sub: value,
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
    }

    pub fn decode<T>(token: &str) -> Result<Claims<T>, Error>
    where
        T: DeserializeOwned,
    {
        let secret = env::var("JWT_SECRET").expect("JWT Secret was not set");
        let decoding_key = DecodingKey::from_secret(secret.as_ref());
        let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        decode::<Claims<T>>(token, &decoding_key, &validation).map(|data| data.claims)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims<T> {
    pub sub: T,
    pub exp: usize,
}