use crate::types::*;

pub struct Nodejs;

impl LanguageBroker for Nodejs {
    fn filename(&self) -> (&'static str, Option<&'static str>) {
        ("src.js", None)
    }

    fn compile<'a>(&self, _: &'a Path, _: &'a str) -> Option<Target<'a>> {
        None
    }

    fn run_case<'a>(&self, task: &'a CaseTask) -> Target<'a> {
        assert!(task.act_path.is_none()); // FIXME:
        let (src, _) = self.filename();
        Target {
            working_dir: task.working_dir,
            bin: "node".into(),
            args: vec![src.into()],
            stdin: Some(&task.stdin_path),
            stdout: Some(&task.userout_path),
            stderr: None,
        }
    }
}
