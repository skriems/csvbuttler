use crate::data;
use crate::error::Error as ServiceError;
use crate::jwt;
use crate::user;
use actix_identity::Identity;
use actix_web::{http::StatusCode, web, Error, HttpResponse, Responder};
use csrf_token::CsrfTokenGenerator;
use futures::future::{ok, Future};
use hex;
use std::env;
use std::sync::{Arc, Mutex};

/// Dummy root handler
pub fn index() -> impl Responder {
    let rust = env::var("RUST").unwrap_or("env var not set".to_string());
    HttpResponse::Ok().body(format!("Rust {}", rust))
}

/// Asynchronous product handler
pub fn product(
    path: web::Path<(usize,)>,
    data: web::Data<Arc<Mutex<data::AppState>>>,
    auth: user::SlimUser,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let map = &data.lock().unwrap().map;
    dbg!("auth: {:?}", auth);
    if let Some(product) = map.get(&path.0) {
        ok(HttpResponse::Ok().json(product))
    } else {
        ok(HttpResponse::new(StatusCode::NOT_FOUND))
    }
}

pub fn login(
    auth_user: web::Json<user::AuthUser>,
    data: web::Data<Arc<Mutex<data::AppState>>>,
    id: Identity,
    generator: web::Data<CsrfTokenGenerator>,
) -> Result<HttpResponse, HttpResponse> {
    let user = auth_user.login().map_err(|e| match e {
        ServiceError::Unauthorized => HttpResponse::NotFound().json(e.to_string()),
        _ => HttpResponse::InternalServerError().json(e.to_string()),
    })?;

    // This is the jwt token we will send in a cookie.
    let secret = &data.lock().unwrap().settings.secrets.jwt;
    let token = jwt::create_token(&user.email, &user.company, &secret.as_bytes())?;

    id.remember(token);

    // Finally our response will have a csrf token for security.
    let response = HttpResponse::Ok()
        .header("X-CSRF-TOKEN", hex::encode(generator.generate()))
        .json(user);
    Ok(response)
}

pub fn logout(id: Identity) -> Result<HttpResponse, HttpResponse> {
    id.forget();
    Ok(HttpResponse::Ok().into())
}
