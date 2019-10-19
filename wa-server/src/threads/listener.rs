use super::*;
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
            log::info!("reloading");
            handle!(self.redis.reload(), "redis error: {}");

            log::info!("start listening");
            loop {
                log::info!("waiting submission");
                let submission = handle!(self.redis.get_submission(), "redis error: {}");

                log::info!("submission id = {}", submission.id);

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
