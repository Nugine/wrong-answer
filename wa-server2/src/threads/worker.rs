use super::*;
use crate::lang::*;
use crate::types::*;
use crate::GLOBAL_CONFIG;

pub struct Worker {
    pub submission_receiver: Receiver<Submission>,
    pub update_sender: Sender<Update>,
    pub workspace: PathBuf,
}

impl Worker {
    pub fn work(self) -> impl Fn() + Send + 'static {
        move || loop {
            let submission = handle!(
                self.submission_receiver.recv(),
                "submission sender is disconnected: {}"
            );

            let submission_id = submission.id;
            let working_dir = self.workspace.join(submission_id.to_string());

            handle!(@custom
                std::fs::create_dir_all(&working_dir),
                "can not create dir: {:?}",
                &working_dir,
            );

            if let Err(e) = self.handle_submission(submission, &working_dir) {
                log::error!("system error: {}", e);
                self.send_update(Update {
                    submission_id,
                    status: JudgeStatus::SE,
                    result: Some(JudgeResult::zero()),
                });
                // NOTE: SE
            }

            handle!(@custom
                std::fs::remove_dir_all(&working_dir),
                "can not remove dir: {:?}",
                &working_dir,
            );
        }
    }

    fn handle_submission(&self, submission: Submission, working_dir: &Path) -> WaResult<()> {
        let broker = submission.lang.get_broker();

        // save source code
        let src_path: PathBuf = broker.save_source(&submission.source_code, working_dir)?;

        // Queuing -> Compiling
        self.try_send_update(submission.update(JudgeStatus::Compiling))?;

        // compile
        let compile_message_path = working_dir.join("ce.txt");
        let task: CompileTask = CompileTask {
            working_dir,
            src_path: &src_path,
            compile_message_path: &compile_message_path,
            lang: submission.lang,
        };
        let bin_path = match broker.compile(task)? {
            CompileResult::None => None,
            CompileResult::CLE => {
                let result = JudgeResult::zero();
                let update = submission.final_update(JudgeStatus::CLE, result);
                return self.try_send_update(update); // NOTE: CLE
            }
            CompileResult::CE(msg) => {
                let result = JudgeResult::from_ce(msg);
                let update = submission.final_update(JudgeStatus::CE, result);
                return self.try_send_update(update); // NOTE: CE
            }
            CompileResult::Success(bin) => Some(bin),
        };

        // Compiling -> Judging
        self.try_send_update(submission.update(JudgeStatus::Judging))?;

        let data_dir: &Path = &GLOBAL_CONFIG.data_dir;
        let mut case_task = CaseTask {
            working_dir,
            submission: &submission,
            src_path,
            bin_path,
            case_index: 0,
            stdin_path: data_dir.to_owned(),
            stdout_path: data_dir.to_owned(),
            userout_path: working_dir.join("userout.out"),
            act_path: match submission.judge_type {
                JudgeType::Interactive => Some(working_dir.join("act")),
                _ => None,
            },
            spj_path: match submission.judge_type {
                JudgeType::SpecialJudge => Some(working_dir.join("spj")),
                _ => None,
            },
        };

        let mut status = JudgeStatus::AC;
        let mut result = JudgeResult::zero();
        let cases =
            (1..=submission.case_num).map(|i| (i, format!("{}.in", i), format!("{}.out", i)));
        for (i, input, output) in cases {
            case_task.case_index = i + 1;
            case_task.stdin_path.push(input);
            case_task.stdout_path.push(output);

            let res = match broker.run_case(&case_task) {
                Err(err) => {
                    log::error!(
                        "system error: {}, submission_id = {}, problem_id = {}, case = {}",
                        err,
                        submission.id,
                        submission.problem_id,
                        case_task.case_index,
                    );
                    return Err(err);
                }
                Ok(res) => res,
            };

            if res.status != JudgeStatus::AC {
                status = res.status;
            }

            result.time += res.time;
            result.memory += res.memory;
            result.cases.push(res);

            case_task.stdin_path.pop();
            case_task.stdout_path.pop();
        }

        self.try_send_update(submission.final_update(status, result))
    }

    fn send_update(&self, update: Update) {
        handle!(
            self.update_sender.send(update),
            "updater is disconnected: {}"
        );
    }

    fn try_send_update(&self, update: Update) -> WaResult<()> {
        self.update_sender
            .send(update)
            .map_err(|_| WaError::Channel("updater is disconnected"))
    }
}

fn cases(case_num: u32) -> impl Iterator<Item = (String, String)> {
    (1..=case_num).map(|i| (format!("{}.in", i), format!("{}.out", i)))
}
