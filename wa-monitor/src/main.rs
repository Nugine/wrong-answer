use std::ffi::{CString, NulError};
use structopt::StructOpt;
use wa_monitor::Target;

fn parse_c_string(s: &str) -> Result<CString, NulError> {
    CString::new(s)
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(try_from_str = parse_c_string))]
    bin: CString,

    #[structopt(parse(try_from_str = parse_c_string))]
    args: Vec<CString>,

    #[structopt(
        short = "i",
        long,
        value_name = "path",
        parse(try_from_str = parse_c_string)
    )]
    stdin: Option<CString>,

    #[structopt(
        short = "o",
        long,
        value_name = "path",
        parse(try_from_str = parse_c_string)
    )]
    stdout: Option<CString>,

    #[structopt(
        short = "e",
        long,
        value_name = "path",
        parse(try_from_str = parse_c_string)
    )]
    stderr: Option<CString>,
}

fn build_target(opt: Opt) -> Target {
    Target {
        bin: opt.bin,
        args: opt.args,
        stdin: opt.stdin,
        stdout: opt.stdout,
        stderr: opt.stderr,
    }
}

fn main() {
    env_logger::init();

    let opt = Opt::from_args();
    let target = build_target(opt);
    let status = target.run();
    let output = serde_json::to_string(&status).unwrap();
    log::info!("{}", output);
    println!("{}", output);
}
