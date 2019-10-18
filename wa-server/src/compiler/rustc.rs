use crate::types::*;

/// rustc $SOURCE -o $BIN -O --edition 2018
pub struct Rustc<S: SandBox> {
    sandbox: S,
}

impl<S: SandBox> Compiler for Rustc<S> {
    fn compile(&self, task: CompileTask, limit: Option<Limit>) -> WaResult<TargetStatus> {
        let args: Vec<&str> = vec![
            task.source_path,
            "-o",
            task.binary_path,
            "-O",
            "--edition",
            "2018",
        ];

        let target = Target {
            working_dir: task.working_dir,
            bin: "rustc",
            args: &args,
            stdin: None,
            stdout: None,
            stderr: Some(task.ce_message_path),
        };

        self.sandbox.run(target, limit)
    }
}

#[test]
fn test_rustc() {
    use crate::sandbox::BareMonitorSandBox;
    let compiler = Rustc {
        sandbox: BareMonitorSandBox,
    };

    const HELLO_PATH: &str = "../assets/hello-rustc.rs";

    let task = CompileTask {
        working_dir: Path::new("."),
        source_path: HELLO_PATH,
        binary_path: "../temp/hello-rustc",
        ce_message_path: "../temp/ce-rustc.txt",
    };

    let ret = compiler.compile(task, Limit::no_effect());

    assert_eq!(ret.unwrap().code, Some(0));
}
