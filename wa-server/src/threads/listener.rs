use super::*;
use crate::redis::RedisBroker;
use crate::types::*;
use crate::GLOBAL_CONFIG;

pub struct Listener {
    pub redis: RedisBroker,
    pub submission_sender: Sender<Submission>,
    pub update_sender: Sender<Update>,
    pub data_lock: Arc<DataLock>,
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
                    self.sync_data(submission.problem_id),
                    "fail to sync data: {}"
                );

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

    pub fn sync_data(&self, problem_id: u64) -> WaResult<()> {
        let remote_timestamp = self.redis.get_data_timestamp(problem_id)?;

        let guard = self.data_lock.write().expect("data lock error");

        let mut data_path: PathBuf = GLOBAL_CONFIG.data_dir.join(problem_id.to_string());
        const TIMESTAMP_FILENAME: &str = "timestamp";

        let mut local = None;
        let needs_sync = if data_path.exists() {
            data_path.push(TIMESTAMP_FILENAME);

            let needs = if data_path.exists() {
                let timestamp = std::fs::read_to_string(&data_path)?;
                let local_timestamp: u64 = timestamp.trim().parse().expect("invalid timestamp");
                local = Some(local_timestamp);
                local_timestamp < remote_timestamp
            } else {
                true
            };

            data_path.pop();
            needs
        } else {
            std::fs::create_dir_all(&data_path)?;
            true
        };

        if !needs_sync {
            return Ok(());
        }

        log::info!(
            "sync data: problem id = {}, local = {:?}, remote = {}",
            problem_id,
            local,
            remote_timestamp
        );

        let data = self.redis.get_problem_data(problem_id)?;

        let mut file_path = data_path;
        for (filename, text) in data {
            file_path.push(filename);
            log::info!("write file: {:?}", file_path);
            std::fs::write(&file_path, text)?;
            file_path.pop();
        }

        file_path.push(TIMESTAMP_FILENAME);
        std::fs::write(&file_path, remote_timestamp.to_string())?;

        drop(guard);
        Ok(())
    }
}
