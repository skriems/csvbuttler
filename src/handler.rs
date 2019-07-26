use actix_web::{http::StatusCode, web, Error, HttpResponse, Responder};
use futures::future::{ok, Future};
use std::env;
use std::sync::{Arc, Mutex};

use crate::data;

pub fn index() -> impl Responder {
    let rust = env::var("RUST").unwrap_or("env var not set".to_string());
    HttpResponse::Ok().body(format!("Rust {}", rust))
}

pub fn product(
    path: web::Path<(usize,)>,
    data: web::Data<Arc<Mutex<data::AppState>>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let map = &data.lock().unwrap().map;
    if let Some(product) = map.get(&path.0) {
        ok(HttpResponse::Ok().json(product))
    } else {
        ok(HttpResponse::new(StatusCode::NOT_FOUND))
    }
}
