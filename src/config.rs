use std::env;
use std::error;
use std::path::PathBuf;

use dotenv;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "csvbuttler", about = "serves data from csv files")]
pub struct Config {
    /// fetch CSV from URL
    #[structopt(long = "--url")]
    pub url: Option<String>,
    /// use a local csv file
    #[structopt(short = "-f", long = "--from-file", parse(from_os_str))]
    pub file: Option<PathBuf>,
    /// delimiter
    #[structopt(short = "-d", long = "--delimiter")]
    pub delimiter: Option<String>,
}

pub fn get_config() -> Result<Config, Box<dyn error::Error>> {
    dotenv::dotenv().ok();
    let mut cfg = Config::from_args();

    let url = match cfg.url {
        Some(url) => url,
        None => env::var("CSV_URL").expect("No csv data to serve"),
    };

    cfg.url = Some(url.to_owned());
    Ok(cfg)
}
