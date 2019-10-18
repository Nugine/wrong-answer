use super::unit::*;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
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
    pub id: u64,
    pub time_limit: Second,
    pub memory_limit: MegaByte,
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
    pub time: MilliSecond,
    pub memory: KiloByte,
    pub status: JudgeStatus,
}

pub struct JudgeResult {
    pub time: MilliSecond,
    pub memory: KiloByte,
    pub ce_message: String, // compile error message
    pub cases: Vec<JudgeCaseResult>,
}

pub struct Submission {
    pub id: u64,
    pub problem: Problem,
    pub source_code: String,
    pub language: Language,
    pub status: JudgeStatus,
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

impl JudgeResult {
    pub fn from_ce(ce_message: String) -> Self {
        Self {
            time: 0,
            memory: 0,
            ce_message,
            cases: vec![],
        }
    }
}
