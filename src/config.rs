use std::env;
use std::path::Path;

use crate::error::{Error, ErrorKind};

use dotenv;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "csvbuttler", about = "serves data from csv files")]
pub struct Config {
    /// csv file (local or URL)
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

    /// Access-Control-Allow-Origin Header
    #[structopt(name = "allow-origin", short = "-o", long)]
    pub allow_origin: Option<String>,

    /// Basic Auth username for fetching the CSV
    #[structopt(name = "csv-username", short = "-u", long)]
    pub csv_username: Option<String>,

    /// Basic Auth password for fetching the CSV
    #[structopt(name = "csv-password", short = "-w", long)]
    pub csv_password: Option<String>,
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

/// Utility function which populates environment variables via `dotenv` and merges them into the
/// `Config` struct
pub fn get_config() -> Result<Config, Error> {
    dotenv::dotenv().ok();
    let mut cfg = Config::from_args();

    if cfg.file.is_none() {
        // we need either a file or a URL so fail here, if we got neither
        let uri = env::var("CSV_URL")?;
        cfg.file = Some(uri);
    }

    // If no allowed origin is specified via cli, we try to get it from env.
    // Otherwise, set it to None - no need to fail here
    if cfg.allow_origin.is_none() {
        let allowed_origin = match env::var("CORS_ALLOW_ORIGIN") {
            Ok(origin) => Some(origin),
            Err(_) => None,
        };
        cfg.allow_origin = allowed_origin;
    }

    if cfg.csv_username.is_none() {
        let csv_username = match env::var("CSV_USERNAME") {
            Ok(username) => Some(username),
            Err(_) => None,
        };
        cfg.csv_username = csv_username;
    }

    if cfg.csv_password.is_none() {
        let csv_password = match env::var("CSV_PASSWORD") {
            Ok(password) => Some(password),
            Err(_) => None,
        };
        cfg.csv_password = csv_password;
    }

    // TODO can we move that validation to structopt somehow?
    if cfg.csv_username.is_some() && cfg.csv_password.is_none() {
        return Err(From::from(ErrorKind::Other(
            "Need password for Basic Auth".into(),
        )));
    }
    Ok(cfg)
}
