use crate::types::*;

// javac -J-Xms64m -J-Xmx512m -encoding UTF-8 -sourcepath $SOURCE -d $BIN Main.java
pub struct Javac<S: SandBox> {
    sandbox: S,
}

impl<S: SandBox> Compiler for Javac<S> {
    fn compile(&self, task: CompileTask, limit: Option<Limit>) -> WaResult<TargetStatus> {
        let bin = "javac";
        let java_path = &format!("{}/Main.java", task.source_path);
        let args = vec![
            "-J-Xms64m",
            "-J-Xmx512m",
            "-encoding",
            "UTF-8",
            "-sourcepath",
            task.source_path,
            "-d",
            task.binary_path,
            &java_path,
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
fn test_javac() {
    use crate::sandbox::BareMonitorSandBox;
    let compiler = Javac {
        sandbox: BareMonitorSandBox,
    };

    const HELLO_PATH: &str = "../assets/hello-javac";

    let task = CompileTask {
        working_dir: Path::new("."),
        source_path: HELLO_PATH,
        binary_path: "../temp/hello-javac",
        ce_message_path: "../temp/ce-javac.txt",
    };

    let ret = compiler.compile(task, Limit::no_effect());

    assert_eq!(ret.unwrap().code, Some(0));
}
