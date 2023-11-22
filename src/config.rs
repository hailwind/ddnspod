use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug, Clone)]
pub struct Sub {
    pub subdomain: String,
    pub record_id: u64,
    pub record_type: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub interface: String,
    pub secret_id: String,
    pub secret_key: String,
    pub domain: String,
    pub subs: Vec<Sub>,
}

pub fn read_config<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let c = serde_json::from_reader(reader)?;
    Ok(c)
}
