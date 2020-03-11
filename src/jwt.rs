use std::env;

use actix_web::HttpResponse;
use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::user::SlimUser;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // company
    pub company: String,
    // subject
    pub sub: String,
    // expiry
    pub exp: usize,
}

// struct to get converted to token and back
impl Claims {
    fn with_email(email: &str, company: &str) -> Self {
        Claims {
            company: company.into(),
            sub: email.into(),
            exp: (Local::now() + Duration::hours(24)).timestamp() as usize,
        }
    }
}

pub fn create_token(email: &str, company: &str, secret: &[u8]) -> Result<String, HttpResponse> {
    let claims = Claims::with_email(email, company);
    encode(&Header::default(), &claims, secret)
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn decode_token(token: &str) -> Result<SlimUser, HttpResponse> {
    let secret = env::var("APP_SECRETS_JWT").map_err(|_| HttpResponse::InternalServerError())?;
    decode::<Claims>(token, secret.as_bytes(), &Validation::default())
        .map(|data| data.claims.into())
        .map_err(|e| HttpResponse::Unauthorized().json(e.to_string()))
}
