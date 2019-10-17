use crate::redis::RedisBroker;
use crate::types::*;
use crossbeam_channel::Sender;

pub struct Listener {
    pub redis: RedisBroker,
    pub submission_sender: Sender<Submission>,
    pub update_sender: Sender<Update>,
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

impl Listener {
    pub fn listen(self) -> impl Fn() + Send + Sync + 'static {
        move || loop {
            let submission = handle!(self.redis.get_submission(), "redis: {}");

            handle!(
                self.update_sender
                    .send(Update::queuing(submission.submission_id)),
                "updater disconnected: {}"
            );

            handle!(
                self.submission_sender.send(submission),
                "worker disconnected: {}"
            );
        }
    }
}
