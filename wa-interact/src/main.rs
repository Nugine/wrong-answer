use std::fs::File;
use std::process::{Child, Command, Stdio};
use structopt::StructOpt;
use wa_monitor::types::MonitorErrorKind;

#[derive(Debug, Clone, StructOpt)]
struct PipeOpt {
    #[structopt(long, value_name = "path")]
    uapipe: String,

    #[structopt(long, value_name = "path")]
    aupipe: String,
}

#[derive(Debug, StructOpt)]
struct ActOpt {
    #[structopt(long, value_name = "path")]
    actin: String,

    #[structopt(long, value_name = "path")]
    actout: String,

    #[structopt(long, value_name = "path")]
    actpath: String,
}
#[derive(Debug, StructOpt)]
struct MonitorOpt {
    bin: String,

    args: Vec<String>,
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(flatten)]
    monitor_opt: MonitorOpt,

    #[structopt(flatten)]
    act_opt: ActOpt,

    #[structopt(flatten)]
    pipe_opt: PipeOpt,
}

fn main() {
    env_logger::init();

    let opt = Opt::from_args();

    let act_opt = opt.act_opt;
    let monitor_opt = opt.monitor_opt;
    let pipe_opt1 = opt.pipe_opt.clone();
    let pipe_opt2 = opt.pipe_opt;

    let act_handle = std::thread::spawn(move || spawn_act(act_opt, pipe_opt1));

    let monitor_handle = std::thread::spawn(move || spawn_monitor(monitor_opt, pipe_opt2));

    let mut act = match act_handle.join() {
        Ok(Ok(child)) => child,
        Ok(Err(e)) => {
            log::error!("spawn act error: {}", e);
            std::process::exit(e as i32)
        }
        Err(_) => {
            log::error!("can not spawn act thread");
            std::process::exit(MonitorErrorKind::ThreadError as i32)
        }
    };

    let mut monitor = match monitor_handle.join() {
        Ok(Ok(child)) => child,
        Ok(Err(e)) => {
            log::error!("spawn monitor error: {}", e);
            std::process::exit(e as i32)
        }
        Err(_) => {
            log::error!("can not spawn monitor thread");
            std::process::exit(MonitorErrorKind::ThreadError as i32)
        }
    };

    let _ = monitor.wait();
    let _ = act.wait();
}

fn spawn_act(opt: ActOpt, pipe: PipeOpt) -> Result<Child, MonitorErrorKind> {
    // monitor opens aupipe firstly
    // open aupipe firstly here to avoid deadlock

    let stdout = match File::create(&pipe.aupipe) {
        Ok(file) => file,
        Err(e) => {
            log::error!("fifo error: {}", e);
            return Err(MonitorErrorKind::FifoError);
        }
    };

    let stdin = match File::open(&pipe.uapipe) {
        Ok(file) => file,
        Err(e) => {
            log::error!("fifo error: {}", e);
            return Err(MonitorErrorKind::FifoError);
        }
    };

    Command::new(opt.actpath)
        .arg(opt.actin)
        .arg(opt.actout)
        .stdin(stdin)
        .stdout(stdout)
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| {
            log::error!("can not spawn act: {}", e);
            MonitorErrorKind::ForkError
        })
}

fn spawn_monitor(opt: MonitorOpt, pipe: PipeOpt) -> Result<Child, MonitorErrorKind> {
    Command::new("wa-monitor")
        .arg("-i")
        .arg(pipe.aupipe)
        .arg("-o")
        .arg(pipe.uapipe)
        .arg("-e")
        .arg("/dev/null")
        .arg(opt.bin)
        .args(opt.args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| {
            log::error!("can not spawn monitor: {}", e);
            MonitorErrorKind::ForkError
        })
}
