use std::str::FromStr;
use jsonwebtoken::{encode, decode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration}; 
use anyhow::Result;
use crate::config::Config;
use crate::core::constants::security::{
    JWT_ALGORITHM,
    JWT_ISSUER,
    TOKEN_EXPIRATION_BUFFER,
};
use cuid::cuid2; 

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,    
    pub exp: i64,   
    pub iat: i64,     
    pub iss: String,   
    pub jti: String, 
}

impl Claims {
    pub fn new(user_id: String, duration: Duration) -> Self {
        let now = Utc::now();
        Self {
            sub: user_id,
            iat: now.timestamp(),
            exp: (now + duration).timestamp(),
            iss: JWT_ISSUER.to_string(),
            jti: cuid2().to_string(),
        }
    }
}

pub struct TokenManager {
    access_token_secret: String,
    refresh_token_secret: String,
}

impl TokenManager {
    pub fn new(config: &Config) -> Self {
        Self {
            access_token_secret: config.jwt.access_token_secret.clone(),
            refresh_token_secret: config.jwt.refresh_token_secret.clone(),
        }
    }

    pub fn create_access_token(&self, user_id: String) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = Claims::new(user_id, Duration::hours(1));
        self.encode_jwt(&claims, &self.access_token_secret)
    }

    pub fn create_refresh_token(&self, user_id: String) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = Claims::new(user_id, Duration::days(7));
        self.encode_jwt(&claims, &self.refresh_token_secret)
    }


    pub fn verify_access_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        self.decode_jwt(token, &self.access_token_secret)
    }

    pub fn verify_refresh_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        self.decode_jwt(token, &self.refresh_token_secret)
    }

    fn encode_jwt(&self, claims: &Claims, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let mut header = Header::default();
        header.alg = Algorithm::from_str(JWT_ALGORITHM).unwrap_or(Algorithm::HS256);
        
        encode(
            &header,
            claims,
            &EncodingKey::from_secret(secret.as_ref())
        )
    }

    fn decode_jwt(&self, token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let mut validation = Validation::new(Algorithm::from_str(JWT_ALGORITHM).unwrap_or(Algorithm::HS256));
        validation.set_issuer(&[JWT_ISSUER]);
        validation.leeway = TOKEN_EXPIRATION_BUFFER as u64;
        
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation
        )
        .map(|data| data.claims)
    }
}
