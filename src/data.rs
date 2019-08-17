use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex};

use crate::config;
use crate::error;
use crate::model::{error_product, Product};

/// type alias for `AppState`
type StateType = Arc<Mutex<AppState>>;

#[derive(Debug)]
pub struct AppState {
    pub cfg: config::Config,
    pub map: HashMap<usize, Product>,
}

impl AppState {
    pub fn new() -> Result<StateType, error::Error> {
        let cfg = config::get_config()?;
        let map = read_data()?;

        Ok(Arc::new(Mutex::new(AppState { cfg, map })))
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
