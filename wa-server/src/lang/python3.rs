use crate::types::*;

pub struct Python3;

impl LanguageBroker for Python3 {
    fn filename(&self) -> (&'static str, Option<&'static str>) {
        ("src.py", None)
    }

    fn compile<'a>(&self, _: &'a Path, _: &'a str) -> Option<Target<'a>> {
        None
    }

    fn run_case<'a>(&self, task: &'a CaseTask) -> Target<'a> {
        assert!(task.act_path.is_none()); // FIXME:
        let (src, _) = self.filename();
        Target {
            working_dir: task.working_dir,
            bin: "python3".into(),
            args: vec![src.into()],
            stdin: Some(&task.stdin_path),
            stdout: Some(&task.userout_path),
            stderr: None,
        }
    }
}
