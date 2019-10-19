use crate::config::RedisConfig;
use crate::types::*;
use crate::GLOBAL_CONFIG;
use r2d2_redis::RedisConnectionManager;
use redis::Commands;
use redis::PipelineCommands;

#[derive(Debug, Clone)]
pub struct RedisBroker {
    pool: r2d2::Pool<RedisConnectionManager>,
    submission_queue_key: String,
    temp_queue_key: String,
    judge_result_queue_key: String,
    data_time_map_key: String,
}

impl RedisBroker {
    pub fn new() -> Self {
        let redis: &RedisConfig = &GLOBAL_CONFIG.redis;

        let manager =
            RedisConnectionManager::new(redis.url.as_str()).expect("can not connect to redis");
        let pool = r2d2::Pool::builder()
            .max_size(4)
            .build(manager)
            .expect("fail to create redis pool");
        let submission_queue_key = format!("{}-{}", redis.key_prefix, redis.submission_queue_key);

        let temp_queue_key = format!("{}-{}", redis.key_prefix, redis.temp_queue_key);

        let judge_result_queue_key =
            format!("{}-{}", redis.key_prefix, redis.judge_result_queue_key);

        let data_time_map_key = format!("{}-{}", redis.key_prefix, redis.data_time_map_key);

        Self {
            pool,
            submission_queue_key,
            temp_queue_key,
            judge_result_queue_key,
            data_time_map_key,
        }
    }

    pub fn reload(&self) -> WaResult<()> {
        let conn: &mut redis::Connection = &mut *self.pool.get()?;
        loop {
            let id: Option<u64> =
                conn.rpoplpush(&self.temp_queue_key, &self.submission_queue_key)?;
            if id.is_none() {
                break;
            }
        }
        Ok(())
    }

    pub fn get_submission(&self) -> WaResult<Submission> {
        let conn: &mut redis::Connection = &mut *self.pool.get()?;
        let id: u64 = conn.brpoplpush(&self.submission_queue_key, &self.temp_queue_key, 0)?;
        let key = format!(
            "{}-{}-{}",
            GLOBAL_CONFIG.redis.key_prefix, GLOBAL_CONFIG.redis.submission_key_prefix, id
        );
        let value: String = conn.get(key)?;
        Ok(serde_json::from_str(&value).unwrap())
    }

    pub fn update_submission(&self, update: Update) -> WaResult<()> {
        let conn: &mut redis::Connection = &mut *self.pool.get()?;
        let redis: &RedisConfig = &GLOBAL_CONFIG.redis;

        let id = update.submission_id;

        let value = serde_json::to_string(&update).unwrap();
        let key = format!(
            "{}-{}-{}",
            redis.key_prefix, redis.submission_status_key_prefix, id,
        );
        conn.set(key, value)?;

        if update.is_final() {
            redis::pipe()
                .atomic()
                .lrem(&self.temp_queue_key, 0, id)
                .lpush(&self.judge_result_queue_key, id)
                .query(conn)?;
        }
        Ok(())
    }

    pub fn get_data_timestamp(&self, problem_id: u64) -> WaResult<u64> {
        let conn: &mut redis::Connection = &mut *self.pool.get()?;
        let timestamp: u64 = conn.hget(&self.data_time_map_key, problem_id)?;
        Ok(timestamp)
    }

    pub fn get_problem_data(&self, problem_id: u64) -> WaResult<HashMap<String, String>> {
        let conn: &mut redis::Connection = &mut *self.pool.get()?;
        let redis: &RedisConfig = &GLOBAL_CONFIG.redis;

        let key = format!(
            "{}-{}-{}",
            redis.key_prefix, redis.data_key_prefix, problem_id,
        );

        let map: HashMap<String, String> = conn.hgetall(key)?;
        Ok(map)
    }
}
