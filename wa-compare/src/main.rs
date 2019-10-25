use std::path::PathBuf;
use structopt::StructOpt;
use wa_compare::{compare_ascii, compare_utf8};

#[derive(Debug, Clone, StructOpt)]
struct Opt {
    stdout: PathBuf,
    userout: PathBuf,

    #[structopt(short = "p", long)]
    /// Do not check PE
    permissive: bool,

    #[structopt(short = "a", long)]
    /// Ascii mode, defaults UTF-8 mode
    ascii: bool,
}

fn main() {
    env_logger::init();

    let opt = Opt::from_args();

    let compare = if opt.ascii {
        compare_ascii
    } else {
        compare_utf8
    };

    let ret = compare(opt.permissive, &opt.stdout, &opt.userout);
    match ret {
        Ok(comp) => println!("{:?}", comp),

        Err(e) => {
            log::error!("compare error: {}", e);
            match e.raw_os_error() {
                Some(errno) => std::process::exit(errno),
                None => panic!(e),
            }
        }
    };
}
