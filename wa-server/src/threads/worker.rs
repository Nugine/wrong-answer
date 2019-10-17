use crate::types::*;
use crossbeam_channel::{Receiver, Sender};
use std::path::{Path, PathBuf};

pub struct Worker {
    pub submission_receiver: Receiver<Submission>,
    pub update_sender: Sender<Update>,
    pub working_dir: PathBuf,
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
    (@custom $ret:expr,$fmt:expr,$($other:expr,)*) => {{
        match $ret {
            Err(e) => {
                log::error!($fmt, $($other)*);
                panic!(e)
            }
            Ok(r) => r,
        }
    }};
}

impl Worker {
    pub fn work(self) -> impl Fn() + Send + Sync + 'static {
        move || loop {
            let submission = handle!(
                self.submission_receiver.recv(),
                "submission sender is disconnected: {}"
            );

            let id = submission.id;

            let submission_dir = self.working_dir.join(id.to_string());
            handle!(@custom
                std::fs::create_dir(&submission_dir),
                "can not create dir: {:?}",
                &submission_dir,
            );

            if let Err(e) = self.handle_submission(submission, &submission_dir) {
                log::error!("system error: {}", e);
                self.send_update(Update::from_status(id, JudgeStatus::SE))
            }

            handle!(@custom
                std::fs::remove_dir_all(&submission_dir),
                "can not remove dir: {:?}",
                &submission_dir,
            );
        }
    }
}

impl Worker {
    fn send_update(&self, update: Update) {
        handle!(
            self.update_sender.send(update),
            "updater is disconnected: {}"
        );
    }

    // TODO:
    fn handle_submission(&self, submission: Submission, dir: &Path) -> WaResult<()> {
        unimplemented!()
    }
}
