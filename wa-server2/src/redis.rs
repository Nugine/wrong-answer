use crate::types::*;

#[derive(Debug,Clone)]
pub struct RedisBroker;

impl RedisBroker {
    pub fn new(redis_url: &str)->WaResult<Self>{
        unimplemented!()
    }

    pub fn reload(&self) -> WaResult<()> {
        unimplemented!()
    }

    pub fn get_submission(&self) -> WaResult<Submission> {
        unimplemented!()
    }

    pub fn update_submission(&self, update: Update) -> WaResult<()> {
        unimplemented!()
    }
}
