use actix_identity::Identity;
use actix_web::{dev::Payload, Error, FromRequest, HttpRequest, HttpResponse};
use csrf_token::CsrfTokenGenerator;
use serde::{Deserialize, Serialize};

use crate::error::Error as ServiceError;
use crate::jwt::{decode_token, Claims};

// We're using a struct so we can implement a conversion from
// Claims to SlimUser, useful in the decode function.
#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub email: String,
    pub company: String,
}

impl FromRequest for SlimUser {
    type Error = Error;
    type Future = Result<SlimUser, Error>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let generator = req
            .app_data::<CsrfTokenGenerator>()
            .ok_or(ServiceError::InternalServerError)?;

        let csrf_token = req
            .headers()
            .get("x-csrf-token")
            .ok_or(ServiceError::Unauthorized)?;

        let decoded_token = hex::decode(&csrf_token)
            .map_err(|err| HttpResponse::InternalServerError().json(err.to_string()))?;

        generator
            .verify(&decoded_token)
            .map_err(|_| ServiceError::Unauthorized)?;

        if let Some(identity) = Identity::from_request(req, payload)?.identity() {
            let user: SlimUser = decode_token(&identity)?;
            return Ok(user);
        }
        Err(ServiceError::Unauthorized.into())
    }
}

impl From<Claims> for SlimUser {
    fn from(claims: Claims) -> Self {
        SlimUser {
            email: claims.sub,
            company: claims.company,
        }
    }
}

#[derive(Deserialize)]
pub struct AuthUser {
    pub email: String,
    pub password: String,
}

impl AuthUser {
    pub fn login(&self) -> Result<SlimUser, ServiceError> {
        let email = "foo@bar.com".to_owned();
        let company = "Foo Inc.".to_owned();
        Ok(SlimUser { email, company })
    }
}

