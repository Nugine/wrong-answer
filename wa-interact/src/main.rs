use std::fs::File;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use structopt::StructOpt;
use wa_monitor::types::MonitorErrorKind;

#[derive(Debug, Clone, StructOpt)]
struct PipeOpt {
    #[structopt(long, value_name = "path")]
    uapipe: PathBuf,

    #[structopt(long, value_name = "path")]
    aupipe: PathBuf,
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
struct BinOpt {
    bin: String,

    args: Vec<String>,
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(flatten)]
    bin_opt: BinOpt,

    #[structopt(flatten)]
    act_opt: ActOpt,

    #[structopt(flatten)]
    pipe_opt: PipeOpt,
}

fn main() {
    env_logger::init();

    let Opt {
        act_opt,
        bin_opt,
        pipe_opt,
    } = Opt::from_args();
    let pipe_opt1 = pipe_opt.clone();

    let act_handle = std::thread::spawn(move || -> Result<(File, File), MonitorErrorKind> {
        let ua_rx = fifo_open(&pipe_opt1.uapipe)?;
        let au_tx = fifo_create(&pipe_opt1.aupipe)?;
        Ok((ua_rx, au_tx))
    });

    let bin_handle = std::thread::spawn(move || -> Result<(File, File), MonitorErrorKind> {
        let ua_tx = fifo_create(&pipe_opt.uapipe)?;
        let au_rx = fifo_open(&pipe_opt.aupipe)?;
        Ok((au_rx, ua_tx))
    });

    let (ua_rx, au_tx) = match act_handle.join().unwrap() {
        Ok(f) => f,
        Err(e) => std::process::exit(e as i32),
    };

    let (au_rx, ua_tx) = match bin_handle.join().unwrap() {
        Ok(f) => f,
        Err(e) => std::process::exit(e as i32),
    };

    if let Err(e) = Command::new(act_opt.actpath)
        .arg(act_opt.actin)
        .arg(act_opt.actout)
        .stdin(ua_rx)
        .stdout(au_tx)
        .stderr(Stdio::null())
        .spawn()
    {
        log::error!("can not spawn act: {}", e);
        std::process::exit(MonitorErrorKind::ForkError as i32)
    }

    let e = Command::new(bin_opt.bin)
        .args(bin_opt.args)
        .stdin(au_rx)
        .stdout(ua_tx)
        .stderr(Stdio::null())
        .exec();
    log::error!("can not exec bin: {}", e);
    std::process::exit(MonitorErrorKind::ForkError as i32)
}

fn fifo_open(path: &Path) -> Result<File, MonitorErrorKind> {
    File::open(path).map_err(|e| {
        log::error!("fifo error: {}", e);
        MonitorErrorKind::FifoError
    })
}

fn fifo_create(path: &Path) -> Result<File, MonitorErrorKind> {
    File::create(path).map_err(|e| {
        log::error!("fifo error: {}", e);
        MonitorErrorKind::FifoError
    })
}
