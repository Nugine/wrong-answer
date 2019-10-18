use crate::types::*;

pub struct RedisBroker;

impl RedisBroker {
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
