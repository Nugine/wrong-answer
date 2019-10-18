use super::unit::*;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
pub enum Language {
    C11,
    C89,
    C99,
    Cpp11,
    Cpp14,
    Cpp17,
    // Rust,
    // Java,
    // Python3,
    // JavaScript,
    // TypeScript,
}

pub enum JudgeType {
    Strict,
    IgnoreTrialingSpace,
    SpecialJudge,
    Interactive,
}

pub struct Submission {
    pub id: u64,
    pub problem_id: u64,
    pub judge_type: JudgeType,
    pub time_limit: Second,
    pub memory_limit: MegaByte,
    pub case_num: u32,
    pub source_code: String,
    pub lang: Language,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JudgeStatus {
    Pending,
    Queuing,
    Compiling,
    Judging,
    AC,  // Accepted
    WA,  // Wrong Answer
    RE,  // Runtime Error
    SC,  // Similar Code
    CLE, // Compile Limit Exceeded
    CE,  // Compile Error
    PE,  // Presentation Error
    TLE, // Time Limit Exceeded
    MLE, // Memory Limit Exceeded
    OLE, // Output Limit Exceeded
    SE,  // System Error
}

pub struct JudgeCaseResult {
    pub time: MilliSecond,
    pub memory: KiloByte,
    pub status: JudgeStatus,
}

pub struct JudgeResult {
    pub time: MilliSecond,
    pub memory: KiloByte,
    pub compile_message: Option<String>, // compile error message
    pub cases: Vec<JudgeCaseResult>,
}

pub struct Update {
    pub submission_id: u64,
    pub status: JudgeStatus,
    pub result: Option<JudgeResult>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparision {
    AC,
    WA,
    PE,
}

impl Submission {
    pub fn update(&self, status: JudgeStatus) -> Update {
        Update {
            submission_id: self.id,
            status,
            result: None,
        }
    }

    pub fn final_update(&self, status: JudgeStatus, result: JudgeResult) -> Update {
        Update {
            submission_id: self.id,
            status,
            result: Some(result),
        }
    }
}

impl JudgeResult {
    pub fn zero() -> Self {
        Self {
            time: 0,
            memory: 0,
            compile_message: None,
            cases: vec![],
        }
    }

    pub fn from_ce(msg: String) -> Self {
        Self {
            time: 0,
            memory: 0,
            compile_message: Some(msg),
            cases: vec![],
        }
    }
}

impl Comparision {
    pub fn to_status(self) -> JudgeStatus {
        match self {
            Comparision::AC => JudgeStatus::AC,
            Comparision::PE => JudgeStatus::PE,
            Comparision::WA => JudgeStatus::WA,
        }
    }
}
