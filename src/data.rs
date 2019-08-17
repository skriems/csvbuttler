use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

use crate::config::{get_config, Config};
use crate::error;
use crate::model::{error_product, Product};

/// type alias for `AppState`
pub type StateType = Arc<Mutex<AppState>>;
use actix_rt::System;
use actix_web::client::Client;
use futures::future::{lazy, Future};

#[derive(Debug)]
pub struct AppState {
    pub cfg: Config,
    pub map: HashMap<usize, Product>,
}

impl AppState {
    pub fn new() -> Result<StateType, error::Error> {
        let cfg = get_config()?;
        let map = read_data(&cfg)?;

        Ok(Arc::new(Mutex::new(AppState { cfg, map })))
    }
}

fn read_csv(cfg: &Config) -> Result<String, error::Error> {
    if cfg.is_local() {
        if let Some(filename) = &cfg.file {
            let mut file = File::open(filename)?;
            let mut s = String::new();
            file.read_to_string(&mut s)?;
            Ok(s)
        } else {
            Err(error::Error)
        }
    } else {
        if let Some(url) = &cfg.file {
            let data = fetch_data(url)?;
            Ok(data)
        } else {
            Err(error::Error)
        }
    }
}

pub fn read_data(cfg: &Config) -> io::Result<HashMap<usize, Product>> {
    let mut map = HashMap::new();

    let data = read_csv(&cfg)?;

    let mut rdr = csv::ReaderBuilder::new()
        // TODO this can panic if an empty string is provided as delimiter
        .delimiter(cfg.delimiter.clone().into_bytes()[0])
        .from_reader(data.as_bytes());

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

pub fn fetch_data(url: &str) -> Result<String, error::Error> {
    System::new("test").block_on(lazy(|| {
        println!("Fetching {}", &url);
        let client = Client::default();
        client
            .get(url) // <- Create request builder
            .header("User-Agent", "Actix-web")
            .send() // <- Send http request
            .map_err(|_| error::Error)
            .and_then(|mut res| {
                res.body()
                    .and_then(move |bytes| {
                        let s = std::str::from_utf8(&bytes).expect("utf8 parse error)");
                        Ok(s.to_owned())
                    })
                    .map_err(|_| error::Error)
            })
            .map_err(|_| error::Error)
    }))
}
