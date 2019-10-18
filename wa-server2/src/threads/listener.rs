use super::*;
use crate::handle;
use crate::redis::RedisBroker;
use crate::types::*;

pub struct Listener {
    pub redis: RedisBroker,
    pub submission_sender: Sender<Submission>,
    pub update_sender: Sender<Update>,
}

impl Listener {
    pub fn listen(self) -> impl Fn() + Send + Sync + 'static {
        move || {
            handle!(self.redis.reload(), "redis error: {}");

            loop {
                let submission = handle!(self.redis.get_submission(), "redis error: {}");

                handle!(
                    self.update_sender
                        .send(submission.update(JudgeStatus::Queuing)),
                    "updater is disconnected: {}"
                );

                handle!(
                    self.submission_sender.send(submission),
                    "workers are disconnected: {}"
                );
            }
        }
    }
}
