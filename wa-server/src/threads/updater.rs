use crate::redis::RedisBroker;
use crate::types::*;
use crossbeam_channel::Receiver;

pub struct Updater {
    pub redis: RedisBroker,
    pub update_receiver: Receiver<Update>,
}

macro_rules! handle {
    ($ret:expr,$fmt:expr) => {{
        match $ret {
            Err(e) => {
                log::error!($fmt, e);
                panic!(e)
            }
            Ok(r) => r,
        }
    }};
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
