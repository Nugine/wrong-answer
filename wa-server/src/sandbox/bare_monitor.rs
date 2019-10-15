use crate::types::{Limit, MonitorErrorKind, SandBox, Target, TargetStatus, WaError, WaResult};
use num_traits::FromPrimitive;
use std::process::Command;
use std::process::Stdio;

pub struct BareMonitorSandBox;

const MONITOR_PATH: &str = "wa-monitor";
const NULL_DEVICE: &str = "/dev/null";

impl SandBox for BareMonitorSandBox {
    /// limit: no effect
    fn run(&self, target: Target, _limit: Limit) -> WaResult<TargetStatus> {
        let mut command = Command::new(MONITOR_PATH);
        command.current_dir(target.working_dir);

        command.arg("-i").arg(target.stdin.unwrap_or(NULL_DEVICE));
        command.arg("-o").arg(target.stdout.unwrap_or(NULL_DEVICE));
        command.arg("-e").arg(target.stderr.unwrap_or(NULL_DEVICE));

        command
            .arg("--")
            .arg(target.bin)
            .args(target.args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit());

        let child = command.spawn()?;

        let output = child.wait_with_output()?;
        let code = output.status.code();
        match code {
            Some(0) => {}
            Some(_) | None => {
                let kind = code
                    .and_then(MonitorErrorKind::from_i32)
                    .unwrap_or(MonitorErrorKind::Unknown);
                let err: WaError = kind.into();
                return Err(err);
            }
        };

        let status: TargetStatus = serde_json::from_slice(&output.stdout)
            .map_err(|_| -> WaError { MonitorErrorKind::Unknown.into() })?;
        Ok(status)
    }
}

#[test]
fn test_bare_monitor() {
    let sandbox = BareMonitorSandBox;

    let ret = sandbox.run(
        Target {
            working_dir: "./",
            bin: "ls",
            args: &vec![],
            stdin: None,
            stdout: None,
            stderr: None,
        },
        Limit {
            time: u64::max_value(),
            memory: u64::max_value(),
            output: u64::max_value(),
            security_cfg_path: "",
        },
    );

    assert_eq!(ret.unwrap().code, Some(0));

    let ret = sandbox.run(
        Target {
            working_dir: "./",
            bin: "qwertyuiop",
            args: &vec![],
            stdin: None,
            stdout: None,
            stderr: None,
        },
        Limit {
            time: u64::max_value(),
            memory: u64::max_value(),
            output: u64::max_value(),
            security_cfg_path: "",
        },
    );

    assert_eq!(
        ret.unwrap().code,
        Some(MonitorErrorKind::ExecvpError as i32)
    );
}
