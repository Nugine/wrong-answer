use crate::types::JudgeCaseResult;
use crate::types::JudgeStatus;
use crate::types::Limit;
use crate::types::TargetStatus;
use crate::types::{KiloByte, MilliSecond};

pub fn parse_status(
    status: &TargetStatus,
    limit: &Limit,
) -> (MilliSecond, KiloByte, Option<JudgeCaseResult>) {
    let cpu_time = status.user_time + status.sys_time;
    let memory = status.memory;

    let gen = |status| {
        (
            cpu_time,
            memory,
            Some(JudgeCaseResult {
                time: cpu_time,
                memory,
                status,
            }),
        )
    };

    if cpu_time > limit.time {
        return gen(JudgeStatus::TLE);
    }

    if memory > limit.memory {
        return gen(JudgeStatus::MLE);
    }

    // NOTE: ENOTTY 25
    if status.signal == Some(25) {
        return gen(JudgeStatus::OLE);
    }

    if status.code != Some(0) {
        return gen(JudgeStatus::RE);
    }

    (cpu_time, memory, None)
}
