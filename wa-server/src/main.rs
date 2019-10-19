mod config;
mod lang;
mod redis;
mod sandbox;
mod threads;
mod types;
mod utils;

pub use config::GLOBAL_CONFIG;

#[macro_export]
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

use crate::redis::RedisBroker;
use crate::sandbox::BareMonitorSandBox;
use std::thread::{spawn, JoinHandle};
use threads::{Listener, Updater, Worker};

fn main() {
    env_logger::init();

    let redis = RedisBroker::new();

    let submission_channel = crossbeam_channel::bounded(0);
    let update_channel = crossbeam_channel::unbounded();

    let listener = Listener {
        redis: redis.clone(),
        submission_sender: submission_channel.0,
        update_sender: update_channel.0.clone(),
    };

    let updater = Updater {
        redis: redis.clone(),
        update_receiver: update_channel.1,
    };

    let workers: Vec<Worker<BareMonitorSandBox>> = vec![
        Worker {
            submission_receiver: submission_channel.1.clone(),
            update_sender: update_channel.0.clone(),
            workspace: GLOBAL_CONFIG.workspace.clone(),
            sandbox: BareMonitorSandBox
        };
        GLOBAL_CONFIG.worker_num as usize
    ];

    drop(submission_channel.1);
    drop(update_channel.0);
    drop(redis);

    let listener: JoinHandle<()> = spawn(listener.listen());
    let updater: JoinHandle<()> = spawn(updater.update());
    let workers: Vec<JoinHandle<()>> = workers
        .into_iter()
        .map(|worker| spawn(worker.work()))
        .collect();

    let _ = listener.join();
    let _ = updater.join();
    for worker in workers {
        let _ = worker.join();
    }
}
