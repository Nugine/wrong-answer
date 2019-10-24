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
        Target::vm(task, "python3".into(), src.into())
    }
}
