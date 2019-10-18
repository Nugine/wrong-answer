use super::helper::parse_status;
use super::*;
use crate::sandbox::SandBox;
use crate::types::*;
use crate::utils::compare;
use crate::GLOBAL_CONFIG;

pub struct Gcc<S: SandBox> {
    sandbox: S,
    std: &'static str,
    is_cpp: bool,
}

impl<S> LanguageBroker for Gcc<S>
where
    S: SandBox,
{
    fn save_source(&self, source_code: &str, working_dir: &Path) -> WaResult<PathBuf> {
        let src_path = working_dir.join(if self.is_cpp { "src.cpp" } else { "src.c" });
        std::fs::write(&src_path, source_code)?;
        Ok(src_path)
    }

    /// `gcc   src.c -o src -O2 -static -std=$STD`
    /// `g++ src.cpp -o src -O2 -static -std=$STD`
    fn compile(&self, task: CompileTask) -> WaResult<CompileResult> {
        let bin = if self.is_cpp { "g++" } else { "gcc" };
        let std = format!("-std={}", self.std);
        let args = vec![
            if self.is_cpp { "src.cpp" } else { "src.c" },
            "-o",
            "src",
            "-O2",
            "-static",
            &std,
        ];

        let target = Target {
            working_dir: task.working_dir,
            bin,
            args,
            stdin: None,
            stdout: None,
            stderr: Some(task.compile_message_path),
        };

        let limit = task.lang.get_limit();
        let status = self.sandbox.run(target, limit.as_ref())?;
        let ret = match status.code {
            Some(0) => CompileResult::Success(task.working_dir.join("src")),
            Some(_) => CompileResult::CE(std::fs::read_to_string(task.compile_message_path)?),
            None => CompileResult::CLE,
        };
        Ok(ret)
    }

    fn run_case(&self, task: &CaseTask) -> WaResult<JudgeCaseResult> {
        assert!(task.act_path.is_none()); // FIXME:

        let target = Target::direct(task);
        let limit = Limit::from_submission(&task.submission);

        let status = self.sandbox.run(target, Some(&limit))?;
        let (time, memory, ret) = parse_status(&status, &limit);

        if let Some(ret) = ret {
            return Ok(ret);
        }

        let status = if task.spj_path.is_some() {
            let target = Target::spj(task);
            let code = self.sandbox.run(target, Some(&limit))?.code;
            match code {
                Some(0) => JudgeStatus::AC,
                Some(1) => JudgeStatus::WA,
                Some(2) => JudgeStatus::PE,
                _ => {
                    log::error!("special judge error: code = {:?}, signal = {:?}, submission_id = {}, case = {}",
                        status.code,
                        status.signal,
                        task.submission.id,
                        task.case_index,
                    );
                    JudgeStatus::WA
                }
            }
        } else {
            let ignore_trailing_space = match task.submission.judge_type {
                JudgeType::Strict => false,
                JudgeType::IgnoreTrialingSpace => true,
                _ => unreachable!(),
            };
            let cmp = compare(ignore_trailing_space, &task.stdout_path, &task.userout_path)?;
            cmp.to_status()
        };

        Ok(JudgeCaseResult {
            time,
            memory,
            status,
        })
    }
}
