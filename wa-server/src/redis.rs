use crate::types::{Submission, WaResult};

pub type RedisPool = r2d2::Pool<r2d2_redis::RedisConnectionManager>;

#[derive(Clone)]
pub struct RedisBroker {
    pool: RedisPool,
}

impl RedisBroker {
    pub fn new(pool: RedisPool) -> Self {
        Self { pool }
    }
}

impl RedisBroker {
    // TODO:
    pub fn get_submission(&self) -> WaResult<Submission> {
        unimplemented!()
    }
}
