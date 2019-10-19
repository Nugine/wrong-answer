use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JudgeType {
    Strict,
    IgnoreTrialingSpace,
    SpecialJudge,
    Interactive,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct JudgeCaseResult {
    pub time: MilliSecond,
    pub memory: KiloByte,
    pub status: JudgeStatus,
}

#[derive(Serialize, Deserialize)]
pub struct JudgeResult {
    pub time: MilliSecond,
    pub memory: KiloByte,
    pub compile_message: Option<String>, // compile error message
    pub cases: Vec<JudgeCaseResult>,
}

#[derive(Serialize, Deserialize)]
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

pub struct CaseTask<'a> {
    pub working_dir: &'a Path,
    pub submission: &'a Submission,
    pub src_filename: &'a str,
    pub bin_filename: Option<&'a str>,
    pub case_index: u32,
    pub stdin_path: PathBuf,
    pub stdout_path: PathBuf,
    pub userout_path: PathBuf,
    pub act_path: Option<PathBuf>,
    pub spj_path: Option<PathBuf>,
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

impl Update {
    pub fn is_final(&self) -> bool {
        self.result.is_some()
    }
}

#[test]
fn print_mock_submission() {
    let sm = Submission {
        id: 47,
        problem_id: 1001,
        judge_type: JudgeType::Strict,
        time_limit: 1,
        memory_limit: 32,
        case_num: 1,
        source_code: include_str!("../../../assets/hello-javac/Main.java").into(),
        lang: Language::Java,
    };
    let value = serde_json::to_string(&sm).unwrap();
    println!("{:?}", value);
}
