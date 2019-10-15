use crate::types::{Limit, MonitorErrorKind, SandBox, Target, TargetStatus, WaError, WaResult};
use num_traits::FromPrimitive;
use std::process::Command;
use std::process::Stdio;

pub struct BareMonitorSandBox;

const MONITOR_PATH: &str = "wa-monitor";
const NULL_DEVICE: &str = "/dev/null";

impl SandBox for BareMonitorSandBox {
    /// limit: no effect
    fn run(&self, working_dir: &str, target: &Target, _limit: &Limit) -> WaResult<TargetStatus> {
        let mut child_builder = Command::new(MONITOR_PATH);
        child_builder.current_dir(working_dir);

        // target is from utf8 data, so unwrap here
        fn transform(s: &Option<std::ffi::CString>) -> &str {
            s.as_ref()
                .map(|s| s.to_str().unwrap())
                .unwrap_or(NULL_DEVICE)
        }

        child_builder.arg("-i").arg(transform(&target.stdin));
        child_builder.arg("-o").arg(transform(&target.stdout));
        child_builder.arg("-e").arg(transform(&target.stderr));

        let args = target.args.iter().map(|s| s.to_str().unwrap());

        let child = child_builder
            .arg(target.bin.to_str().unwrap())
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

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
    use std::ffi::CString;

    let sandbox = BareMonitorSandBox;

    let ret = sandbox.run(
        "./",
        &Target {
            bin: CString::new("ls").unwrap(),
            args: vec![],
            stdin: None,
            stdout: None,
            stderr: None,
        },
        &Limit {
            time: u64::max_value(),
            memory: u64::max_value(),
            output: u64::max_value(),
            security_cfg_path: "".into(),
        },
    );

    dbg!(ret.unwrap());

    let ret = sandbox.run(
        "./",
        &Target {
            bin: CString::new("qwertyuiop").unwrap(),
            args: vec![],
            stdin: None,
            stdout: None,
            stderr: None,
        },
        &Limit {
            time: u64::max_value(),
            memory: u64::max_value(),
            output: u64::max_value(),
            security_cfg_path: "".into(),
        },
    );

    dbg!(ret.unwrap());
}