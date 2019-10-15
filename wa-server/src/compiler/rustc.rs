use crate::types::*;

/// rustc $SOURCE -o $BIN -O --edition 2018
pub struct Rustc<S: SandBox> {
    sandbox: S,
}

impl<S: SandBox> Compiler for Rustc<S> {
    fn compile(&self, task: CompileTask, limit: Limit) -> WaResult<()> {
        let binary_path = match task.binary_path {
            Some(p) => p,
            None => {
                const MSG: &str = "compiler: expected binary path, found None";
                log::error!("{}", MSG);
                return Err(WaError::Internal(MSG.into()));
            }
        };

        let args: Vec<&str> = vec![
            task.source_path,
            "-o",
            binary_path,
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
            stderr: task.ce_message_path,
        };

        self.sandbox.run(target, limit).map(|_| ())
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
        working_dir: "./",
        source_path: HELLO_PATH,
        binary_path: Some("../temp/hello-rustc"),
        ce_message_path: Some("../temp/ce.txt"),
    };

    let ret = compiler.compile(
        task,
        Limit {
            time: u64::max_value(),
            memory: u64::max_value(),
            output: u64::max_value(),
            security_cfg_path: "",
        },
    );

    assert!(ret.is_ok());
}
