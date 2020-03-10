use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

use crate::error::Error;
use crate::model::{error_product, Product};
use crate::settings::Settings;

use reqwest;

/// type alias for `AppState`
pub type StateType = Arc<Mutex<AppState>>;

#[derive(Debug)]
/// The `AppState` is constructed with the app configuration and the generated `HashMap` from the
/// csv data.
pub struct AppState {
    pub settings: Settings,
    pub map: HashMap<usize, Product>,
}

/// `AppState` implements a `new` function for convenience.
/// Note that this should only be done once. Subsequent updates to the `map` should be done via
/// mutable references to the `Mutex` inside of the `Arc`.
impl AppState {
    pub fn new() -> Result<StateType, Error> {
        let settings = Settings::new()?;
        println!("{:?}", &settings);
        let csv = get_csv(&settings)?;
        let map = parse_csv(&settings, csv)?;

        Ok(Arc::new(Mutex::new(AppState { settings, map })))
    }
}

/// Retrieve the csv either from a local file, or try to fetch it, from an external service
fn get_csv(settings: &Settings) -> Result<String, Error> {
    if settings.is_local() {
        let mut file = File::open(&settings.csv.uri)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        Ok(s)
    } else {
        let data = fetch_data(&settings)?;
        Ok(data)
    }
}

/// Parse the csv, deserializing it with `serde` based on the `Product` struct
pub fn parse_csv(settings: &Settings, data: String) -> io::Result<HashMap<usize, Product>> {
    let mut map = HashMap::new();

    let mut rdr = csv::ReaderBuilder::new()
        // FIXME this can panic if an empty string is provided as delimiter
        .delimiter(settings.csv.delimiter.clone().into_bytes()[0])
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
pub fn fetch_data(settings: &Settings) -> Result<String, Error> {
    println!("Fetching data from {}", settings.csv.uri);
    let client = match &settings.csv.username {
        Some(username) => {
            if let Some(password) = &settings.csv.password {
                reqwest::Client::new()
                    .get(&settings.csv.uri)
                    .basic_auth(username, Some(password))
            } else {
                return Err(Error::Other("Need password for Basic Auth".into()));
            }
        }
        None => reqwest::Client::new().get(&settings.csv.uri),
    };
    let resp = client.send()?.text()?;
    Ok(resp)
}
