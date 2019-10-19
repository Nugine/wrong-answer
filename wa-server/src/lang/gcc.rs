use crate::types::*;

pub struct Gcc {
    pub is_cpp: bool,
    pub std: &'static str,
}

impl LanguageBroker for Gcc {
    fn filename(&self) -> (&'static str, Option<&'static str>) {
        (if self.is_cpp { "src.cpp" } else { "src.c" }, Some("src"))
    }

    /// `gcc   src.c -o src -O2 -static -std=$STD`
    /// `g++ src.cpp -o src -O2 -static -std=$STD`
    fn compile<'a>(
        &self,
        working_dir: &'a Path,
        ce_filename: &'static str,
    ) -> Target<'a> {
        let gcc = if self.is_cpp { "g++" } else { "gcc" };
        let (_, bin) = self.filename();
        let std = format!("-std={}", self.std);

        let args = vec![
            bin.unwrap().into(),
            "-o".into(),
            "src".into(),
            "-O2".into(),
            "-static".into(),
            std,
        ];

        Target {
            working_dir,
            bin: gcc,
            args,
            stdin: None,
            stdout: None,
            stderr: Some(Path::new(ce_filename)),
        }
    }

    fn run_case<'a>(&self, task: &'a CaseTask) -> Target<'a> {
        assert!(task.act_path.is_none()); // FIXME:
        Target::direct(task)
    }
}