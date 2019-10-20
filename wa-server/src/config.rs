use crate::types::*;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub key_prefix: String,
    pub submission_queue_key: String,
    pub temp_queue_key: String,
    pub submission_key_prefix: String,
    pub submission_status_key_prefix: String,
    pub judge_result_queue_key: String,
    pub data_time_map_key: String,
    pub data_key_prefix: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub redis: RedisConfig,

    pub output_hard_limit: MegaByte,
    pub memory_hard_limit: MegaByte,

    pub data_dir: PathBuf,
    pub workspace: PathBuf,

    pub compile_limit: Option<Limit>,

    pub worker_num: u32,
}

const CONFIG_ENV_KEY: &str = "WA_CONFIG_PATH";

fn load_config() -> Config {
    let config_path = std::env::var(CONFIG_ENV_KEY).expect("invalid config path");
    let config_file = File::open(&config_path).expect("can not open config file");
    serde_json::from_reader(BufReader::new(config_file)).expect("invalid config")
}

fn validate_config(config: &Config) {
    if !config.data_dir.exists(){
        std::fs::create_dir_all(&config.data_dir).expect("can not create data dir");
    }
    if !config.workspace.exists(){
        std::fs::create_dir_all(&config.workspace).expect("can not create workspace");
    }
}

lazy_static! {
    pub static ref GLOBAL_CONFIG: Config = {
        let config = load_config();
        validate_config(&config);
        config
    };
}
