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
    assert!(config.data_dir.is_dir());
}

lazy_static! {
    pub static ref GLOBAL_CONFIG: Config = {
        let config = load_config();
        validate_config(&config);
        config
    };
}

pub const DATA_TIME_FILENAME: &str = "timestamp";

pub fn load_data_time() -> WaResult<HashMap<u64, u64>> {
    let entries = std::fs::read_dir(&GLOBAL_CONFIG.data_dir)?;

    let mut map = HashMap::new();

    for entry in entries {
        let entry = entry?;
        let mut path = entry.path();
        let problem_id: u64 = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .parse()
            .expect("invalid problem id");
        path.push(DATA_TIME_FILENAME);
        let timestamp = std::fs::read_to_string(&path).map_err(|e| {
            log::error!("not found: {:?}", path);
            e
        })?;
        let timestamp = timestamp.trim().parse().expect("invalid timestamp");
        map.insert(problem_id, timestamp);
    }

    Ok(map)
}
