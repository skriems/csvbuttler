use std::env;
use std::path::Path;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "csvbuttler", about = "serves data from csv files")]
struct Cli {
    /// interface
    #[structopt(name = "interface", short, long)]
    pub interface: Option<String>,

    /// port
    #[structopt(name = "port", short, long)]
    pub port: Option<String>,

    /// csv file (local or URL)
    #[structopt(name = "file", short, long)]
    pub file: Option<String>,

    /// delimiter
    #[structopt(name = "delimiter", short, long)]
    pub delimiter: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Default {
    pub alloworigin: String,
    pub interface: String,
    pub port: u16,
    pub domain: String,
    pub https: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Csv {
    pub uri: String,
    pub delimiter: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Secrets {
    pub app: String,
    pub csrf: String,
    pub jwt: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub default: Default,
    pub csv: Csv,
    pub secrets: Secrets,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("config/default"))?;

        // Add in the current environment file
        // Default to 'dev' env
        // Note that this file is _optional_
        let env = env::var("APP_ENV").unwrap_or("dev".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        s.merge(File::with_name("config/local").required(false))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("app").separator("_"))?;

        // Add in settings from the CLI
        // s.set("database.url", "postgres://")?;
        let cli = Cli::from_args();

        if cli.interface.is_some() {
            s.set("default.interface", cli.interface.unwrap())?;
        }

        if cli.port.is_some() {
            s.set("default.port", cli.port.unwrap())?;
        }

        if cli.file.is_some() {
            s.set("csv.file", cli.file.unwrap())?;
        }

        if cli.delimiter.is_some() {
            s.set("csv.delimiter", cli.delimiter.unwrap())?;
        }

        // populate the `APP_SECRETS_JWT` env var so we can get in the `decode_token` fn
        let jwt_secret = s.get_str("secrets.jwt")?;
        env::set_var("APP_SECRETS_JWT", jwt_secret);

        // deserialize and freeze the settings
        s.try_into()
    }

    pub fn is_local(&self) -> bool {
        Path::new(&self.csv.uri).exists()
    }
}
