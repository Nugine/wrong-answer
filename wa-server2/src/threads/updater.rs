use super::*;
use crate::redis::RedisBroker;
use crate::types::*;

pub struct Updater {
    pub redis: RedisBroker,
    pub update_receiver: Receiver<Update>,
}

impl Updater {
    pub fn update(self) -> impl Fn() + Send + Sync + 'static {
        move || loop {
            let update = handle!(
                self.update_receiver.recv(),
                "update senders are disconnected: {}"
            );

            handle!(self.redis.update_submission(update), "redis error: {}");
        }
    }
}
