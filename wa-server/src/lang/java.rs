use crate::types::*;

pub struct Java;

impl LanguageBroker for Java {
    fn filename(&self) -> (&'static str, Option<&'static str>) {
        ("Main.java", Some("Main"))
    }

    /// `javac -encoding UTF-8 -sourcepath . -d . Main.java`
    fn compile<'a>(&self, working_dir: &'a Path, ce_filename: &'a str) -> Option<Target<'a>> {
        let (src, _) = self.filename();

        let args = vec![
            "-encoding".into(),
            "UTF-8".into(),
            "-sourcepath".into(),
            ".".into(),
            "-d".into(),
            ".".into(),
            src.into(),
        ];

        Some(Target {
            working_dir,
            bin: "javac".into(),
            args,
            stdin: None,
            stdout: None,
            stderr: Some(Path::new(ce_filename)),
        })
    }

    fn run_case<'a>(&self, task: &'a CaseTask) -> Target<'a> {
        assert!(task.act_path.is_none()); // FIXME:
        Target {
            working_dir: task.working_dir,
            bin: "java".into(),
            args: vec!["Main".into()],
            stdin: Some(&task.stdin_path),
            stdout: Some(&task.userout_path),
            stderr: None,
        }
    }
}
