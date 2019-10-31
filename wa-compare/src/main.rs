use std::path::PathBuf;
use structopt::StructOpt;
use wa_compare::{compare_ascii, compare_fast, compare_utf8, Comparison};

#[derive(Debug, Clone, Copy, PartialEq, Eq, StructOpt)]
enum Mode {
    Utf8,
    Ascii,
    Fast,
}

impl std::str::FromStr for Mode {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Mode, &'static str> {
        match s {
            "utf8" => Ok(Mode::Utf8),
            "ascii" => Ok(Mode::Ascii),
            "fast" => Ok(Mode::Fast),
            _ => Err("<invalid mode>"),
        }
    }
}

#[derive(Debug, Clone, StructOpt)]
struct Opt {
    stdout: PathBuf,
    userout: PathBuf,

    #[structopt(short = "p", long)]
    /// Do not check PE, invalid in fast mode
    permissive: bool,

    #[structopt(short = "m", long)]
    /// "utf8"|"ascii"|"fast"
    mode: Mode,
}

fn main() {
    env_logger::init();

    let opt = Opt::from_args();

    let ret: std::io::Result<Comparison> = match opt.mode {
        Mode::Utf8 => compare_utf8(opt.permissive, &opt.stdout, &opt.userout),
        Mode::Ascii => compare_ascii(opt.permissive, &opt.stdout, &opt.userout),
        Mode::Fast => compare_fast(&opt.stdout, &opt.userout),
    };

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
