use lazy_static::lazy_static;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize)]

pub struct Config {
    pub redis_url: String,
}

const CONFIG_ENV_KEY: &str = "WA_CONFIG_PATH";

fn load_config() -> Config {
    let config_path = std::env::var(CONFIG_ENV_KEY).expect("invalid config path");
    let config_file = File::open(&config_path).expect("can not open config file");
    serde_json::from_reader(BufReader::new(config_file)).expect("invalid config")
}

lazy_static! {
    pub static ref GLOBAL_CONFIG: Config = { load_config() };
}
