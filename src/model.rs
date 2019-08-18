//! Module holding the model that is used to deserialize rows

use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Deserialize, Serialize)]
pub struct Product {
    pub id: usize,
    pub title: String,
    pub description: Option<String>,
    pub brand: String,
    pub price: String,
}

impl Responder for Product {
    type Error = Error;
    type Future = Result<HttpResponse, Error>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self)?;

        // Create response and set content type
        Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .body(body))
    }
}

pub fn error_product() -> Product {
    Product {
        id: 0,
        title: "Foo".to_string(),
        description: None,
        brand: "Foo".to_string(),
        price: "Bar".to_string(),
    }
}
