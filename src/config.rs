use std::env;
use std::path::Path;

use crate::error::Error;

use dotenv;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "csvbuttler", about = "serves data from csv files")]
pub struct Config {

    /// csv file
    #[structopt(name = "file", short, long)]
    pub file: Option<String>,

    /// delimiter
    #[structopt(short = "-d", long = "--delimiter", default_value = ",")]
    pub delimiter: String,

    /// interface
    #[structopt(name = "interface", short, long, default_value = "127.0.0.1")]
    pub interface: String,

    /// port
    #[structopt(name = "port", short, long, default_value = "8000")]
    pub port: String,
}

impl Config {
    pub fn is_local(&self) -> bool {
        if let Some(f) = &self.file {
            Path::new(f).exists()
        } else {
            false
        }
    }
}

pub fn get_config() -> Result<Config, Error> {
    dotenv::dotenv().ok();
    let mut cfg = Config::from_args();

    if let Some(uri) = cfg.file {
        cfg.file = Some(uri);
    } else {
        let uri = env::var("CSV_URL")?;
        cfg.file = Some(uri);
    }

    println!("Starting csvbuttler with {:?}", cfg);
    Ok(cfg)
}
