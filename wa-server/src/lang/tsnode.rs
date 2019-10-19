use crate::types::*;

pub struct TsNode;

impl LanguageBroker for TsNode {
    fn filename(&self) -> (&'static str, Option<&'static str>) {
        ("src.ts", Some("src.js"))
    }

    /// tsc src.ts --outFile src.js --strict -t ESNEXT
    fn compile<'a>(&self, working_dir: &'a Path, ce_filename: &'a str) -> Option<Target<'a>> {
        let (src, bin) = self.filename();

        let args = vec![
            src.into(),
            "--outFile".into(),
            bin.unwrap().into(),
            "--strict".into(),
            "-t".into(),
            "ESNEXT".into(),
        ];

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
        Target {
            working_dir: task.working_dir,
            bin: "node".into(),
            args: vec![bin.unwrap().into()],
            stdin: Some(&task.stdin_path),
            stdout: Some(&task.userout_path),
            stderr: None,
        }
    }
}
