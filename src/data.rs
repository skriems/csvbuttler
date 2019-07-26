use std::collections::HashMap;
use std::io;

use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug)]
pub struct AppState {
    pub map: HashMap<usize, Product>,
}

impl AppState {
    pub fn from_map(map: HashMap<usize, Product>) -> Self {
        AppState { map }
    }
}

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

fn error_product() -> Product {
    Product {
        id: 0,
        title: "Foo".to_string(),
        description: None,
        brand: "Foo".to_string(),
        price: "Bar".to_string(),
    }
}

pub fn read_data() -> io::Result<HashMap<usize, Product>> {
    let mut map = HashMap::new();

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path("./feed.csv")?;

    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic deserialization.  And by
        // needing to do that, we seem to have no idiomatic way of skipping bogus lines :( Hence
        // we print out the error here for now and return a dummy Product that is used to continue
        // the loop
        let record: Product = result.unwrap_or_else(|e| {
            println!("{}", e);
            error_product()
        });
        if record.id == 0 {
            continue;
        };
        map.insert(record.id, record);
    }
    Ok(map)
}
