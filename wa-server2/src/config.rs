use crate::types::*;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize)]
pub struct CompileLimit {
    pub time: Second,
    pub memory: MegaByte,
    pub output: MegaByte,
    pub security_cfg_path: Option<PathBuf>,
}

#[derive(Deserialize)]
pub struct Config {
    pub redis_url: String,
    pub output_hard_limit: MegaByte,
    pub memory_hard_limit: MegaByte,
    pub data_dir: PathBuf,
    pub compile_limit: HashMap<Language, CompileLimit>,
}

const CONFIG_ENV_KEY: &str = "WA_CONFIG_PATH";

fn load_config() -> Config {
    let config_path = std::env::var(CONFIG_ENV_KEY).expect("invalid config path");
    let config_file = File::open(&config_path).expect("can not open config file");
    serde_json::from_reader(BufReader::new(config_file)).expect("invalid config")
}

fn validate_config(config: &Config) {
    assert!(config.data_dir.is_dir());
}

lazy_static! {
    pub static ref GLOBAL_CONFIG: Config = {
        let config = load_config();
        validate_config(&config);
        config
    };
}
