use crate::into_vec;
use crate::types::*;

pub struct Rustc;

impl LanguageBroker for Rustc {
    fn filename(&self) -> (&'static str, Option<&'static str>) {
        ("src.rs", Some("src"))
    }

    /// `rustc src.rs -o src -O --edition 2018`
    fn compile<'a>(&self, working_dir: &'a Path, ce_filename: &'a str) -> Option<Target<'a>> {
        let rustc = "rustc".into();
        let (src, bin) = self.filename();

        let args = into_vec![src, "-o", bin.unwrap(), "-O", "--edition", "2018",];

        Some(Target {
            working_dir,
            bin: rustc,
            args,
            stdin: None,
            stdout: None,
            stderr: Some(Path::new(ce_filename)),
        })
    }

    fn run_case<'a>(&self, task: &'a CaseTask) -> Target<'a> {
        assert!(task.act_path.is_none()); // FIXME:
        Target::direct(task)
    }
}
