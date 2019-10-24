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
        Target::vm(task, "node".into(), src.into())
    }
}
