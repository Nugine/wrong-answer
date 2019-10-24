use crate::into_vec;
use crate::types::*;

pub struct TsNode;

impl LanguageBroker for TsNode {
    fn filename(&self) -> (&'static str, Option<&'static str>) {
        ("src.ts", Some("src.js"))
    }

    /// tsc src.ts --outFile src.js --strict -t ESNEXT
    fn compile<'a>(&self, working_dir: &'a Path, ce_filename: &'a str) -> Option<Target<'a>> {
        let (src, bin) = self.filename();

        let args = into_vec![src, "--outFile", bin.unwrap(), "--strict", "-t", "ESNEXT",];

        Some(Target {
            working_dir,
            bin: "tsc".into(),
            args,
            stdin: None,
            stdout: Some(Path::new(ce_filename)),
            stderr: None,
        })
    }

    fn run_case<'a>(&self, task: &'a CaseTask) -> Target<'a> {
        assert!(task.act_path.is_none()); // FIXME:
        let (_, bin) = self.filename();
        Target::vm(task, "node".into(), bin.unwrap().into())
    }
}
