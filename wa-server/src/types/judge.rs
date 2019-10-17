use super::unit::*;

pub enum Language {
    C11,
    C89,
    C99,
    Cpp11,
    Cpp14,
    Cpp17,
    Java,
    Python3,
    JavaScript,
    TypeScript,
    Rust,
}

pub struct Problem {
    pub time_limit: Second,
    pub memory_limit: KiloByte,
    pub case_num: u32,
}

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
    pub time: MicroSecond,
    pub memory: KiloByte,
    pub status: JudgeStatus,
}

pub struct JudgeResult {
    pub time: MicroSecond,
    pub memory: KiloByte,
    pub status: JudgeStatus,
    pub ce_message: String, // compile error message
    pub cases: Vec<JudgeCaseResult>,
}

pub struct Submission {
    pub id: u64,
    pub problem_id: u64,
    pub source_code: String,
    pub language: Language,
    pub result: Option<JudgeResult>,
}

pub struct Update {
    pub submission_id: u64,
    pub status: JudgeStatus,
    pub result: Option<JudgeResult>,
}

impl Update {
    pub fn from_status(id: u64, status: JudgeStatus) -> Self {
        Self {
            submission_id: id,
            status,
            result: None,
        }
    }
}
