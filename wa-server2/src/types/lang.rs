use super::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    C11,
    C89,
    C99,
    Cpp11,
    Cpp14,
    Cpp17,
    // Rust,
    // Java,
    // Python3,
    // JavaScript,
    // TypeScript,
}

pub trait LanguageBroker {
    fn filename(&self) -> (&'static str, Option<&'static str>);
    fn compile<'a>(&self, working_dir: &'a Path, ce_filename: &'static str) -> Target<'a>;
    fn run_case<'a>(&self, task: &'a CaseTask) -> Target<'a>;
}

