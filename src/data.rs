use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

use crate::config::{get_config, Config};
use crate::error::{Error, ErrorKind};
use crate::model::{error_product, Product};

use reqwest;

/// type alias for `AppState`
pub type StateType = Arc<Mutex<AppState>>;

#[derive(Debug)]
/// The `AppState` is constructed with the app configuration and the generated `HashMap` from the
/// csv data.
pub struct AppState {
    pub cfg: Config,
    pub map: HashMap<usize, Product>,
}

/// `AppState` implements a `new` function for convenience.
/// Note that this should only be done once. Subsequent updates to the `map` should be done via
/// mutable references to the `Mutex` inside of the `Arc`.
impl AppState {
    pub fn new() -> Result<StateType, Error> {
        let cfg = get_config()?;
        println!("{:?}", &cfg);
        let csv = get_csv(&cfg)?;
        let map = parse_csv(&cfg, csv)?;

        Ok(Arc::new(Mutex::new(AppState { cfg, map })))
    }
}

/// Retrieve the csv either from a local file, or try to fetch it, from an external service
fn get_csv(cfg: &Config) -> Result<String, Error> {
    if cfg.is_local() {
        if let Some(filename) = &cfg.file {
            let mut file = File::open(filename)?;
            let mut s = String::new();
            file.read_to_string(&mut s)?;
            Ok(s)
        } else {
            Err(From::from(ErrorKind::Other("No file?".into())))
        }
    } else {
        if let Some(url) = &cfg.file {
            let data = fetch_data(url)?;
            Ok(data)
        } else {
            Err(From::from(ErrorKind::Other("No URL?".into())))
        }
    }
}

/// Parse the csv, deserializing it with `serde` based on the `Product` struct
pub fn parse_csv(cfg: &Config, data: String) -> io::Result<HashMap<usize, Product>> {
    let mut map = HashMap::new();

    let mut rdr = csv::ReaderBuilder::new()
        // FIXME this can panic if an empty string is provided as delimiter
        .delimiter(cfg.delimiter.clone().into_bytes()[0])
        .from_reader(data.as_bytes());

    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic deserialization. And by
        // needing to do that, we seem to have no idiomatic way of skipping bogus lines :(
        // Hence we print out the error here for now and return a dummy Product that is used to
        // continue the loop
        let record: Product = result.unwrap_or_else(|e| {
            eprintln!("{}", e);
            error_product()
        });
        if record.id == 0 {
            continue;
        };
        map.insert(record.id, record);
    }
    Ok(map)
}

/// Fetch csv data from an external service and return it as a `String`
pub fn fetch_data(url: &str) -> Result<String, Error> {
    println!("Fetching data from {}", &url);
    let body = reqwest::get(url)?.text()?;
    Ok(body)
}
