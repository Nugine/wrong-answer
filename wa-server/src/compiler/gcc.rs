use crate::types::*;

// gcc $SOURCE -o $BIN -O2 -static -std=$STD
// g++ $SOURCE -o $BIN -O2 -static -std=$STD
pub struct Gcc<S: SandBox> {
    sandbox: S,
    is_cpp: bool,
    std: &'static str,
}

impl<S: SandBox> Compiler for Gcc<S> {
    fn compile(&self, task: CompileTask, limit: Option<Limit>) -> WaResult<TargetStatus> {
        let bin = if self.is_cpp { "g++" } else { "gcc" };
        let std = &format!("-std={}", self.std);
        let args = vec![
            task.source_path,
            "-o",
            task.binary_path,
            "-O2",
            "-static",
            &std,
        ];

        let target = Target {
            working_dir: task.working_dir,
            bin,
            args: &args,
            stdin: None,
            stdout: None,
            stderr: Some(task.ce_message_path),
        };

        self.sandbox.run(target, limit)
    }
}

#[test]
fn test_gcc() {
    use crate::sandbox::BareMonitorSandBox;
    let compiler = Gcc {
        sandbox: BareMonitorSandBox,
        is_cpp: false,
        std: "c99",
    };

    const HELLO_PATH: &str = "../assets/hello-gcc.c";

    let task = CompileTask {
        working_dir: Path::new("."),
        source_path: HELLO_PATH,
        binary_path: "../temp/hello-gcc",
        ce_message_path: "../temp/ce-gcc.txt",
    };

    let ret = compiler.compile(task, Limit::no_effect());

    assert_eq!(ret.unwrap().code, Some(0));
}

#[test]
fn test_gpp() {
    use crate::sandbox::BareMonitorSandBox;
    let compiler = Gcc {
        sandbox: BareMonitorSandBox,
        is_cpp: true,
        std: "c++14",
    };

    const HELLO_PATH: &str = "../assets/hello-g++.cpp";

    let task = CompileTask {
        working_dir: Path::new("."),
        source_path: HELLO_PATH,
        binary_path: "../temp/hello-g++",
        ce_message_path: "../temp/ce-g++.txt",
    };

    let ret = compiler.compile(task, Limit::no_effect());

    assert_eq!(ret.unwrap().code, Some(0));
}
