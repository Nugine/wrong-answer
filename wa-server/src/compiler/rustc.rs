use crate::types::*;
use std::ffi::CString;

/// rustc $SOURCE -o $BIN -O --edition 2018
pub struct Rustc<S: SandBox> {
    sandbox: S,
}

impl<S: SandBox> Compiler for Rustc<S> {
    fn compile(&self, working_dir: &str, task: CompileTask, limit: &Limit) -> WaResult<()> {
        let binary_path = match task.binary_path {
            Some(p) => p,
            None => {
                const MSG: &str = "compiler: expected binary path, found None";
                log::error!("{}", MSG);
                return Err(WaError::Internal(MSG.into()));
            }
        };

        let bin = CString::new("rustc").unwrap();
        let args: Vec<CString> = [
            task.source_path,
            "-o",
            binary_path,
            "-O",
            "--edition",
            "2018",
        ]
        .iter()
        .map(|&s| CString::new(s).unwrap())
        .collect();

        let stderr = task.ce_message_path.map(|s| CString::new(s).unwrap());

        let target = Target {
            bin,
            args,
            stdin: None,
            stdout: None,
            stderr,
        };

        self.sandbox.run(working_dir, &target, limit).map(|_| ())
    }
}

#[test]
fn test_rustc() {
    use crate::sandbox::BareMonitorSandBox;
    let compiler = Rustc {
        sandbox: BareMonitorSandBox,
    };

    const HELLO_PATH: &str = "../assets/hello.rs";

    let task = CompileTask {
        source_path: HELLO_PATH,
        binary_path: Some("../temp/hello"),
        ce_message_path: Some("../temp/ce.txt"),
    };

    let ret = compiler.compile(
        "./",
        task,
        &Limit {
            time: u64::max_value(),
            memory: u64::max_value(),
            output: u64::max_value(),
            security_cfg_path: "".into(),
        },
    );

    assert!(ret.is_ok());
}
